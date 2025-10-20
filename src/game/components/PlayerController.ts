import Component from "../engine/Component";
import Physics from "../engine/components/Physics";
import TileMapCollider from "../engine/components/TileMapCollider";

class PlayerController extends Component {
	private moveSpeed: number = 10;
	private jumpHeight: number = 10;

	private physicsRef: Physics | null = null;
	private tileMapColliderRef: TileMapCollider | null = null;

	public setup(): void {
		this.physicsRef = this.entity.getComponent(Physics);
		this.tileMapColliderRef = this.entity.getComponent(TileMapCollider);

		this.entity.transform.scale = {
			x: 2,
			y: 3,
		};
	}

	public update(_deltaTime: number): void {
		if (!this.physicsRef) return;

		const input = this.game.input;
		const direction =
			(input.isKeyPressed("ArrowRight") ? 1 : 0) -
			(input.isKeyPressed("ArrowLeft") ? 1 : 0);
		this.physicsRef.velocity.x = direction * this.moveSpeed;

		if (input.isKeyPressed("ArrowUp") && this.tileMapColliderRef?.grounded()) {
			this.physicsRef.velocity.y = -this.jumpHeight;
		}

		this.game.debug(
			`X: ${this.entity.transform.position.x.toFixed(2)} Y: ${this.entity.transform.position.y.toFixed(2)}`,
		);
	}
}

export default PlayerController;
