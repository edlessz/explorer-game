import Component from "../Component";
import type { Vector2 } from "../types";
import TileMap from "./TileMap";

class TileMapCollider extends Component {
	private tileMaps: TileMap[] = [];
	private readonly edgeBoundary: number = 0.001;

	public setup(): void {
		this.tileMaps = this.entity.game
			.getEntitiesWithComponent(TileMap)
			.map((e) => e.getComponent(TileMap))
			.filter((tm): tm is TileMap => tm !== null);
	}

	public grounded(): boolean {
		this.entity.transform.position.y += this.edgeBoundary;
		const isColliding = this.colliding();
		this.entity.transform.position.y -= this.edgeBoundary;
		return isColliding;
	}

	private pairsCollidingWithTileMap(
		pairs: Vector2[],
		tileMap: TileMap,
	): boolean {
		const x = this.entity.transform.position.x;
		const y = this.entity.transform.position.y;
		const tileMapPos = tileMap.entity.transform.position;
		return pairs.some(
			({ x: dx, y: dy }) =>
				(tileMap.getTile(dx + x - tileMapPos.x, dy + y - tileMapPos.y) ?? 0) >
				0,
		);
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
