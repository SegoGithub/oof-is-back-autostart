#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod crossplatform;

use std::{thread::sleep, time::Duration};

fn main() {
    crossplatform::check_ver();
    loop {
        crossplatform::check_ver();
        sleep(Duration::from_secs(30));
    }
}
