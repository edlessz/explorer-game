use engine::Game;
use game_desktop::MiniFbRenderer;

fn main() {
    let width: usize = 800;
    let height: usize = 600;

    let mut game = Game::<MiniFbRenderer>::new(width, height, "Explorer Game");
    game.run();
}
