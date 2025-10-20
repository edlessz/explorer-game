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
	private caveMap: Map<Address, boolean> = new Map();

	private generateChunk(cx: number, cy: number): void {
		if (!this.tileMapRef) return;
		const addr = this.encodeAddress(cx, cy);
		if (this.generatedChunks.has(addr)) return;

		// First pass: generate terrain and initial cave noise
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
				} else if (y === landstripHeight) {
					this.tileMapRef.setTile(x, landstripHeight, 2);
				}

				// Enhanced cave generation with multiple frequencies
				const caveValue = this.getCaveValue(x, y);
				this.caveMap.set(this.encodeAddress(x, y), caveValue);
			}
		}

		// Second pass: apply cellular automata smoothing
		const smoothedCaves = this.smoothCaves(cx, cy);

		// Third pass: carve out caves
		for (let dx = 0; dx < this.chunkSize.x; dx++) {
			const x = cx + dx;
			for (let dy = 0; dy < this.chunkSize.y; dy++) {
				const y = cy + dy;
				const addr = this.encodeAddress(x, y);

				if (smoothedCaves.get(addr)) {
					this.tileMapRef.setTile(x, y, 0);
				}
			}
		}

		this.generatedChunks.add(addr);
	}

	private getCaveValue(x: number, y: number): boolean {
		// Layer 1: Large cave systems (scale 40)
		const largeCaves = this.normalizedNoise(x / 40, y / 40);

		// Layer 2: Medium tunnels (scale 20)
		const mediumCaves = this.normalizedNoise(x / 20, y / 20);

		// Layer 3: Small pockets (scale 10)
		const smallCaves = this.normalizedNoise(x / 10, y / 10);

		// Layer 4: Vertical variation using different seed offset
		const verticalNoise = this.normalizedNoise(x / 30, y / 30 + 1000);

		// Combine layers with weights
		const combined =
			largeCaves * 0.5 +
			mediumCaves * 0.3 +
			smallCaves * 0.15 +
			verticalNoise * 0.05;

		// Create caves where the combined value is below threshold
		// Lower threshold = more caves
		return combined < 0.42;
	}

	private smoothCaves(cx: number, cy: number): Map<Address, boolean> {
		const smoothed = new Map<Address, boolean>();

		// Apply cellular automata rules for more organic caves
		for (let dx = 0; dx < this.chunkSize.x; dx++) {
			const x = cx + dx;
			for (let dy = 0; dy < this.chunkSize.y; dy++) {
				const y = cy + dy;

				// Count cave neighbors in 3x3 grid
				let caveNeighbors = 0;
				for (let ox = -1; ox <= 1; ox++) {
					for (let oy = -1; oy <= 1; oy++) {
						if (ox === 0 && oy === 0) continue;
						const neighborAddr = this.encodeAddress(x + ox, y + oy);
						if (this.caveMap.get(neighborAddr)) {
							caveNeighbors++;
						}
					}
				}

				// Smoothing rules:
				// - If 5+ neighbors are caves, become a cave
				// - If 4- neighbors are caves, become solid
				const addr = this.encodeAddress(x, y);
				const isCave = caveNeighbors >= 5;
				smoothed.set(addr, isCave);
			}
		}

		return smoothed;
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
