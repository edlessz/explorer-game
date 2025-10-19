import { useEffect, useRef } from "react";
import "./App.css";
import Game from "./game/Game";

const App: React.FC = () => {
	const game = useRef<Game>(new Game());
	const viewportRef = useRef<HTMLCanvasElement | null>(null);

	useEffect(() => {
		game.current.setViewport(viewportRef.current);
		return () => {
			game.current.setViewport(null);
			game.current.start();
		};
	}, []);

	return (
		<div className="App">
			<canvas ref={viewportRef} className="viewport"></canvas>
		</div>
	);
};

export default App;
