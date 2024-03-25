mod bt;

use std::path::PathBuf;
use forester_rs::runtime::action::{Impl, Tick};
use forester_rs::runtime::args::{RtArgs, RtValue};
use forester_rs::runtime::builder::ForesterBuilder;
use forester_rs::runtime::context::TreeContextRef;
use forester_rs::runtime::{RuntimeError, TickResult, to_fail};
use forester_rs::tracer::{Tracer, TracerConfig};
use crate::bt::{ArrivedCheck, DefineDir, Move, Pick, Place, prepare_fb, tracer};

const GRID_SIZE: (i16, i16) = (100, 1);
// Now we define the pixel size of each tile, which we make 32x32 pixels.
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

// Next we define how large we want our actual window to be by multiplying
// the components of our grid size by its corresponding pixel size.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

// Here we're defining how often we want our game to update. This will be
// important later so that we don't have our snake fly across the screen because
// it's moving a full tile every frame.
const DESIRED_FPS: u32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
}


fn main() {
    let mut fb = prepare_fb();

    fb.register_sync_action("pick", Pick);
    fb.register_sync_action("place", Place);
    fb.register_sync_action("move", Move);
    fb.register_sync_action("is_arrived", ArrivedCheck);
    fb.register_sync_action("define_direction", DefineDir);

    fb.tracer(tracer());
    let mut forester = fb.build().unwrap();
    let res = forester.run();

    println!("{:?}", res);
}




