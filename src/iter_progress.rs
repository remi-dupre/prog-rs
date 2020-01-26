use std::time::{Duration, Instant};

use crate::progress::{BarPosition, OutputStream, Progress};
use crate::utils::convert_to_unit;

pub struct IterProgress<I, E>
where
    I: Iterator<Item = E>,
{
    inner:      I,
    iter_count: usize,
    iter_size:  Option<usize>,
    progress:   Progress,
    time_start: Instant,
}

impl<'a, I, E> IterProgress<I, E>
where
    I: Iterator<Item = E>,
{
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            iter_count: 0,
            iter_size: None,
            progress: Progress::default(),
            time_start: Instant::now(),
        }
    }

    pub fn with_progress(mut self, progress: Progress) -> Self {
        self.progress = progress;
        self
    }

    pub fn with_bar_position(mut self, bar_position: BarPosition) -> Self {
        self.progress = self.progress.with_bar_position(bar_position);
        self
    }

    pub fn with_bar_width(mut self, bar_width: usize) -> Self {
        self.progress = self.progress.with_bar_width(bar_width);
        self
    }

    pub fn with_display_width(mut self, display_width: usize) -> Self {
        self.progress = self.progress.with_display_width(display_width);
        self
    }

    pub fn with_extra_infos(mut self, extra_infos: String) -> Self {
        self.progress = self.progress.with_extra_infos(extra_infos);
        self
    }

    pub fn with_output_stream(mut self, output_stream: OutputStream) -> Self {
        self.progress = self.progress.with_output_stream(output_stream);
        self
    }

    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.progress = self.progress.with_prefix(prefix);
        self
    }

    pub fn with_refresh_delay(mut self, refresh_delay: Duration) -> Self {
        self.progress = self.progress.with_refresh_delay(refresh_delay);
        self
    }

    pub fn with_shape_body(mut self, shape_body: char) -> Self {
        self.progress = self.progress.with_shape_body(shape_body);
        self
    }

    pub fn with_shape_head(mut self, shape_head: char) -> Self {
        self.progress = self.progress.with_shape_head(shape_head);
        self
    }

    pub fn with_shape_void(mut self, shape_void: char) -> Self {
        self.progress = self.progress.with_shape_void(shape_void);
        self
    }

    pub fn speed(&self) -> f32 {
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
        let iter_size = self.iter_size.unwrap_or_else(|| {
            self.iter_count
                + self.inner.size_hint().0
                + usize::from(ret.is_some())
        });

        if self.progress.need_refresh() || ret.is_none() {
            let remaining = match ret {
                Some(_) => Duration::from_secs_f32(
                    (iter_size - self.iter_count) as f32
                        / (1. + self.iter_count as f32)
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
