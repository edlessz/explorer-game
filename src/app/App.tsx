import { useEffect, useRef } from "react";
import "./App.css";
import CameraController from "../game/components/CameraController";
import CursorController from "../game/components/CursorController";
import PlayerController from "../game/components/PlayerController";
import WorldGenerator from "../game/components/WorldGenerator";
import Camera from "../game/engine/components/Camera";
import ColorRenderer from "../game/engine/components/ColorRenderer";
import LightMap from "../game/engine/components/LightMap";
import Physics from "../game/engine/components/Physics";
import TileMap from "../game/engine/components/TileMap";
import TileMapCollider from "../game/engine/components/TileMapCollider";
import TileRegistry from "../game/engine/components/TileRegistry";
import Game from "../game/engine/Game";

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
tileMap.addComponent(LightMap);
const tileRegistry = tileMap.addComponent(TileRegistry);
tileRegistry.registerTile({
	tileId: 1,
	name: "Dirt",
	assetPath: "dirt.png",
	solid: true,
});
tileRegistry.registerTile({
	tileId: 2,
	name: "Grass",
	assetPath: "grass.png",
	solid: true,
});
tileRegistry.registerTile({
	tileId: 3,
	name: "Stone",
	assetPath: "stone.png",
	solid: true,
});
tileRegistry.registerTile({
	tileId: 4,
	name: "Light",
	assetPath: "light.png",
	solid: false,
	lightIntensity: 0.5,
	lightRadius: 10,
	lightColor: "#ffffff",
});
tileRegistry.registerTile({
	tileId: 5,
	name: "Light",
	assetPath: "light.png",
	solid: false,
	lightIntensity: 1,
	lightRadius: 10,
	lightColor: "#0000ff",
});
tileMap.tag = "editableTileMap";

const cursor = game.addEntity();
cursor.addComponent(CursorController);
cursor.addComponent(ColorRenderer).color = "rgba(255, 255, 255, 0.5)";

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
