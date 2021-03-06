//! Defines a wrapper around iterators to display a progress bar.

use crate::step_progress::{StepProgress, WithStepProgress};

/// A wrapper iterator arround another iterator which adds a progress bar.
///
/// # Example
///
/// ```
/// use prog_rs::prelude::*;
///
/// for _ in (0..1_000)
///     .progress()
///     .with_prefix("Processing...")
///     .with_output_stream(prog_rs::OutputStream::StdErr)
///     .with_bar_position(prog_rs::BarPosition::Right)
/// {
///     do_something();
/// }
/// ```
#[derive(Clone, Debug)]
pub struct IterProgress<I, E>
where
    I: Iterator<Item = E>,
{
    inner: I,
    step_progress: StepProgress,
}

impl<I, E> IterProgress<I, E>
where
    I: Iterator<Item = E>,
{
    fn new(inner: I) -> Self {
        let (max_step, _) = inner.size_hint();

        Self {
            inner,
            step_progress: StepProgress::new().with_max_step(max_step),
        }
    }
}

impl<I, E> Iterator for IterProgress<I, E>
where
    I: Iterator<Item = E>,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inner.next();

        match item {
            None => self.step_progress.finish(),
            Some(_) => {
                let new_max_step = self.step_progress.cur_step() + self.inner.size_hint().0 + 1;
                self.step_progress.set_max_step(new_max_step);
                self.step_progress.step(1)
            }
        }

        item
    }
}

impl<I, E> WithStepProgress for IterProgress<I, E>
where
    I: Iterator<Item = E>,
{
    fn get_step_progress(&mut self) -> &mut StepProgress {
        &mut self.step_progress
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
