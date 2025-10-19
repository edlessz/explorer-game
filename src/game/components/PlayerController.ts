import Component from "../engine/Component";
import TileMap from "../engine/components/TileMap";
import type { Vector2 } from "../engine/types";

class PlayerController extends Component {
	private speed: number = 5;
	private world: TileMap | null = null;

	private velocity: Vector2 = { x: 0, y: 0 };

	public setup(): void {
		this.world =
			(this.entity.game
				.getEntities()
				.flatMap((e) => e.getComponents())
				.find((c) => c instanceof TileMap) as TileMap) || null;

		this.entity.transform.scale = { x: 1, y: 2 };
	}

	public update(deltaTime: number): void {
		const input = this.entity.game.input;
		const direction =
			(input.isKeyPressed("ArrowRight") ? 1 : 0) -
			(input.isKeyPressed("ArrowLeft") ? 1 : 0);
		if (input.isKeyPressed("ArrowUp") && this.grounded()) this.velocity.y = -5;

		this.velocity.x = direction * this.speed;
		this.velocity.y += 0.1;

		this.entity.transform.position.x += this.velocity.x * deltaTime;
		if (this.colliding()) {
			const sign = Math.sign(this.velocity.x);
			while (this.colliding() && sign !== 0) {
				this.entity.transform.position.x -= sign * 0.01;
				this.velocity.x = 0;
			}
			this.entity.transform.position.x =
				Math.round(this.entity.transform.position.x * 2) / 2;
		}
		this.entity.transform.position.y += this.velocity.y * deltaTime;
		if (this.colliding()) {
			const sign = Math.sign(this.velocity.y);
			while (this.colliding() && sign !== 0) {
				this.entity.transform.position.y -= sign * 0.01;
				this.velocity.y = 0;
			}
			this.entity.transform.position.y =
				Math.round(this.entity.transform.position.y * 2) / 2;
		}
	}

	public grounded(): boolean {
		this.entity.transform.position.y += 0.01;
		const isColliding = this.colliding();
		this.entity.transform.position.y -= 0.01;
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

export default PlayerController;
