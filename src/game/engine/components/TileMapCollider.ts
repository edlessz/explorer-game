import Component from "../Component";
import type { Vector2 } from "../types";
import TileMap from "./TileMap";
import TileRegistry from "./TileRegistry";

interface TileMapContainer {
	tileMap: TileMap;
	tileRegistry: TileRegistry | null;
}

class TileMapCollider extends Component {
	private tileMaps: TileMapContainer[] = [];
	private readonly edgeBoundary: number = 0.001;

	public setup(): void {
		const foundTileMaps = this.game.getEntitiesWithComponent(TileMap);

		this.tileMaps = foundTileMaps
			.map((tm) => ({
				tileMap: tm.getComponent(TileMap),
				tileRegistry: tm.getComponent(TileRegistry),
			}))
			.filter((tm) => tm.tileMap !== null) as TileMapContainer[];
	}

	public grounded(): boolean {
		this.entity.transform.position.y += this.edgeBoundary;
		const isColliding = this.colliding();
		this.entity.transform.position.y -= this.edgeBoundary;
		return isColliding;
	}

	private pairsCollidingWithTileMap(
		pairs: Vector2[],
		tileMap: TileMapContainer,
	): boolean {
		const x = this.entity.transform.position.x;
		const y = this.entity.transform.position.y;
		const tileMapPos = tileMap.tileMap.entity.transform.position;
		return pairs.some(({ x: dx, y: dy }) => {
			const tileId =
				tileMap.tileMap.getTile(dx + x - tileMapPos.x, dy + y - tileMapPos.y) ??
				0;
			if (!tileMap.tileRegistry) return tileId > 0;

			const tileEntry = tileMap.tileRegistry.getTileEntry(tileId);
			return tileEntry?.solid ?? false;
		});
	}
	public colliding(): boolean {
		if (this.tileMaps.length === 0) return false;

		let xs = Array.from(
			{ length: this.entity.transform.scale.x + 1 },
			(_, i) => i - this.entity.transform.scale.x / 2,
		);
		let ys = Array.from(
			{ length: this.entity.transform.scale.y + 1 },
			(_, i) => i - this.entity.transform.scale.y / 2,
		);

		if (xs.length === 1)
			xs = [
				-this.entity.transform.scale.x / 2,
				this.entity.transform.scale.x / 2,
			];
		if (ys.length === 1)
			ys = [
				-this.entity.transform.scale.y / 2,
				this.entity.transform.scale.y / 2,
			];

		const edgeBoundary = 0.001;
		xs[0] += edgeBoundary;
		xs[xs.length - 1] -= edgeBoundary;
		ys[0] += edgeBoundary;
		ys[ys.length - 1] -= edgeBoundary;

		const pairs: Vector2[] = xs.flatMap((x) => ys.map((y) => ({ x, y })));

		return this.tileMaps.some((tileMap) =>
			this.pairsCollidingWithTileMap(pairs, tileMap),
		);
	}
}

export default TileMapCollider;
