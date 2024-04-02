mod bt;

use std::path::PathBuf;
use forester_rs::runtime::action::{Impl, Tick};
use forester_rs::runtime::args::{RtArgs, RtValue};
use forester_rs::runtime::builder::ForesterBuilder;
use forester_rs::runtime::context::TreeContextRef;
use forester_rs::runtime::{RuntimeError, TickResult, to_fail};
use forester_rs::tracer::{Tracer, TracerConfig};

// let mut fb = prepare_fb();
//
// fb.register_sync_action("pick", Pick);
// fb.register_sync_action("place", Place);
// fb.register_sync_action("move", Move);
// fb.register_sync_action("is_arrived", ArrivedCheck);
// fb.register_sync_action("define_direction", DefineDir);
//
// fb.tracer(tracer());
// let mut forester = fb.build().unwrap();
// let res = forester.run();
//
// println!("{:?}", res);

use bevy::prelude::*;


fn main() {}