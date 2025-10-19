import { useEffect, useRef } from "react";
import "./App.css";
import CameraController from "./game/components/CameraController";
import PlayerController from "./game/components/PlayerController";
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

const tileMap2 = game.addEntity();
tileMap2.addComponent(TileMap);
tileMap2.transform.position.x = 5.5;
tileMap2.transform.position.y = 1;

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
