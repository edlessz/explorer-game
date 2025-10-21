import type Component from "./Component";
import type { ComponentConstructor } from "./Component";
import Camera from "./components/Camera";
import Entity from "./Entity";
import FrameTimer from "./FrameTimer";
import Input from "./Input";

class Game {
	private viewport: HTMLCanvasElement | null = null;
	private context: CanvasRenderingContext2D | null = null;
	private resizeObserver = new ResizeObserver(this.handleResize.bind(this));
	private frameTimer = new FrameTimer();
	public input = new Input();

	private entities: Entity[] = [];
	private camera: Entity | null = null;
	private cameraComponent: Camera | null = null;

	private debugList: string[] = [];
	public debug(s: string): void {
		this.debugList.push(s);
	}

	public setCamera(entity: Entity | null): void {
		if (entity === null) {
			this.camera = null;
			this.cameraComponent = null;
			return;
		}

		this.cameraComponent = entity.getComponent(Camera);
		if (!this.cameraComponent)
			throw new Error("Entity does not have a Camera component");
		this.camera = entity;
	}
	public getCamera(): Camera | null {
		return this.cameraComponent;
	}

	private dispatchToComponents<E extends MouseEvent | KeyboardEvent>(
		event: E,
		method: (component: Component, event: E) => void,
	): void {
		for (const entity of this.entities) {
			for (const component of entity.getComponents()) {
				if (component.enabled) method(component, event);
			}
		}
	}

	private handleResize(entries: ResizeObserverEntry[]): void {
		for (const entry of entries) {
			const target = entry.target as HTMLCanvasElement;

			const width = target.clientWidth;
			const height = target.clientHeight;
			const dpr = window.devicePixelRatio || 1;

			target.width = width * dpr;
			target.height = height * dpr;
		}

		if (!this.context) return;
		this.render(this.context);
	}

	public setViewport(canvas: HTMLCanvasElement | null): void {
		if (this.viewport) {
			// Unhook
			this.resizeObserver.unobserve(this.viewport);
			this.input.cleanup();
		}
		this.viewport = canvas;
		this.context = canvas?.getContext("2d") ?? null;
		if (canvas) {
			// Hook
			this.resizeObserver.observe(canvas);
			this.input.initialize(canvas);

			// Automatically dispatch all input events to enabled components
			this.input.onMouseDown = (event) => {
				this.dispatchToComponents(event, (c, e) => c.onMouseDown(e));
			};
			this.input.onMouseMove = (event) => {
				this.dispatchToComponents(event, (c, e) => c.onMouseMove(e));
			};
			this.input.onMouseUp = (event) => {
				this.dispatchToComponents(event, (c, e) => c.onMouseUp(e));
			};
			this.input.onKeyDown = (event) => {
				this.dispatchToComponents(event, (c, e) => c.onKeyDown(e));
			};
			this.input.onKeyUp = (event) => {
				this.dispatchToComponents(event, (c, e) => c.onKeyUp(e));
			};
		}
	}
	public getViewport(): HTMLCanvasElement | null {
		return this.viewport;
	}

	public start(): void {
		requestAnimationFrame(this.gameLoop.bind(this));

		for (const entity of this.entities) {
			entity.setup();
		}
	}

	private gameLoop(): void {
		const deltaTime = this.frameTimer.recordFrame();

		if (!this.context || !this.viewport) {
			requestAnimationFrame(this.gameLoop.bind(this));
			return;
		}

		this.update(deltaTime);
		this.render(this.context);
		this.entities = this.entities.filter((e) => !e.markedForDestruction);

		requestAnimationFrame(this.gameLoop.bind(this));
	}

	private update(deltaTime: number): void {
		for (const entity of this.entities) {
			if (!entity.enabled) continue;
			entity.update(deltaTime);
		}
	}

	private render(g: CanvasRenderingContext2D): void {
		if (!this.viewport) return;

		g.resetTransform();
		g.clearRect(0, 0, this.viewport.width, this.viewport.height);

		if (this.camera && this.cameraComponent) {
			this.cameraComponent.applyTransform(g);
			for (const entity of this.entities) {
				if (!entity.enabled) continue;
				entity.render(g);
			}
		}

		g.resetTransform();
		g.globalCompositeOperation = "difference";
		g.fillStyle = "white";
		g.font = "16px monospace";
		g.fillText(`FPS: ${this.frameTimer.getFPS().toFixed(2)}`, 10, 20);

		for (let i = 0; i < this.debugList.length; i++) {
			g.fillText(this.debugList[i], 10, 40 + i * 20);
		}
		g.globalCompositeOperation = "source-over";
		this.debugList = [];
	}

	public addEntity(): Entity {
		const entity = new Entity(this);
		this.entities.push(entity);
		return entity;
	}
	public getEntitiesWithComponent<T extends Component>(
		ctor: ComponentConstructor<T>,
	): Entity[] {
		return this.entities.filter((e) => e.getComponent(ctor));
	}
	public getEntity(tag: string): Entity | null {
		return this.entities.find((e) => e.tag === tag) || null;
	}
}

export default Game;
