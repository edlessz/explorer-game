import Component from "../Component";
import { type Address, encodeAddress } from "../utils";
import LightMap from "./LightMap";

interface ChunkCache {
	canvas: HTMLCanvasElement;
	ctx: CanvasRenderingContext2D;
	isDirty: boolean;
}

class TileMap extends Component {
	private tiles: Map<Address, number> = new Map();
	public tileSet: Map<number, HTMLImageElement> = new Map();

	// Chunk cache settings
	private readonly chunkSize = 32; // Tiles per chunk
	private chunkCache: Map<Address, ChunkCache> = new Map();
	private cachePPU: { x: number; y: number } = { x: 32, y: 32 }; // PPU used for cached chunks

	private lightMapRef: LightMap | null = null;

	public setup(): void {
		this.lightMapRef = this.entity.getComponent(LightMap);
	}

	public getTile(x: number, y: number): number {
		const addr = encodeAddress(x, y);
		return this.tiles.get(addr) ?? 0;
	}
	public setTile(x: number, y: number, tileId: number): void {
		const addr = encodeAddress(
			x - this.entity.transform.position.x,
			y - this.entity.transform.position.y,
		);
		this.tiles.set(addr, tileId);

		// Mark the chunk containing this tile as dirty
		const chunkX = Math.floor(x / this.chunkSize) * this.chunkSize;
		const chunkY = Math.floor(y / this.chunkSize) * this.chunkSize;
		this.lightMapRef?.markDirty(chunkX, chunkY);

		const chunkAddr = encodeAddress(chunkX, chunkY);
		const chunk = this.chunkCache.get(chunkAddr);
		if (chunk) chunk.isDirty = true;
	}

	public markDirty(chunkX: number, chunkY: number): void {
		const chunkAddr = encodeAddress(chunkX, chunkY);
		const chunk = this.chunkCache.get(chunkAddr);
		if (chunk) chunk.isDirty = true;
	}

	private getOrCreateChunkCache(chunkX: number, chunkY: number): ChunkCache {
		const chunkAddr = encodeAddress(chunkX, chunkY);
		let chunk = this.chunkCache.get(chunkAddr);

		if (!chunk) {
			const canvas = document.createElement("canvas");

			// Size in pixels based on cachePPU (fixed at creation time)
			canvas.width = this.chunkSize * this.cachePPU.x;
			canvas.height = this.chunkSize * this.cachePPU.y;

			const ctx = canvas.getContext("2d", {
				alpha: true,
				willReadFrequently: false,
			});
			if (!ctx) throw new Error("Failed to create chunk canvas context");

			chunk = { canvas, ctx, isDirty: true };
			this.chunkCache.set(chunkAddr, chunk);
		}

		return chunk;
	}
	private renderChunkToCache(chunkX: number, chunkY: number): void {
		const chunk = this.getOrCreateChunkCache(chunkX, chunkY);
		if (!chunk.isDirty) return;

		const ctx = chunk.ctx;
		const ppuX = this.cachePPU.x;
		const ppuY = this.cachePPU.y;

		// Clear the chunk canvas
		ctx.clearRect(0, 0, chunk.canvas.width, chunk.canvas.height);

		// Render all tiles in this chunk
		for (let dx = 0; dx < this.chunkSize; dx++) {
			for (let dy = 0; dy < this.chunkSize; dy++) {
				const x = chunkX + dx;
				const y = chunkY + dy;
				const tileId = this.getTile(x, y);
				const tileImage = this.tileSet.get(tileId);

				if (tileId > 0) {
					if (!tileImage) {
						// Fallback magenta checkerboard
						ctx.fillStyle = "#000";
						ctx.fillRect(dx * ppuX, dy * ppuY, ppuX, ppuY);
						ctx.fillStyle = "#f0f";
						ctx.fillRect(dx * ppuX, dy * ppuY, ppuX / 2, ppuY / 2);
						ctx.fillRect(
							dx * ppuX + ppuX / 2,
							dy * ppuY + ppuY / 2,
							ppuX / 2,
							ppuY / 2,
						);
					} else {
						ctx.drawImage(tileImage, dx * ppuX, dy * ppuY, ppuX, ppuY);
					}
				}

				// Apply lighting overlay
				if (this.lightMapRef) {
					const lightValue = this.lightMapRef.getLighting(x, y);
					ctx.fillStyle = `rgba(0, 0, 0, ${1 - lightValue})`;
					ctx.fillRect(dx * ppuX, dy * ppuY, ppuX, ppuY);
				}
			}
		}

		chunk.isDirty = false;
	}

	public render(g: CanvasRenderingContext2D): void {
		const camera = this.game.getCamera();
		const bounds = camera?.getBounds();

		if (!camera || !bounds) return;
		const [min, max] = bounds;

		// Disable image smoothing for crisp pixel art
		g.imageSmoothingEnabled = false;

		// Calculate which chunks are visible
		const minChunkX = Math.floor(min.x / this.chunkSize) * this.chunkSize;
		const maxChunkX = Math.floor(max.x / this.chunkSize) * this.chunkSize;
		const minChunkY = Math.floor(min.y / this.chunkSize) * this.chunkSize;
		const maxChunkY = Math.floor(max.y / this.chunkSize) * this.chunkSize;

		this.lightMapRef?.updateDirtyChunks();

		// Render each visible chunk
		for (
			let chunkX = minChunkX;
			chunkX <= maxChunkX;
			chunkX += this.chunkSize
		) {
			for (
				let chunkY = minChunkY;
				chunkY <= maxChunkY;
				chunkY += this.chunkSize
			) {
				// Update chunk cache if dirty
				this.renderChunkToCache(chunkX, chunkY);

				// Get the cached chunk
				const chunk = this.getOrCreateChunkCache(chunkX, chunkY);

				// Draw the entire chunk with a single drawImage call
				// The cached chunk is always rendered at cachePPU resolution,
				// but we draw it scaled to match the current world coordinates
				const worldX = chunkX + this.entity.transform.position.x;
				const worldY = chunkY + this.entity.transform.position.y;

				g.drawImage(
					chunk.canvas,
					worldX,
					worldY,
					this.chunkSize,
					this.chunkSize,
				);
			}
		}
	}
}

export default TileMap;
