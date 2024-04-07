use std::f64::consts::PI;
use forester_rs::runtime::action::Impl;
use forester_webots::wb_robot_init;
use crate::robot::{randdouble, Robot};


mod robot;

fn main() {
    println!("Rust controller of the iRobot Create robot started");
    println!("The robot is {}", if wb_robot_init() > 0 { "ready" } else { "not ready" });

    let mut controller = Robot::default();
    controller.init_devices();
    controller.led_on();
    controller.wait(0.5f64);

    controller.turn(randdouble() * PI);

    loop {
        if controller.is_there_a_virtual_wall() {
            println!("Virtual wall detected! Turning...");
            controller.turn(PI)
        } else if controller.is_there_a_collision_at_left()
            || controller.is_there_a_cliff_at_left() {
            println!("The obstacle at left is detected! Turning...");
            controller.go_backward();
            controller.wait(0.5f64);
            controller.turn(randdouble() * PI);
        }
        else if controller.is_there_a_cliff_at_right()
            || controller.is_there_a_cliff_at_front()
            || controller.is_there_a_collision_at_right(){
            println!("The obstacle at right is detected! Turning...");
            controller.go_backward();
            controller.wait(0.5f64);
            controller.turn(-randdouble() * PI);
        }
        else {
            controller.go_forward();
        }
        controller.flush_ir_receiver();
        controller.step();
    }
}
