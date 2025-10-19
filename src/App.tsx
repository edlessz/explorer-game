import { useEffect, useRef } from "react";
import "./App.css";
import ColorRenderer from "./game/components/ColorRenderer";
import Game from "./game/engine/Game";

const game = new Game();
const test = game.addEntity();
const colorRenderer = test.addComponent(ColorRenderer);
colorRenderer.color = "#f00";
game.start();

const App: React.FC = () => {
	const viewportRef = useRef<HTMLCanvasElement | null>(null);

	useEffect(() => {
		game.setViewport(viewportRef.current);
	}, []);

	return (
		<div className="App">
			<canvas ref={viewportRef} className="viewport"></canvas>
		</div>
	);
};

export default App;
