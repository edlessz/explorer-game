import Component from "../engine/Component";
import ColorRenderer from "../engine/components/ColorRenderer";
import TileMap from "../engine/components/TileMap";
import type { Vector2 } from "../engine/types";

class CursorController extends Component {
	public mouseScreenPos: Vector2 | null = null;

	private colorRenderer: ColorRenderer | null = null;
	private editableTileMap: TileMap | null = null;

	public setup(): void {
		this.colorRenderer = this.entity.getComponent(ColorRenderer);
		this.editableTileMap =
			this.game
				.getEntitiesWithComponent(TileMap)
				.find((e) => e.tag === "editableTileMap")
				?.getComponent(TileMap) ?? null;

		this.entity.transform.position.z = 5;
	}

	public update(_deltaTime: number): void {
		if (this.colorRenderer) this.colorRenderer.enabled = false;
		if (!this.mouseScreenPos) return;
		const camera = this.game.getCamera();
		if (!camera) return;

		const worldPos = camera.screenToWorld(
			this.mouseScreenPos.x,
			this.mouseScreenPos.y,
		);
		if (!worldPos) return;

		this.entity.transform.position.x = Math.floor(worldPos.x) + 0.5;
		this.entity.transform.position.y = Math.floor(worldPos.y) + 0.5;

		if (this.colorRenderer) this.colorRenderer.enabled = true;
	}

	public onMouseMove(event: MouseEvent): void {
		this.mouseScreenPos = {
			x: event.clientX,
			y: event.clientY,
		};

		const input = this.game.input;
		if (input.isMouseButtonPressed(0)) {
			this.place(4);
		}
		if (input.isMouseButtonPressed(2)) {
			this.place(0);
		}
	}

	public onMouseUp(_event: MouseEvent): void {
		const input = this.game.input;
		if (input.isMouseButtonPressed(0)) {
			this.place(4);
		}
		if (input.isMouseButtonPressed(2)) {
			this.place(0);
		}
	}

	public place(tileId: number): void {
		if (!this.editableTileMap) return;

		this.editableTileMap.setTile(
			this.entity.transform.position.x,
			this.entity.transform.position.y,
			tileId,
		);
	}
}

export default CursorController;
