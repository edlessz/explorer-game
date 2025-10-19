import { useEffect, useRef } from "react";
import "./App.css";
import PlayerController from "./game/components/PlayerController";
import Camera from "./game/engine/components/Camera";
import ColorRenderer from "./game/engine/components/ColorRenderer";
import TileMap from "./game/engine/components/TileMap";
import Game from "./game/engine/Game";

const game = new Game();

const player = game.addEntity();
player.addComponent(ColorRenderer).color = "#f00";
player.addComponent(PlayerController);

const camera = game.addEntity();
camera.addComponent(Camera);
game.setCamera(camera);

const tileMap = game.addEntity();
tileMap.addComponent(TileMap);

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
