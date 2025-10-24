import Component from "../Component";
import { type Address, decodeAddress, encodeAddress } from "../utils";
import TileMap from "./TileMap";
import TileRegistry from "./TileRegistry";

class LightMap extends Component {
	private lighting: Map<Address, number> = new Map();
	private readonly chunkSize = 32; // Tiles per chunk

	private tileMapRef: TileMap | null = null;
	private tileRegistryRef: TileRegistry | null = null;

	private dirtyChunks: Set<Address> = new Set();
	public markDirty(chunkX: number, chunkY: number): void {
		const chunkAddr = encodeAddress(chunkX, chunkY);
		this.dirtyChunks.add(chunkAddr);
	}

	public updateDirtyChunks(): void {
		if (this.dirtyChunks.size === 0) return;
		for (const addr of this.dirtyChunks) {
			const decodedAddress = decodeAddress(addr);
			this.bakeChunkLighting(decodedAddress.x, decodedAddress.y);
		}
		this.dirtyChunks.clear();
	}

	public setup(): void {
		this.tileMapRef = this.entity.getComponent(TileMap);
		this.tileRegistryRef = this.entity.getComponent(TileRegistry);
	}

	public getLighting(x: number, y: number): number {
		const addr = encodeAddress(x, y);
		return this.lighting.get(addr) ?? 0;
	}

	public setLighting(x: number, y: number, value: number): void {
		const addr = encodeAddress(x, y);
		this.lighting.set(addr, value);
	}

	public onKeyUp(event: KeyboardEvent): void {
		if (event.key === "l") {
			console.log(this.lightSourceDependencies);
		}
	}

	private lightSourceDependencies: Map<Address, Set<Address>> = new Map();
	// Reverse mapping: tile -> set of light sources affecting it
	private tileLightSources: Map<Address, Set<Address>> = new Map();

	public bakeChunkLighting(chunkX: number, chunkY: number): void {
		if (!this.tileMapRef) return;

		let lightSourcesChanged = false;
		const affectedChunks = new Set<Address>();
		for (let x = chunkX; x < chunkX + this.chunkSize; x++) {
			for (let y = chunkY; y < chunkY + this.chunkSize; y++) {
				const tileId = this.tileMapRef.getTile(x, y);
				const tileEntry = this.tileRegistryRef?.getTileEntry(tileId);
				const address = encodeAddress(x, y);

				// Step 1: Clear existing light data
				this.lighting.delete(address);

				// Step 2: Detect changes in light source status
				// If a light is removed, queue affected chunks for recalculation
				// If a light is added or removed, mark that light sources changed
				const dependencies = this.lightSourceDependencies.get(address);
				const hadLightSource = !!dependencies;
				const hasLightSource =
					tileEntry?.lightIntensity !== undefined &&
					tileEntry.lightIntensity > 0;
				if (hadLightSource !== hasLightSource) {
					lightSourcesChanged = true;
					if (hadLightSource && dependencies.size > 0) {
						for (const affectedAddr of dependencies) {
							const { x: tileX, y: tileY } = decodeAddress(affectedAddr);
							const newChunkX =
								Math.floor(tileX / this.chunkSize) * this.chunkSize;
							const newChunkY =
								Math.floor(tileY / this.chunkSize) * this.chunkSize;
							affectedChunks.add(encodeAddress(newChunkX, newChunkY));
						}
					}
				}

				// Step 3: Clean up old forward mapping and its reverse dependencies
				const oldAffectedTiles = this.lightSourceDependencies.get(address);
				if (oldAffectedTiles) {
					// Remove this source from all tiles it used to affect
					for (const affectedAddr of oldAffectedTiles) {
						const sources = this.tileLightSources.get(affectedAddr);
						if (sources) {
							sources.delete(address);
							if (sources.size === 0) {
								this.tileLightSources.delete(affectedAddr);
							}
						}
					}
				}
				this.lightSourceDependencies.delete(address);

				// If the tile is not a light source, skip further processing
				if (!hasLightSource) continue;

				// Step 4: New light source - calculate affected tiles
				const affectedTiles = new Set<Address>();
				const lightRadius = tileEntry?.lightRadius ?? 0;
				for (let dx = -lightRadius; dx <= lightRadius; dx++) {
					for (let dy = -lightRadius; dy <= lightRadius; dy++) {
						const dist = dx * dx + dy * dy;
						if (dist > lightRadius * lightRadius) continue;

						const targetX = x + dx;
						const targetY = y + dy;
						const targetAddr = encodeAddress(targetX, targetY);

						// Forward mapping: source -> affected tiles
						affectedTiles.add(targetAddr);
						const newChunkX =
							Math.floor(targetX / this.chunkSize) * this.chunkSize;
						const newChunkY =
							Math.floor(targetY / this.chunkSize) * this.chunkSize;
						affectedChunks.add(encodeAddress(newChunkX, newChunkY));

						// Reverse mapping: affected tile -> sources
						if (!this.tileLightSources.has(targetAddr)) {
							this.tileLightSources.set(targetAddr, new Set());
						}
						this.tileLightSources.get(targetAddr)?.add(address);
					}
				}

				this.lightSourceDependencies.set(address, affectedTiles);
			}
		}

		// Step 5: Calculate final lighting values for tiles in the chunk
		for (let x = chunkX; x < chunkX + this.chunkSize; x++) {
			for (let y = chunkY; y < chunkY + this.chunkSize; y++) {
				const addr = encodeAddress(x, y);
				const sources = this.tileLightSources.get(addr);

				if (!sources || sources.size === 0) continue;

				let maxLightValue = 0;

				for (const sourceAddr of sources) {
					const sourcePos = decodeAddress(sourceAddr);
					const dx = x - sourcePos.x;
					const dy = y - sourcePos.y;
					const sourceTileId = this.tileMapRef.getTile(
						sourcePos.x,
						sourcePos.y,
					);
					const sourceTileEntry =
						this.tileRegistryRef?.getTileEntry(sourceTileId);
					const lightRadius = sourceTileEntry?.lightRadius ?? 0;
					const dist = Math.sqrt(dx * dx + dy * dy);

					const ratio = dist / lightRadius;

					const lightValue = Math.max(
						0,
						(sourceTileEntry?.lightIntensity ?? 0) -
							ratio * (sourceTileEntry?.lightIntensity ?? 0),
					);
					maxLightValue = Math.max(maxLightValue, lightValue);
				}

				this.lighting.set(addr, maxLightValue);
			}
		}

		// Step 6: Mark neighboring chunks dirty so they can recalculate their lighting
		// But only if light sources actually changed in this chunk. If not, skip all chunks queued for baking
		if (lightSourcesChanged) {
			const currentChunkAddr = encodeAddress(chunkX, chunkY);
			for (const chunkDep of affectedChunks) {
				// Skip the current chunk - we just baked it
				if (chunkDep === currentChunkAddr) continue;

				const { x, y } = decodeAddress(chunkDep);
				this.markDirty(x, y);
				this.tileMapRef?.markDirty(x, y);
			}
		}
	}
}

export default LightMap;
