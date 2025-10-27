use engine::Game;

fn main() {
    let width: usize = 800;
    let height: usize = 600;

    let mut game = Game::new(width, height);
    game.run();
}
