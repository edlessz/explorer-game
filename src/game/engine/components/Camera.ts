import Component from "../Component";
import type { Vector2 } from "../types";

class Camera extends Component {
	public ppuX: number = 16; // pixels per unit
	public ppuY: number = 16; // pixels per unit

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

	public applyTransform(g: CanvasRenderingContext2D): void {
		const { position, rotation } = this.entity.transform;
		const viewport = this.entity.game.getViewport();
		if (!viewport) return;

		g.scale(this.ppuX, this.ppuY);
		g.rotate(-rotation);
		const translation = this.roundPositionToPixel(
			viewport.width / 2 / this.ppuX - position.x,
			viewport.height / 2 / this.ppuY - position.y,
		);
		g.translate(translation.x, translation.y);
	}

	public roundPositionToPixel(x: number, y: number): Vector2 {
		return {
			x: Math.round(x * this.ppuX) / this.ppuX,
			y: Math.round(y * this.ppuY) / this.ppuY,
		};
	}

	public screenToWorld(screenX: number, screenY: number): Vector2 | null {
		const viewport = this.entity.game.getViewport();
		if (!viewport) return null;

		const { position, rotation } = this.entity.transform;

		// Convert screen coordinates to normalized device coordinates (-1 to 1)
		const ndcX = (screenX / viewport.width) * 2 - 1;
		const ndcY = 1 - (screenY / viewport.height) * 2;

		// Convert NDC to world coordinates
		const worldX =
			position.x +
			(ndcX * (viewport.width / this.ppuX)) / 2 / this.entity.transform.scale.x;
		const worldY =
			position.y +
			(ndcY * (viewport.height / this.ppuY)) /
				2 /
				-this.entity.transform.scale.y;

		// Apply inverse rotation
		const cos = Math.cos(rotation);
		const sin = Math.sin(rotation);
		const dx = worldX - position.x;
		const dy = worldY - position.y;

		return {
			x: position.x + (dx * cos + dy * sin),
			y: position.y + (-dx * sin + dy * cos),
		};
	}
}

export default Camera;
