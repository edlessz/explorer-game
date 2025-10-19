import type Entity from "./Entity";

class Component {
	public enabled: boolean = true;
	public readonly entity: Entity;

	constructor(entity: Entity) {
		this.entity = entity;
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
}

export type ComponentConstructor<T extends Component> = new (
	entity: Entity,
	...args: unknown[]
) => T;

export default Component;
