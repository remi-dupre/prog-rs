//! prog_rs
//! =======
//!
//! A Rust library to help easily build a progress bar.
//!
//!
//! Basic usage
//! -----------
//!
//! To add a progress bar while iterating, just add `.progress()` behind your
//! iterator:
//!
//! ```
//! use prog_rs::prelude::*;
//!
//! for _ in (0..1_000).progress() {
//!     std::thread::sleep(std::time::Duration::from_millis(5));
//! }
//! ```
//!
//! Some parameters can be tweaked using `with_` prefixed methods:
//!
//! ```
//! use prog_rs::prelude::*;
//!
//! for _ in (0..1_000)
//!     .progress()
//!     .with_prefix("Processing...")
//!     .with_output_stream(prog_rs::OutputStream::StdErr)
//!     .with_bar_position(prog_rs::BarPosition::Right)
//! {
//!     do_something();
//! }
//! ```
//!
//! This same behaviour is also implemented for files:
//!
//! ```
//! use prog_rs::prelude::*;
//!
//! let f = File::open("../../data/addresses/bano.csv")
//!     .unwrap()
//!     .progress()
//!     .with_prefix(" Read file ...")
//!     .with_bar_position(BarPosition::Right);
//! let f = BufReader::new(f);
//! println!("This file has {} lines", f.lines().count());
//! ```
//!
//!
//! Advanced usage
//! --------------
//!
//! It is possible to control how the progress bar behaves by controlling it
//! outside of the iterators wrapper:
//!
//! ```
//! let mut progress = Progress::new()
//!     .with_bar_width(30)
//!     .with_extra_infos("Hello, World!")
//!     .with_refresh_delay(Duration::from_millis(100))
//!     .with_output_stream(OutputStream::StdErr);
//!
//! for i in 0..10_000 {
//!     progress.update(i as f32 / 10_000.).unwrap();
//!     progress = progress
//!         .with_extra_infos(format!("Hello, World! ({}/10000)", i + 1));
//!     sleep(Duration::from_nanos(110));
//! }
//!
//! progress.finished().ok();
//! ```
//!
//!
//! Performances
//! ------------
//!
//! The progress bar redraw rate is restricted to avoid making a huge number of
//! I/O and avoid loosing too much CPU time.
//!
//! On most uses, it won't affect performances, but avoid using it if you use
//! an iterator very intensively (more than about 100M iterations/s).
//!
//!
//! Implementation details
//! ----------------------
//!
//! #### How is the remaining number of iterations computed?
//!
//! The remaining number of iterations is computed by using `size_hint` by
//! default, if you want to specify a more accurate value, you can use
//! `with_iter_size`.

extern crate term_size;

mod utils;

pub mod file_progress;
pub mod iter_progress;
pub mod prelude;
pub mod progress;
pub mod step_progress;

pub use file_progress::*;
pub use iter_progress::*;
pub use progress::*;
pub use step_progress::*;
