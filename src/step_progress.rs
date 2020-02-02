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
    humanize: bool,
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
            humanize: false,
            max_step: None,
            progress: Progress::new(),
            time_start: Instant::now(),
            time_history: vec![(Instant::now(), 0)].into(),
            unit: String::new(),
        }
    }

    /// Change wether units are converted to human-readable units.
    pub fn with_humanize(mut self, humanize: bool) -> Self {
        self.humanize = humanize;
        self
    }

    /// Change displayed unit.
    pub fn with_unit<S: Into<String>>(mut self, unit: S) -> Self {
        self.unit = unit.into();
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

    /// Make progress for `count` iterations and redraw if necessary.
    pub fn step(&mut self, count: usize) {
        self.cur_step += count;

        if self.progress.need_refresh() {
            self.draw(false);
        }
    }

    /// End iterations and redraw.
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

        // Compute speed
        let speed = {
            if finished {
                self.total_speed()
            } else {
                self.speed()
            }
        };

        let (speed, unit_prefix) = convert_to_unit(speed);

        // Compute current state with unit
        let displayed_precision = if self.humanize { 2 } else { 0 };

        let (displayed_cur, displayed_cur_unit) = {
            if self.humanize {
                convert_to_unit(self.cur_step as f32)
            } else {
                (self.cur_step as f32, "")
            }
        };

        let (displayed_max, displayed_max_unit) = {
            if self.humanize {
                convert_to_unit(nb_steps as f32)
            } else {
                (nb_steps as f32, "")
            }
        };

        self.progress.set_extra_infos(format!(
            "{:.precision$}{}{unit}/{:>.precision$}{}{unit}, {:.1?} ({:.1} {}{unit}/s) ",
            displayed_cur,
            displayed_cur_unit,
            displayed_max,
            displayed_max_unit,
            duration,
            speed,
            unit_prefix,
            precision = displayed_precision,
            unit = self.unit
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
    fn get_progress(&mut self) -> &mut Progress {
        &mut self.progress
    }
}
