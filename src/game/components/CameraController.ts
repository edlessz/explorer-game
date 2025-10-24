import Component from "../engine/Component";
import Camera from "../engine/components/Camera";
import type Entity from "../engine/Entity";
import type { Vector2 } from "../engine/types";

class CameraController extends Component {
	public cameraSpeed: number = 10;
	public followTarget: Entity | null = null;

	private camera: Camera | null = null;

	public setup(): void {
		this.followTarget = this.game.getEntity("player");
		this.camera = this.entity.getComponent(Camera);

		console.log(this);
	}

	public update(deltaTime: number): void {
		if (!this.followTarget) return;

		const targetPos = this.followTarget.transform.position;
		const cameraPos = this.entity.transform.position;

		const direction: Vector2 = {
			x: targetPos.x - cameraPos.x,
			y: targetPos.y - cameraPos.y,
		};

		this.entity.transform.position.x +=
			direction.x * this.cameraSpeed * deltaTime;
		this.entity.transform.position.y +=
			direction.y * this.cameraSpeed * deltaTime;

		const input = this.game.input;
		if (this.camera) {
			if (input.isKeyPressed("=")) {
				this.camera.ppuX *= 1.1;
				this.camera.ppuY *= 1.1;

				this.camera.ppuX = Math.round(this.camera.ppuX);
				this.camera.ppuY = Math.round(this.camera.ppuY);
			}
			if (input.isKeyPressed("-")) {
				this.camera.ppuX /= 1.1;
				this.camera.ppuY /= 1.1;

				this.camera.ppuX = Math.round(this.camera.ppuX);
				this.camera.ppuY = Math.round(this.camera.ppuY);
			}
		}
	}
}

export default CameraController;
