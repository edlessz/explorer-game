import { useEffect, useRef } from "react";
import "./App.css";
import CameraController from "../game/components/CameraController";
import CursorController from "../game/components/CursorController";
import PlayerController from "../game/components/PlayerController";
import WorldGenerator from "../game/components/WorldGenerator";
import scene0 from "../game/data/scene0.json";
import tileRegistry0 from "../game/data/tileRegistry0.json";
import TileRegistry from "../game/engine/components/TileRegistry";
import Game from "../game/engine/Game";

const game = new Game();

game.registerComponent(CameraController);
game.registerComponent(CursorController);
game.registerComponent(PlayerController);
game.registerComponent(WorldGenerator);

game.loadScene(scene0);
const registry =
	game
		.getEntitiesWithComponent(TileRegistry)
		.at(0)
		?.getComponent(TileRegistry) ?? null;
if (registry) registry.registerTiles(tileRegistry0);

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
