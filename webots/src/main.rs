use forester_rs::runtime::action::Impl;
use forester_webots::wb_robot_init;
use crate::robot::Robot;


mod robot;

fn main() {
    println!("Rust controller of the iRobot Create robot started");
    println!("Init robot: {}", wb_robot_init());

    let mut controller = Robot::default();
    controller.init_devices();
    controller.wait(0.5f64);
    loop {
        println!("Robot goes forward");
        controller.go_forward();
        controller.step();
    }
}
