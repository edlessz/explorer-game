import Component from "../Component";

class ColorRenderer extends Component {
	public color: string = "#000";

	public render(g: CanvasRenderingContext2D): void {
		const { position, rotation, scale } = this.entity.transform;

		g.save();
		g.translate(position.x, position.y);
		g.rotate(rotation);
		g.scale(scale.x, scale.y);

		g.fillStyle = this.color;
		g.fillRect(-0.5, -0.5, 1, 1);

		g.restore();
	}
}

export default ColorRenderer;
