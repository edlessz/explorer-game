import { useEffect, useRef } from "react";
import "./App.css";
import Camera from "./game/components/Camera";
import ColorRenderer from "./game/components/ColorRenderer";
import Game from "./game/engine/Game";

const game = new Game();

const box = game.addEntity();
const colorRenderer = box.addComponent(ColorRenderer);
colorRenderer.color = "#f00";

const camera = game.addEntity();
camera.addComponent(Camera);
game.setCamera(camera);

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
