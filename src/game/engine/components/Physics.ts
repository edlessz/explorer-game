import Component from "../Component";
import type { Vector2 } from "../types";
import TileMapCollider from "./TileMapCollider";

class Physics extends Component {
	public static gravity: number = 9.81;

	public velocity: Vector2 = { x: 0, y: 0 };
	public acceleration: Vector2 = { x: 0, y: Physics.gravity };

	public tileMapColliderRef: TileMapCollider | null = null;

	public setup(): void {
		this.tileMapColliderRef = this.entity.getComponent(TileMapCollider);
	}

	public update(deltaTime: number): void {
		this.velocity.x += this.acceleration.x * deltaTime;
		this.velocity.y += this.acceleration.y * deltaTime;

		this.entity.transform.position.x += this.velocity.x * deltaTime;
		if (this.tileMapColliderRef?.colliding()) {
			const sign = Math.sign(this.velocity.x);
			while (this.tileMapColliderRef.colliding() && sign !== 0) {
				this.entity.transform.position.x -= sign * 0.01;
				this.velocity.x = 0;
			}
			this.entity.transform.position.x =
				Math.round(this.entity.transform.position.x * 2) / 2;
		}
		this.entity.transform.position.y += this.velocity.y * deltaTime;
		if (this.tileMapColliderRef?.colliding()) {
			const sign = Math.sign(this.velocity.y);
			while (this.tileMapColliderRef.colliding() && sign !== 0) {
				this.entity.transform.position.y -= sign * 0.01;
				this.velocity.y = 0;
			}
			this.entity.transform.position.y =
				Math.round(this.entity.transform.position.y * 2) / 2;
		}
	}
}

export default Physics;
