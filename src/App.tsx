import { useEffect, useRef } from "react";
import "./App.css";
import CameraController from "./game/components/CameraController";
import PlayerController from "./game/components/PlayerController";
import WorldGenerator from "./game/components/WorldGenerator";
import Camera from "./game/engine/components/Camera";
import ColorRenderer from "./game/engine/components/ColorRenderer";
import Physics from "./game/engine/components/Physics";
import TileMap from "./game/engine/components/TileMap";
import TileMapCollider from "./game/engine/components/TileMapCollider";
import Game from "./game/engine/Game";

const game = new Game();

const player = game.addEntity();
player.addComponent(ColorRenderer).color = "#f00";
player.addComponent(PlayerController);
player.addComponent(Physics);
player.addComponent(TileMapCollider);
player.tag = "player";

const camera = game.addEntity();
camera.addComponent(Camera);
camera.addComponent(CameraController);
camera.tag = "camera";

const tileMap = game.addEntity();
tileMap.addComponent(TileMap);
tileMap.addComponent(WorldGenerator);

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
