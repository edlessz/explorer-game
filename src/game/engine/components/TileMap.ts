import Component from "../Component";

type Address = `${number},${number}`;

class TileMap extends Component {
	private tiles: Map<Address, number> = new Map();

	public setup() {
		this.tiles = new Map([
			...new Array(20).fill(0).map((_, i) => {
				const x = i % 10;
				const y = Math.floor(i / 10) + 4;
				return [this.encodeAddress(x, y), 1] as [Address, number];
			}),
			["0,2", 1],
			["0,3", 1],
			["9,2", 1],
			["9,3", 1],
		]);
	}

	public setTile(x: number, y: number, tileId: number): void {
		const addr = this.encodeAddress(x, y);
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
		for (const [addr, tileId] of this.tiles) {
			if (tileId === 0) continue;

			const { x, y } = this.decodeAddress(addr);
			g.fillStyle = "#000";
			g.fillRect(x, y, 1, 1);
		}
	}
}

export default TileMap;
