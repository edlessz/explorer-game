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

	private propagateLighting(
		startX: number,
		startY: number,
		startValue: number,
		falloff = 0.1,
	): Set<Address> {
		// Use a queue for iterative breadth-first propagation
		const queue: Array<{ x: number; y: number; value: number }> = [
			{ x: startX, y: startY, value: startValue },
		];
		const affectedChunks: Set<Address> = new Set();

		while (queue.length > 0) {
			const current = queue.shift();
			if (!current) break;
			const { x, y, value } = current;

			// Set the lighting at this position
			const addr = encodeAddress(x, y);
			const currentValue = this.lighting.get(addr) ?? 0;

			const chunkX = Math.floor(x / this.chunkSize) * this.chunkSize;
			const chunkY = Math.floor(y / this.chunkSize) * this.chunkSize;
			const chunkAddr = encodeAddress(chunkX, chunkY);
			affectedChunks.add(chunkAddr);

			if (value <= currentValue) continue; // Skip if not brighter

			this.lighting.set(addr, value);

			// Calculate the new value for neighbors
			const newValue = Math.max(0, value - falloff);
			if (newValue <= 0) continue; // Stop propagating if light is too dim

			// Add neighbors to the queue
			const neighbors = [
				[x + 1, y],
				[x - 1, y],
				[x, y + 1],
				[x, y - 1],
			];

			for (const [nx, ny] of neighbors) {
				const nAddr = encodeAddress(nx, ny);
				const neighborValue = this.lighting.get(nAddr) ?? 0;

				// Only add to queue if this would make it brighter
				if (newValue > neighborValue) {
					queue.push({ x: nx, y: ny, value: newValue });
				}
			}
		}

		return affectedChunks;
	}

	public bakeChunkLighting(chunkX: number, chunkY: number): void {
		// Calculate the light travel distance (max brightness / falloff)
		const lightRadius = Math.ceil(1 / 0.1);

		// Step 2: Find all light sources that could affect this chunk
		// Search in a radius around the chunk to catch nearby light sources
		const searchRadius = lightRadius;
		for (let dx = -searchRadius; dx < this.chunkSize + searchRadius; dx++) {
			for (let dy = -searchRadius; dy < this.chunkSize + searchRadius; dy++) {
				const tileX = chunkX + dx;
				const tileY = chunkY + dy;

				const tileId = this.tileMapRef?.getTile(tileX, tileY);

				// tileId 4 is a light source with full brightness
				if (tileId === 4) {
					const affectedChunks = this.propagateLighting(tileX, tileY, 1);
					for (const addr of affectedChunks) {
						const decodedAddress = decodeAddress(addr);
						this.tileMapRef?.markDirty(decodedAddress.x, decodedAddress.y);
					}
				}
			}
		}
	}
}

export default LightMap;
