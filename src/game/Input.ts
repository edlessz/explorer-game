import type { Vector2 } from "./types";

class Input {
	private viewport: HTMLCanvasElement | null = null;

	public onMouseMove: ((event: MouseEvent) => void) | null = null;
	public onMouseDown: ((event: MouseEvent) => void) | null = null;
	public onMouseUp: ((event: MouseEvent) => void) | null = null;

	private listenerWrappers = {
		mouseMove: (event: MouseEvent) => this.onMouseMove?.(event),
		mouseDown: (event: MouseEvent) => this.onMouseDown?.(event),
		mouseUp: (event: MouseEvent) => this.onMouseUp?.(event),
	};

	public initialize(viewport: HTMLCanvasElement): void {
		this.viewport = viewport;
		this.viewport.addEventListener(
			"mousemove",
			this.listenerWrappers.mouseMove,
		);
		this.viewport.addEventListener(
			"mousedown",
			this.listenerWrappers.mouseDown,
		);
		this.viewport.addEventListener("mouseup", this.listenerWrappers.mouseUp);
	}
	public cleanup(): void {
		if (!this.viewport) return;
		this.viewport.removeEventListener(
			"mousemove",
			this.listenerWrappers.mouseMove,
		);
		this.viewport.removeEventListener(
			"mousedown",
			this.listenerWrappers.mouseDown,
		);
		this.viewport.removeEventListener("mouseup", this.listenerWrappers.mouseUp);
		this.viewport = null;
	}

	public static screenToWorld(event: MouseEvent): Vector2 {
		const viewport = event.target as HTMLCanvasElement;

		return {
			x: event.clientX - viewport.offsetWidth / 2,
			y: event.clientY - viewport.offsetHeight / 2,
		};
	}
}

export default Input;
