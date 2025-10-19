import Component from "../Component";
import type { Vector2 } from "../types";
import TileMap from "./TileMap";

class TileMapCollider extends Component {
	private world: TileMap | null = null;
	private readonly edgeBoundary: number = 0.001;

	public setup(): void {
		this.world =
			(this.entity.game
				.getEntities()
				.flatMap((e) => e.getComponents())
				.find((c) => c instanceof TileMap) as TileMap) || null;
	}

	public grounded(): boolean {
		this.entity.transform.position.y += this.edgeBoundary;
		const isColliding = this.colliding();
		this.entity.transform.position.y -= this.edgeBoundary;
		return isColliding;
	}

	public colliding(): boolean {
		if (!this.world) return false;
		const xs = Array.from(
			{ length: this.entity.transform.scale.x + 1 },
			(_, i) => i - this.entity.transform.scale.x / 2,
		);
		const ys = Array.from(
			{ length: this.entity.transform.scale.y + 1 },
			(_, i) => i - this.entity.transform.scale.y / 2,
		);

		const edgeBoundary = 0.001;
		xs[0] += edgeBoundary;
		xs[xs.length - 1] -= edgeBoundary;
		ys[0] += edgeBoundary;
		ys[ys.length - 1] -= edgeBoundary;

		const pairs: Vector2[] = xs.flatMap((x) => ys.map((y) => ({ x, y })));

		const x = this.entity.transform.position.x;
		const y = this.entity.transform.position.y;
		return pairs.some(
			({ x: dx, y: dy }) => (this.world?.getTile(x + dx, y + dy) ?? 0) > 0,
		);
	}
}

export default TileMapCollider;
