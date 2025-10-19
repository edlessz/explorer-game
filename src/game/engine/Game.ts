import Entity from "./Entity";
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
					for (const component of entity.getComponents()) {
						if (component.enabled) component.onMouseDown(event);
					}
				}
			};
			this.input.onMouseMove = (event) => {
				for (const entity of this.entities) {
					for (const component of entity.getComponents()) {
						if (component.enabled) component.onMouseMove(event);
					}
				}
			};
			this.input.onMouseUp = (event) => {
				for (const entity of this.entities) {
					for (const component of entity.getComponents()) {
						if (component.enabled) component.onMouseUp(event);
					}
				}
			};
		}
	}

	private lastTime: number = 0;
	public start(): void {
		this.lastTime = performance.now();
		requestAnimationFrame(this.gameLoop.bind(this));

		for (const entity of this.entities) {
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
			if (!entity.enabled) continue;
			entity.update(deltaTime);
		}
	}

	private render(g: CanvasRenderingContext2D): void {
		if (!this.viewport) return;

		g.resetTransform();
		g.clearRect(0, 0, this.viewport.width, this.viewport.height);

		for (const entity of this.entities) {
			if (!entity.enabled) continue;

			g.translate(this.viewport.width / 2, this.viewport.height / 2);
			g.scale(32, 32);

			entity.render(g);
		}

		g.resetTransform();
		g.fillStyle = "black";
		g.font = "16px monospace";
		const avgFps =
			this.lastFps.reduce((a, b) => a + b, 0) / this.lastFps.length;
		g.fillText(`FPS: ${avgFps.toFixed(2)}`, 10, 20);
	}

	public addEntity(): Entity {
		const entity = new Entity(this);
		this.entities.push(entity);
		return entity;
	}
}

export default Game;
