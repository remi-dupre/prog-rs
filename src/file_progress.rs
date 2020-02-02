use crate::progress::{Progress, WithProgress};
use crate::step_progress::StepProgress;

use std::convert::TryInto;
use std::fs::File;
use std::io;

#[derive(Debug)]
pub struct FileProgress {
    inner: File,
    step_progress: StepProgress,
}

impl FileProgress {
    fn new(inner: File) -> Self {
        let max_step = inner.metadata().map_or(0, |m| m.len());

        Self {
            inner,
            step_progress: StepProgress::new()
                .with_humanize(true)
                .with_unit("B")
                .with_max_step(max_step.try_into().expect("file size doesn't fit in usize")),
        }
    }
}

impl io::Seek for FileProgress {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let res = self.inner.seek(pos);

        if let Ok(new_pos) = res {
            let cur_step: u64 = self
                .step_progress
                .cur_step()
                .try_into()
                .expect("file size doesn't fit in usize");

            if new_pos > cur_step {
                self.step_progress.step((new_pos - cur_step) as usize);
            }
        }

        res
    }
}

impl io::Read for FileProgress {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let res = self.inner.read(buf);

        if let Ok(step) = res {
            if step == 0 {
                self.step_progress.finish();
            } else {
                self.step_progress.step(step);
            }
        }

        res
    }
}

impl WithProgress for FileProgress {
    fn get_progress(&mut self) -> &mut Progress {
        self.step_progress.get_progress()
    }
}

//  _____                      _____          _ _
// |  ___| __ ___  _ __ ___   |_   _| __ __ _(_) |_
// | |_ | '__/ _ \| '_ ` _ \    | || '__/ _` | | __|
// |  _|| | | (_) | | | | | |   | || | | (_| | | |_
// |_|  |_|  \___/|_| |_| |_|   |_||_|  \__,_|_|\__|
//

pub trait AsFileProgress {
    fn progress(self) -> FileProgress;
}

impl AsFileProgress for File {
    fn progress(self) -> FileProgress {
        FileProgress::new(self)
    }
}
