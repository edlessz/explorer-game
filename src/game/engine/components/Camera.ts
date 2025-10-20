import Component from "../Component";
import type { Vector2 } from "../types";

class Camera extends Component {
	public ppuX: number = 32; // pixels per unit
	public ppuY: number = 32; // pixels per unit

	public getBounds(): [Vector2, Vector2] | null {
		const viewport = this.entity.game.getViewport();
		if (!viewport) return null;

		const { position, scale } = this.entity.transform;

		const halfWidth = viewport.width / this.ppuX / 2 / scale.x;
		const halfHeight = viewport.height / this.ppuY / 2 / scale.y;

		const min: Vector2 = {
			x: position.x - halfWidth,
			y: position.y - halfHeight,
		};
		const max: Vector2 = {
			x: position.x + halfWidth,
			y: position.y + halfHeight,
		};

		return [min, max];
	}
}

export default Camera;
