import type { Vector2 } from "./types";

type Scope = "window" | "viewport";

class Input {
	private viewport: HTMLCanvasElement | null = null;
	private keyMap: Record<string, boolean> = {};

	private keyMapOnKeyDown = (e: KeyboardEvent) => {
		this.keyMap[e.key] = true;
	};
	private keyMapOnKeyUp = (e: KeyboardEvent) => {
		this.keyMap[e.key] = false;
	};

	public isKeyPressed(key: string): boolean {
		return !!this.keyMap[key];
	}

	public onMouseMove: ((event: MouseEvent) => void) | null = null;
	public onMouseDown: ((event: MouseEvent) => void) | null = null;
	public onMouseUp: ((event: MouseEvent) => void) | null = null;
	public onKeyDown: ((event: KeyboardEvent) => void) | null = null;
	public onKeyUp: ((event: KeyboardEvent) => void) | null = null;

	private listenerWrappers: {
		[eventName: string]: {
			listener: (event: never) => void;
			scope: Scope;
		};
	} = {
		mousemove: {
			listener: (event: MouseEvent) => this.onMouseMove?.(event),
			scope: "viewport",
		},
		mousedown: {
			listener: (event: MouseEvent) => this.onMouseDown?.(event),
			scope: "viewport",
		},
		mouseup: {
			listener: (event: MouseEvent) => this.onMouseUp?.(event),
			scope: "viewport",
		},
		keydown: {
			listener: (event: KeyboardEvent) => this.onKeyDown?.(event),
			scope: "window",
		},
		keyup: {
			listener: (event: KeyboardEvent) => this.onKeyUp?.(event),
			scope: "window",
		},
	};

	public initialize(viewport: HTMLCanvasElement): void {
		this.viewport = viewport;

		for (const [eventName, handler] of Object.entries(this.listenerWrappers)) {
			const target = handler.scope === "window" ? window : this.viewport;
			target.addEventListener(eventName, handler.listener as EventListener);
		}
		window.addEventListener("keydown", this.keyMapOnKeyDown);
		window.addEventListener("keyup", this.keyMapOnKeyUp);
	}

	public cleanup(): void {
		if (!this.viewport) return;

		for (const [eventName, handler] of Object.entries(this.listenerWrappers)) {
			const target = handler.scope === "window" ? window : this.viewport;
			target.removeEventListener(eventName, handler.listener as EventListener);
		}
		window.removeEventListener("keydown", this.keyMapOnKeyDown);
		window.removeEventListener("keyup", this.keyMapOnKeyUp);

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
