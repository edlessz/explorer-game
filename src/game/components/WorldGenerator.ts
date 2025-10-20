import { createNoise2D } from "simplex-noise";
import Component from "../engine/Component";
import TileMap from "../engine/components/TileMap";
import type { Vector2 } from "../engine/types";

type Address = `${number},${number}`;

class WorldGenerator extends Component {
	private seed: number = 0.5;
	private noise2D = createNoise2D(() => this.seed);
	private tileMapRef: TileMap | null = null;

	private readonly chunkSize: Vector2 = { x: 32, y: 32 };

	public setup(): void {
		this.tileMapRef = this.entity.getComponent(TileMap);
	}

	public update(_deltaTime: number): void {
		const camera = this.entity.game.getCamera();
		if (!camera) return;
		const bounds = camera.getBounds();
		if (!bounds) return;

		const [min, max] = bounds;

		const minChunkX = Math.floor(min.x / this.chunkSize.x) * this.chunkSize.x;
		const maxChunkX = Math.floor(max.x / this.chunkSize.x) * this.chunkSize.x;
		const minChunkY = Math.floor(min.y / this.chunkSize.y) * this.chunkSize.y;
		const maxChunkY = Math.floor(max.y / this.chunkSize.y) * this.chunkSize.y;

		for (let cx = minChunkX; cx <= maxChunkX; cx += this.chunkSize.x) {
			for (let cy = minChunkY; cy <= maxChunkY; cy += this.chunkSize.y) {
				this.generateChunk(cx, cy);
			}
		}
	}

	private generatedChunks: Set<Address> = new Set();
	private generateChunk(cx: number, cy: number): void {
		if (!this.tileMapRef) return;
		const addr = this.encodeAddress(cx, cy);
		if (this.generatedChunks.has(addr)) return;

		for (let dx = 0; dx < this.chunkSize.x; dx++) {
			const x = cx + dx;
			for (let dy = 0; dy < this.chunkSize.y; dy++) {
				const y = cy + dy;

				let noise = this.normalizedNoise(x / 50, this.seed);
				noise += this.normalizedNoise((x / 50) ** (1 / 2), this.seed) / 2;
				noise += this.normalizedNoise((x / 50) ** (1 / 4), this.seed) / 4;

				const landstripHeight = Math.floor(noise * 10);

				const stoneHeight =
					landstripHeight + this.normalizedNoise(x / 2, this.seed) * 4 + 5;

				if (y > stoneHeight) {
					this.tileMapRef.setTile(x, y, 3);
				} else if (y > landstripHeight) {
					this.tileMapRef.setTile(x, y, 1);
				}
				this.tileMapRef.setTile(x, landstripHeight, 2);
			}
		}

		this.generatedChunks.add(addr);
	}

	private normalizedNoise(x: number, y: number): number {
		const noiseValue = this.noise2D(x, y);
		return (noiseValue + 1) / 2; // Normalize to [0, 1]
	}

	private encodeAddress(x: number, y: number): Address {
		return `${Math.floor(x)},${Math.floor(y)}`;
	}
}

export default WorldGenerator;
