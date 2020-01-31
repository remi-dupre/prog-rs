//! Defines a wrapper arroud iterators to display a progress bar.
//!
//! # Example
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

const HISTORY_DURATION: u64 = 10_000; // in milliseconds

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::progress::{BarPosition, OutputStream, Progress};
use crate::utils::convert_to_unit;

/// A wrapper iterator arround another iterator which adds a progress bar.
#[derive(Clone, Debug)]
pub struct IterProgress<I, E>
where
    I: Iterator<Item = E>,
{
    inner: I,
    iter_count: usize,
    iter_size: Option<usize>,
    progress: Progress,
    time_start: Instant,
    time_history: VecDeque<(Instant, usize)>,
}

impl<'a, I, E> IterProgress<I, E>
where
    I: Iterator<Item = E>,
{
    fn new(inner: I) -> Self {
        Self {
            inner,
            iter_count: 0,
            iter_size: None,
            progress: Progress::default(),
            time_start: Instant::now(),
            time_history: vec![(Instant::now(), 0)].into(),
        }
    }

    /// Specify a progress bar to use, which allows to copy configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use prog_rs::prelude::*;
    /// use prog_rs::{OutputStream, Progress};
    ///
    /// let progress = Progress::new()
    ///     .with_bar_width(50)
    ///     .with_output_stream(OutputStream::StdErr);
    ///
    /// for i in (0..100).progress().with_progress(progress.clone()) {
    ///     do_something(i);
    /// }
    ///
    /// for i in (0..100).progress().with_progress(progress) {
    ///     do_something(i);
    /// }
    /// ```
    pub fn with_progress(mut self, progress: Progress) -> Self {
        self.progress = progress;
        self
    }

    /// Specify the expected size of inner iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use prog_rs::prelude::*;
    ///
    /// for i in (0..10)
    ///     .filter(|x| x % 2 == 0) // `size_hint` wouldn't be relevant
    ///     .progress()
    ///     .with_iter_size(5)
    /// {
    ///     do_something(i);
    /// }
    /// ```
    pub fn with_iter_size(mut self, iter_size: usize) -> Self {
        self.iter_size = Some(iter_size);
        self
    }

    /// Change the style of the bar disposition.
    pub fn with_bar_position(mut self, bar_position: BarPosition) -> Self {
        self.progress = self.progress.with_bar_position(bar_position);
        self
    }

    /// Change the width of the progress bar.
    pub fn with_bar_width(mut self, bar_width: usize) -> Self {
        self.progress = self.progress.with_bar_width(bar_width);
        self
    }

    /// Change the width of the text the displayed informations should try to
    /// fit in. The terminal width will be detected by default.
    pub fn with_display_width(mut self, display_width: usize) -> Self {
        self.progress = self.progress.with_display_width(display_width);
        self
    }

    /// Specify extra informations to display.
    pub fn with_extra_infos<S>(mut self, extra_infos: S) -> Self
    where
        S: Into<String>,
    {
        self.progress = self.progress.with_extra_infos(extra_infos);
        self
    }

    /// Change the output stream the progress bar is displayed in. By default
    /// standart output is used.
    pub fn with_output_stream(mut self, output_stream: OutputStream) -> Self {
        self.progress = self.progress.with_output_stream(output_stream);
        self
    }

    /// Change the text displayed in front of progress informations.
    ///
    /// # Example
    ///
    /// ```
    /// use prog_rs::prelude::*;
    ///
    /// for i in (0..1000)
    ///     .progress()
    ///     .with_prefix("Computing something ...")
    /// {
    ///     do_something(i);
    /// }
    pub fn with_prefix<S>(mut self, prefix: S) -> Self
    where
        S: Into<String>,
    {
        self.progress = self.progress.with_prefix(prefix);
        self
    }

    /// Change the minimum delay between two display updates.
    pub fn with_refresh_delay(mut self, refresh_delay: Duration) -> Self {
        self.progress = self.progress.with_refresh_delay(refresh_delay);
        self
    }

    /// Change the character used to draw the body of the progress bar.
    pub fn with_shape_body(mut self, shape_body: char) -> Self {
        self.progress = self.progress.with_shape_body(shape_body);
        self
    }

    /// Change the character used to draw the head of the progress bar.
    pub fn with_shape_head(mut self, shape_head: char) -> Self {
        self.progress = self.progress.with_shape_head(shape_head);
        self
    }

    /// Change the character used to draw the background of the progress bar.
    pub fn with_shape_void(mut self, shape_void: char) -> Self {
        self.progress = self.progress.with_shape_void(shape_void);
        self
    }

    /// Compute the current average speed of iterations.
    pub fn speed(&self) -> f32 {
        let (old_time, old_iter) = *self.time_history.front().unwrap();
        let (cur_time, cur_iter) = (Instant::now(), self.iter_count);
        (cur_iter - old_iter) as f32 / (cur_time - old_time).as_secs_f32()
    }

    /// Compute the total average speed of iterations.
    pub fn total_speed(&self) -> f32 {
        self.iter_count as f32 / self.time_start.elapsed().as_secs_f32()
    }
}

impl<'a, I, E> Iterator for IterProgress<I, E>
where
    I: Iterator<Item = E>,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.inner.next();

        if self.progress.need_refresh() || ret.is_none() {
            self.time_history
                .push_back((Instant::now(), self.iter_count + 1));

            let iter_size = self.iter_size.unwrap_or_else(|| {
                self.iter_count + self.inner.size_hint().0 + usize::from(ret.is_some())
            });

            let remaining = match ret {
                Some(_) => Duration::from_secs_f32(
                    (iter_size - self.iter_count) as f32 / (1. + self.iter_count as f32)
                        * self.time_start.elapsed().as_secs_f32(),
                ),
                None => self.time_start.elapsed(),
            };

            let (speed, unit) = convert_to_unit(self.speed());
            self.progress.set_extra_infos(format!(
                "{}/{}, {:.1?} ({:.1} {}/s) ",
                self.iter_count, iter_size, remaining, speed, unit
            ));

            match ret {
                Some(_) => self
                    .progress
                    .update(self.iter_count as f32 / iter_size as f32),
                None => self.progress.finished(),
            }
            .ok();

            // Trim history to get a window of size ~10s
            while self.time_history.back().unwrap().0 - self.time_history.front().unwrap().0
                > Duration::from_millis(HISTORY_DURATION)
            {
                self.time_history.pop_front();
            }
        }

        self.iter_count += 1;
        ret
    }
}

//  _____                      _____          _ _
// |  ___| __ ___  _ __ ___   |_   _| __ __ _(_) |_
// | |_ | '__/ _ \| '_ ` _ \    | || '__/ _` | | __|
// |  _|| | | (_) | | | | | |   | || | | (_| | | |_
// |_|  |_|  \___/|_| |_| |_|   |_||_|  \__,_|_|\__|
//

pub trait AsProgressIterator<I, E>
where
    I: Iterator<Item = E>,
{
    fn progress(self) -> IterProgress<I, E>;
}

impl<I, E> AsProgressIterator<I, E> for I
where
    I: Iterator<Item = E>,
{
    fn progress(self) -> IterProgress<I, E> {
        IterProgress::new(self)
    }
}
