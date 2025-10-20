import Component from "../Component";

type Address = `${number},${number}`;

class TileMap extends Component {
	private tiles: Map<Address, number> = new Map();

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

	public decodeAddress(addr: Address): { x: number; y: number } {
		const [x, y] = addr.split(",").map(Number);
		return { x, y };
	}
	public encodeAddress(x: number, y: number): Address {
		return `${Math.floor(x)},${Math.floor(y)}`;
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

				if (tileId === 0) continue;
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
					const pos = camera?.roundPositionToPixel(
						x + this.entity.transform.position.x,
						y + this.entity.transform.position.y,
					);
					g.drawImage(tileImage, pos.x, pos.y, 1, 1);
				}
			}
		}
	}
}

export default TileMap;
