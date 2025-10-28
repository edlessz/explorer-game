use engine::Game;
use game_desktop::DesktopPlatform;

fn main() {
    let width: usize = 800;
    let height: usize = 600;

    let mut game = Game::<DesktopPlatform>::new(width, height, "Explorer Game");
    game.run();
}
