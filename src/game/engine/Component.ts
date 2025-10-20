import type Entity from "./Entity";
import type Game from "./Game";

class Component {
	public enabled: boolean = true;
	public readonly entity: Entity;
	public readonly game: Game;

	constructor(entity: Entity) {
		this.entity = entity;
		this.game = entity.game;
	}

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
	public onKeyDown(event: KeyboardEvent): void {
		void event;
	}
	public onKeyUp(event: KeyboardEvent): void {
		void event;
	}
}

export type ComponentConstructor<T extends Component> = new (
	entity: Entity,
	...args: unknown[]
) => T;

export default Component;
