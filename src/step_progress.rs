//! Defines wrapper for a progress bar which can only step forward.

const HISTORY_DURATION: u64 = 10_000; // in milliseconds

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::progress::{Progress, WithProgress};
use crate::utils::convert_to_unit;

/// A wrapper for a progress bar which can only step forward.
#[derive(Clone, Debug)]
pub struct StepProgress {
    cur_step: usize,
    max_step: Option<usize>,
    progress: Progress,
    time_start: Instant,
    time_history: VecDeque<(Instant, usize)>,
    unit: String,
}

impl StepProgress {
    pub fn new() -> Self {
        Self {
            cur_step: 0,
            max_step: None,
            progress: Progress::new(),
            time_start: Instant::now(),
            time_history: vec![(Instant::now(), 0)].into(),
            unit: "i".to_string(),
        }
    }

    /// Specify a progress bar to use, which allows to copy configuration.
    pub fn with_progress(mut self, progress: Progress) -> Self {
        self.progress = progress;
        self
    }

    /// Update expected max step.
    pub fn with_max_step(mut self, max_step: usize) -> Self {
        self.max_step = Some(max_step);
        self
    }

    /// Update expected max step.
    pub fn set_max_step(&mut self, max_step: usize) {
        self.max_step = Some(max_step)
    }

    /// Get expected max step.
    pub fn max_step(&self) -> Option<usize> {
        self.max_step
    }

    /// Get current step.
    pub fn cur_step(&self) -> usize {
        self.cur_step
    }

    /// Compute the current average speed of iterations.
    pub fn speed(&self) -> f32 {
        let (old_time, old_iter) = *self.time_history.front().unwrap();
        let (cur_time, cur_iter) = (Instant::now(), self.cur_step);
        (cur_iter - old_iter) as f32 / (cur_time - old_time).as_secs_f32()
    }

    /// Compute the total average speed of iterations.
    pub fn total_speed(&self) -> f32 {
        self.cur_step as f32 / self.time_start.elapsed().as_secs_f32()
    }

    // TODO: documentation
    pub fn step(&mut self, count: usize) {
        self.cur_step += count;

        if self.progress.need_refresh() {
            self.draw(false);
        }
    }

    // TODO: documentation
    pub fn finish(&mut self) {
        self.draw(true);
    }

    fn draw(&mut self, finished: bool) {
        self.time_history
            .push_back((Instant::now(), self.cur_step + 1));

        let nb_steps = self.max_step.unwrap_or(self.cur_step);
        let duration = {
            if finished {
                self.time_start.elapsed()
            } else {
                Duration::from_secs_f64(
                    (nb_steps - self.cur_step) as f64 / (1. + self.cur_step as f64)
                        * self.time_start.elapsed().as_secs_f64(),
                )
            }
        };
        let speed = {
            if finished {
                self.total_speed()
            } else {
                self.speed()
            }
        };
        let (speed, unit_prefix) = convert_to_unit(speed);

        self.progress.set_extra_infos(format!(
            "{}/{}, {:.1?} ({:.1} {}{}/s) ",
            self.cur_step, nb_steps, duration, speed, unit_prefix, self.unit
        ));

        if finished {
            self.progress.finished().ok();
        } else {
            self.progress
                .update(self.cur_step as f32 / nb_steps as f32)
                .ok();
        }

        // Trim history to get a window of size ~10s
        while self.time_history.back().unwrap().0 - self.time_history.front().unwrap().0
            > Duration::from_millis(HISTORY_DURATION)
        {
            self.time_history.pop_front();
        }
    }
}

impl Default for StepProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl WithProgress for StepProgress {
    fn update_progress<U>(mut self, update: U) -> Self
    where
        U: FnOnce(Progress) -> Progress,
    {
        self.progress = update(self.progress);
        self
    }
}
