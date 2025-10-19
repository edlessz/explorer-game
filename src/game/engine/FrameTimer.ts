class FrameTimer {
	private measures: number[] = [];
	private lastTime: number = 0;

	public recordFrame(): number {
		const now = performance.now();
		const fps = 1000 / (now - this.lastTime);
		const deltaTime = (now - this.lastTime) / 1000;
		this.lastTime = now;

		this.measures.push(fps);
		if (this.measures.length > 60) this.measures.shift();

		return deltaTime;
	}

	public getFPS(): number {
		return this.measures.reduce((a, b) => a + b, 0) / this.measures.length;
	}
}

export default FrameTimer;
