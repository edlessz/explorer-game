import Component from "../Component";
import { type Address, decodeAddress, encodeAddress } from "../utils";
import TileMap from "./TileMap";

class LightMap extends Component {
	private lighting: Map<Address, number> = new Map();
	private readonly chunkSize = 32; // Tiles per chunk

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
		console.log(`Baked ${this.dirtyChunks.size} light map chunks`);
		this.dirtyChunks.clear();
	}

	private tileMapRef: TileMap | null = null;

	public setup(): void {
		this.tileMapRef = this.entity.getComponent(TileMap);
	}

	public getLighting(x: number, y: number): number {
		const addr = encodeAddress(x, y);
		return this.lighting.get(addr) ?? 0;
	}

	public setLighting(x: number, y: number, value: number): void {
		const addr = encodeAddress(x, y);
		this.lighting.set(addr, value);
	}

	private lightSourceDependencies: Map<Address, Set<Address>> = new Map();
	// Reverse mapping: tile -> set of light sources affecting it
	private tileLightSources: Map<Address, Set<Address>> = new Map();

	public bakeChunkLighting(chunkX: number, chunkY: number): void {
		if (!this.tileMapRef) return;

		const lightRadius = 225;

		// Step 1: Clear existing lighting data for the chunk
		for (let x = chunkX; x < chunkX + this.chunkSize; x++) {
			for (let y = chunkY; y < chunkY + this.chunkSize; y++) {
				const addr = encodeAddress(x, y);
				this.lighting.delete(addr);

				const dependentTiles = this.lightSourceDependencies.get(addr);
				if (dependentTiles) {
					for (const sourceAddr of dependentTiles) {
						const { x: tileX, y: tileY } = decodeAddress(sourceAddr);
						const newChunkX =
							Math.floor(tileX / this.chunkSize) * this.chunkSize;
						const newChunkY =
							Math.floor(tileY / this.chunkSize) * this.chunkSize;
						this.markDirty(newChunkX, newChunkY);
					}
				}
			}
		}

		// Step 2: Recalculate lighting based on tiles in the chunk
		const affectedChunks = new Set<Address>();
		for (let x = chunkX; x < chunkX + this.chunkSize; x++) {
			for (let y = chunkY; y < chunkY + this.chunkSize; y++) {
				const tileId = this.tileMapRef.getTile(x, y);
				const address = encodeAddress(x, y);

				// Clean up old forward mapping and its reverse dependencies
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

				if (tileId !== 4) continue; // Assuming tile ID 4 is a light source

				const affectedTiles = new Set<Address>();
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

		// Step 3: Apply lighting contributions to tiles in the chunk
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
					// const dist = Math.sqrt(dx * dx + dy * dy);
					const dist = dx * dx + dy * dy;

					const lightValue = Math.max(0, 1 - dist / lightRadius);
					maxLightValue = Math.max(maxLightValue, lightValue);
				}

				this.lighting.set(addr, maxLightValue);
			}
		}

		for (const tileDep of affectedChunks) {
			const { x, y } = decodeAddress(tileDep);
			this.markDirty(x, y);
		}

		this.tileMapRef?.markDirty(chunkX, chunkY);
	}
}

export default LightMap;
