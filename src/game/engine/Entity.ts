import type Component from "./Component";
import type Game from "./Game";
import type { Transform } from "./types";

class Entity {
	public readonly game: Game;
	public enabled: boolean = true;
	public markedForDestruction: boolean = false;

	public transform: Transform = {
		position: { x: 0, y: 0 },
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
	public addComponent<T extends Component>(
		ctor: new (entity: Entity, ...args: unknown[]) => T,
	): T {
		const component = new ctor(this);
		this.components.push(component);
		return component;
	}

	public destroy(): void {
		this.markedForDestruction = true;
	}
}

export default Entity;
