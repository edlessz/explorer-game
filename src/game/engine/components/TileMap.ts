import Component from "../Component";

class TileMap extends Component {
	private tiles: Map<number, number> = new Map();
	public lightingEnabled = true;

	public tileSet: Map<number, HTMLImageElement> = new Map();

	public setTile(x: number, y: number, tileId: number): void {
		const addr = this.encodeAddress(
			x - this.entity.transform.position.x,
			y - this.entity.transform.position.y,
		);
		this.tiles.set(addr, tileId);
	}
	public getTile(x: number, y: number): number {
		const addr = this.encodeAddress(x, y);
		return this.tiles.get(addr) ?? 0;
	}

	public decodeAddress(addr: number): { x: number; y: number } {
		// Extract x from upper 16 bits (with sign extension)
		const x = addr >> 16;
		// Extract y from lower 16 bits (with sign extension)
		const y = (addr << 16) >> 16;
		return { x, y };
	}
	public encodeAddress(x: number, y: number): number {
		// Pack x into upper 16 bits, y into lower 16 bits
		// Supports coordinates from -32768 to 32767
		return (Math.floor(x) << 16) | (Math.floor(y) & 0xffff);
	}

	public render(g: CanvasRenderingContext2D): void {
		const camera = this.entity.game.getCamera();
		const bounds = camera?.getBounds();

		if (!camera || !bounds) return;
		const [min, max] = bounds;

		for (let x = Math.floor(min.x); x <= max.x; x++) {
			for (let y = Math.floor(min.y); y <= max.y; y++) {
				const tileId = this.getTile(x, y);
				const tileImage = this.tileSet.get(tileId);

				if (tileId > 0) {
					if (!tileImage) {
						g.fillStyle = "#000";
						g.fillRect(
							x + this.entity.transform.position.x,
							y + this.entity.transform.position.y,
							1,
							1,
						);
						g.fillStyle = "#f0f";
						g.fillRect(
							x + this.entity.transform.position.x,
							y + this.entity.transform.position.y,
							0.5,
							0.5,
						);
						g.fillRect(
							x + this.entity.transform.position.x + 0.5,
							y + this.entity.transform.position.y + 0.5,
							0.5,
							0.5,
						);
					} else {
						g.drawImage(
							tileImage,
							x + this.entity.transform.position.x,
							y + this.entity.transform.position.y,
							1,
							1,
						);
					}
				}
				if (this.lightingEnabled) {
					g.fillStyle = `rgba(0, 0, 0, ${0.5})`;
					// g.fillStyle = `#000`;
					// g.globalAlpha = 0.5;
					g.fillRect(
						x + this.entity.transform.position.x,
						y + this.entity.transform.position.y,
						1,
						1,
					);
					// g.globalAlpha = 1.0;
				}
			}
		}
	}
}

export default TileMap;
