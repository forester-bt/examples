use std::path::PathBuf;
use forester_rs::runtime::builder::ForesterBuilder;

pub fn current_folder() -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("src/");
    root
}

fn main() {
    let mut fb = ForesterBuilder::from_fs();
    fb.main_file("robot_example.tree".to_string());
    fb.root(current_folder())

}


// A  <--100--> B where A = 0 and B = 100
// 3 points => robot point and initial target point and final target point.
// the goal is to take the load from initial target point into final target point
enum Command {
    Left,
    Right,
    Lift,
    Drop,
}

enum State {
    Wait,
    Crash,
}
