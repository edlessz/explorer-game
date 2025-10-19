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
	}

	public update(deltaTime: number): void {
		const input = this.entity.game.input;
		if (input.isKeyPressed("ArrowRight")) {
			this.entity.transform.position.x += this.speed * deltaTime;
		}
		if (input.isKeyPressed("ArrowLeft")) {
			this.entity.transform.position.x -= this.speed * deltaTime;
		}

		this.velocity.x = 0;
		this.velocity.y += 0.1;

		this.entity.transform.position.x += this.velocity.x * deltaTime;
		if (this.colliding()) {
			const sign = Math.sign(this.velocity.x);
			while (this.colliding() && sign !== 0) {
				this.entity.transform.position.x -= sign * 0.01;
				this.velocity.x = 0;
			}
		}
		this.entity.transform.position.y += this.velocity.y * deltaTime;
		if (this.colliding()) {
			const sign = Math.sign(this.velocity.y);
			while (this.colliding() && sign !== 0) {
				this.entity.transform.position.y -= sign * 0.01;
				this.velocity.y = 0;
			}
		}
	}

	public colliding(): boolean {
		if (!this.world) return false;

		const x = Math.floor(this.entity.transform.position.x);
		const y = Math.floor(this.entity.transform.position.y);

		return [
			this.world.getTile(x - 0.5, y - 0.5) > 0,
			this.world.getTile(x + 0.5, y - 0.5) > 0,
			this.world.getTile(x - 0.5, y + 0.5) > 0,
			this.world.getTile(x + 0.5, y + 0.5) > 0,
		].includes(true);
	}
}

export default PlayerController;
