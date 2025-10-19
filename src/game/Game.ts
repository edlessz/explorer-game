import type Entity from "./Entity";
import Input from "./Input";

class Game {
	private viewport: HTMLCanvasElement | null = null;
	private context: CanvasRenderingContext2D | null = null;
	private resizeObserver = new ResizeObserver(this.handleResize.bind(this));
	private input = new Input();

	private entities: Entity[] = [];

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
			this.input.onMouseDown = (event) => {
				for (const entity of this.entities) {
					entity.onMouseDown(event);
				}
			};
			this.input.onMouseMove = (event) => {
				for (const entity of this.entities) {
					entity.onMouseMove(event);
				}
			};
			this.input.onMouseUp = (event) => {
				for (const entity of this.entities) {
					entity.onMouseUp(event);
				}
			};
		}
	}

	public getEntitiesByType<T extends Entity>(
		ctor: new (...args: never[]) => T,
	): T[] {
		return this.entities.filter((e): e is T => e instanceof ctor);
	}
	public addEntity(entity: Entity): void {
		entity.game = this;
		entity.setup();
		this.entities.push(entity);
	}

	private lastTime: number = 0;
	public start(): void {
		this.lastTime = performance.now();
		requestAnimationFrame(this.gameLoop.bind(this));

		for (const entity of this.entities) {
			entity.game = this;
			entity.setup();
		}
	}

	private lastFps: number[] = [];
	private gameLoop(now: number): void {
		this.lastFps.push(1000 / (now - this.lastTime));
		const deltaTime = (now - this.lastTime) / 1000;
		this.lastTime = now;
		if (this.lastFps.length > 50) this.lastFps.shift();

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
			entity.update(deltaTime);
		}
	}

	private render(g: CanvasRenderingContext2D): void {
		if (!this.viewport) return;

		g.resetTransform();
		g.clearRect(0, 0, this.viewport.width, this.viewport.height);

		for (const entity of this.entities) {
			const { position, rotation, scale } = entity.transform;
			const cos = Math.cos(rotation);
			const sin = Math.sin(rotation);

			g.setTransform(
				scale.x * cos,
				scale.x * sin,
				-scale.y * sin,
				scale.y * cos,
				position.x + this.viewport.width / 2,
				position.y + this.viewport.height / 2,
			);
			entity.render(g);
		}

		g.resetTransform();
		g.fillStyle = "black";
		g.font = "16px monospace";
		const avgFps =
			this.lastFps.reduce((a, b) => a + b, 0) / this.lastFps.length;
		g.fillText(`FPS: ${avgFps.toFixed(2)}`, 10, 20);
	}
}

export default Game;
