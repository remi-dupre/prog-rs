use std::io::Write;
use std::time::{Duration, Instant};

use crate::progress::Progress;
use crate::utils::convert_to_unit;

pub struct IterProgress<'a, I, E>
where
    I: Iterator<Item = E>,
{
    inner:      I,
    iter_count: usize,
    iter_size:  usize,
    progress:   Progress<'a>,
    time_start: Instant,
}

impl<'a, I, E> IterProgress<'a, I, E>
where
    I: Iterator<Item = E>,
{
    pub fn new(inner: I, iter_size: usize) -> Self {
        Self {
            inner,
            iter_count: 0,
            iter_size,
            progress: Progress::default(),
            time_start: Instant::now(),
        }
    }

    pub fn with_progress(mut self, progress: Progress<'a>) -> Self {
        self.progress = progress;
        self
    }

    pub fn with_bar_width(mut self, bar_width: u16) -> Self {
        self.progress = self.progress.with_bar_width(bar_width);
        self
    }

    pub fn with_extra_infos(mut self, extra_infos: String) -> Self {
        self.progress = self.progress.with_extra_infos(extra_infos);
        self
    }

    pub fn with_output_stream(
        mut self,
        output_stream: impl Write + 'a,
    ) -> Self {
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

impl<'a, I, E> Iterator for IterProgress<'a, I, E>
where
    I: Iterator<Item = E>,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.inner.next();

        if self.progress.need_refresh() || ret.is_none() {
            let remaining = match ret {
                Some(_) => Duration::from_secs_f32(
                    (self.iter_size - self.iter_count) as f32
                        / (1. + self.iter_count as f32)
                        * self.time_start.elapsed().as_secs_f32(),
                ),
                None => self.time_start.elapsed(),
            };

            let (speed, unit) = convert_to_unit(self.speed());
            self.progress.set_extra_infos(format!(
                "{}/{}, {:.1?} ({:.1} {}/s) ",
                self.iter_count, self.iter_size, remaining, speed, unit
            ));

            match ret {
                Some(_) => self
                    .progress
                    .update(self.iter_count as f32 / self.iter_size as f32),
                None => self.progress.finished(),
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

pub trait AsSizedProgressIterator<'a, I, E>
where
    I: Iterator<Item = E>,
{
    fn progress(self) -> IterProgress<'a, I, E>;
}

impl<'a, I, E> AsSizedProgressIterator<'a, I, E> for I
where
    I: ExactSizeIterator<Item = E>,
{
    fn progress(self) -> IterProgress<'a, I, E> {
        let size = self.len();
        IterProgress::new(self, size)
    }
}

pub trait AsProgressIterator<'a, I, E>
where
    I: Iterator<Item = E>,
{
    fn uprogress(self, size: usize) -> IterProgress<'a, I, E>;
}

impl<'a, I, E> AsProgressIterator<'a, I, E> for I
where
    I: Iterator<Item = E>,
{
    fn uprogress(self, size: usize) -> IterProgress<'a, I, E> {
        IterProgress::new(self, size)
    }
}
