extern crate prog_rs;

use std::thread::sleep;
use std::time::Duration;

use prog_rs::prelude::*;
use prog_rs::{BarPosition, OutputStream, Progress};

fn main() {
    for _ in (0..100_000_000)
        .progress()
        .with_prefix("  Starting ...")
        .with_bar_position(BarPosition::Right)
    {}

    for _ in (0..100_000_000)
        .progress()
        .with_prefix(" Continues ...")
        .with_output_stream(OutputStream::StdErr)
        .with_bar_position(BarPosition::Right)
    {}

    for _ in (0..100_000_000)
        .progress()
        .with_prefix("      Ends ...")
        .with_bar_position(BarPosition::Right)
    {}

    let mut progress = Progress::new()
        .with_bar_width(30)
        .with_extra_infos("Hello, World!")
        .with_refresh_delay(Duration::from_millis(100))
        .with_output_stream(OutputStream::StdErr);

    for i in 0..10_000 {
        progress.update(i as f32 / 10_000.).unwrap();
        progress = progress
            .with_extra_infos(format!("Hello, World! ({}/10000)", i + 1));
        sleep(Duration::from_nanos(110));
    }

    progress.finished().ok();
}
