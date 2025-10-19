import type Game from "./Game";
import type { Transform } from "./types";

abstract class Entity {
	public game: Game | null = null;
	public destroyed: boolean = false;
	public markedForDestruction: boolean = false;

	public transform: Transform = {
		position: { x: 0, y: 0 },
		rotation: 0,
		scale: { x: 1, y: 1 },
	};

	public setup(): void {}
	public update(deltaTime: number): void {
		void deltaTime;
	}
	public render(g: CanvasRenderingContext2D): void {
		void g;
	}

	public onMouseDown(event: MouseEvent): void {
		void event;
	}
	public onMouseMove(event: MouseEvent): void {
		void event;
	}
	public onMouseUp(event: MouseEvent): void {
		void event;
	}
	public destroy(): void {
		this.markedForDestruction = true;
	}
}

export default Entity;
