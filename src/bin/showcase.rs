extern crate rprogress;

use std::io::stderr;
use std::thread::sleep;
use std::time::Duration;

use rprogress::prelude::*;
use rprogress::Progress;

fn main() {
    for _ in (0..100_000_000)
        .progress()
        .with_prefix(" starting ...".to_string())
    {}

    for _ in (0..)
        .take(100_000_000)
        .progress()
        .with_iter_size(0)
        .with_prefix(" starting ...".to_string())
    {}

    let mut progress = Progress::new()
        .with_bar_width(30)
        .with_extra_infos("Hello, World!".to_string())
        .with_refresh_delay(Duration::from_millis(100))
        .with_output_stream(stderr());

    for i in 0..10_000 {
        progress.update(i as f32 / 10_000.);
        progress = progress.with_extra_infos(format!("Hello, World! ({}/10000)", i + 1));
        sleep(Duration::from_nanos(1));
    }

    progress.finished();
}
