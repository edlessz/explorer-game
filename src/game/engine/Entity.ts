import type Component from "./Component";
import type { ComponentConstructor } from "./Component";
import type Game from "./Game";
import type { Transform } from "./types";

class Entity {
	public readonly game: Game;
	public enabled: boolean = true;
	public markedForDestruction: boolean = false;

	public tag: string | null = null;

	public transform: Transform = {
		position: { x: 0, y: 0, z: 0 },
		rotation: 0,
		scale: { x: 1, y: 1 },
	};

	private components: Component[] = [];

	constructor(game: Game) {
		this.game = game;
	}

	public setup(): void {
		for (const component of this.components) {
			component.setup();
		}
	}
	public update(deltaTime: number): void {
		for (const component of this.components) {
			if (component.enabled) component.update(deltaTime);
		}
	}
	public render(g: CanvasRenderingContext2D): void {
		for (const component of this.components) {
			if (component.enabled) component.render(g);
		}
	}

	public getComponents(): ReadonlyArray<Component> {
		return this.components;
	}
	public getComponent<T extends Component>(
		ctor: ComponentConstructor<T>,
	): T | null {
		for (const component of this.components) {
			if (component instanceof ctor) {
				return component as T;
			}
		}
		return null;
	}
	public addComponent<T extends Component>(ctor: ComponentConstructor<T>): T {
		const component = new ctor(this);
		this.components.push(component);
		return component;
	}

	public destroy(): void {
		this.markedForDestruction = true;
	}
}

export default Entity;
