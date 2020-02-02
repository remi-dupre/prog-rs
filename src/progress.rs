//! Defines a basic progress bar that needs to be manually updated.

use std::boxed::Box;
use std::cmp::min;
use std::io;
use std::io::prelude::*;
use std::time::{Duration, Instant};

//   ____             __ _
//  / ___|___  _ __  / _(_) __ _
// | |   / _ \| '_ \| |_| |/ _` |
// | |__| (_) | | | |  _| | (_| |
//  \____\___/|_| |_|_| |_|\__, |
//                         |___/

/// Different display modes for the progress.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BarPosition {
    /// The progress bar will be floating right, with extra informations on
    /// left of it.
    Right,

    /// The progress bar will be displayed on left, just after the prefix.
    Left,
}

/// Available streams to display in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputStream {
    /// Standart output.
    StdOut,

    /// Standart Error.
    StdErr,
}

impl OutputStream {
    fn get(self) -> Box<dyn Write> {
        use OutputStream::*;
        match self {
            StdOut => Box::new(io::stdout()),
            StdErr => Box::new(io::stderr()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProgressConfig {
    bar_position: BarPosition,
    bar_width: usize,
    display_width: Option<usize>,
    extra_infos: String,
    output_stream: OutputStream,
    prefix: String,
    refresh_delay: Duration,
    shape_body: char,
    shape_head: char,
    shape_void: char,
}

impl Default for ProgressConfig {
    fn default() -> Self {
        Self {
            bar_position: BarPosition::Left,
            bar_width: 40,
            display_width: None,
            extra_infos: String::new(),
            output_stream: OutputStream::StdOut,
            prefix: String::new(),
            refresh_delay: Duration::from_millis(200),
            shape_body: '=',
            shape_head: '>',
            shape_void: ' ',
        }
    }
}

//  ____
// |  _ \ _ __ ___   __ _ _ __ ___  ___ ___
// | |_) | '__/ _ \ / _` | '__/ _ \/ __/ __|
// |  __/| | | (_) | (_| | | |  __/\__ \__ \
// |_|   |_|  \___/ \__, |_|  \___||___/___/
//                  |___/

/// A generic progress bar that needs to be manually updated.
///
/// # Example
///
/// ```
/// let mut progress = Progress::new()
///     .with_bar_width(30)
///     .with_extra_infos("Hello, World!")
///     .with_refresh_delay(Duration::from_millis(100))
///     .with_output_stream(OutputStream::StdErr);
///
/// for i in 0..10_000 {
///     progress.update(i as f32 / 10_000.).unwrap();
///     progress = progress
///         .with_extra_infos(format!("Hello, World! ({}/10000)", i + 1));
///     sleep(Duration::from_nanos(110));
/// }
///
/// progress.finished().ok();
/// ```
#[derive(Clone, Debug)]
pub struct Progress {
    config: ProgressConfig,
    last_update_time: Option<Instant>,
}

impl<'a> Default for Progress {
    fn default() -> Self {
        Self {
            config: ProgressConfig::default(),
            last_update_time: None,
        }
    }
}

impl<'a> Progress {
    /// Create a new progress bar with default display settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Update extra informations displayed next to the progress bar.
    pub fn set_extra_infos<S>(&mut self, extra_infos: S)
    where
        S: Into<String>,
    {
        self.config.extra_infos = extra_infos.into()
    }

    /// Check if the timer specified by `with_refresh_delay` has decayed.
    pub fn need_refresh(&self) -> bool {
        if let Some(last_update_time) = self.last_update_time {
            return last_update_time.elapsed() >= self.config.refresh_delay;
        }
        true
    }

    fn bar_shape(&self, progress: f32) -> (usize, usize, usize) {
        let body_length = min(
            self.config.bar_width + 1,
            (progress * (self.config.bar_width + 1) as f32).round() as usize,
        );
        let mut void_length = (self.config.bar_width + 1) - body_length;
        let mut head_length = 0;

        if void_length > 0 {
            void_length -= 1;
            head_length += 1;
        }

        (body_length, void_length, head_length)
    }

    /// Redraw the progress bar if the timer has decayed.
    pub fn update(&mut self, progress: f32) -> io::Result<()> {
        if !self.need_refresh() {
            return Ok(());
        }

        self.last_update_time = Some(Instant::now());

        let (body, void, head) = self.bar_shape(progress);
        let body = self.config.shape_body.to_string().repeat(body);
        let head = self.config.shape_head.to_string().repeat(head);
        let void = self.config.shape_void.to_string().repeat(void);

        // Compute display shape
        let required_width =
            self.config.bar_width + self.config.prefix.len() + self.config.extra_infos.len() + 13;
        let display_width = self
            .config
            .display_width
            .unwrap_or_else(|| term_size::dimensions_stdout().map(|(w, _)| w).unwrap_or(80));

        let (prefix, padding) = {
            if display_width >= required_width {
                (
                    &self.config.prefix[..],
                    " ".repeat(display_width - required_width),
                )
            } else if self.config.prefix.len() >= required_width - display_width {
                let prefix_len = self.config.prefix.len() - (required_width - display_width);
                (&self.config.prefix[0..prefix_len], String::new())
            } else {
                ("", String::new())
            }
        };

        let text = match self.config.bar_position {
            BarPosition::Left => format!(
                "\r{} {:>5.1}% [{}{}{}] {}{}",
                prefix,
                100. * progress,
                body,
                head,
                void,
                self.config.extra_infos,
                padding
            ),
            BarPosition::Right => format!(
                "\r{} {}{} [{}{}{}] {:>5.1}%",
                prefix,
                padding,
                self.config.extra_infos,
                body,
                head,
                void,
                100. * progress
            ),
        };

        // Display text
        let mut stream = self.config.output_stream.get();
        stream.write_all(&text.as_bytes())?;
        stream.flush()
    }

    /// Redraw the progress bar for the last time.
    pub fn finished(&mut self) -> io::Result<()> {
        self.last_update_time = None;
        self.update(1.0)?;
        writeln!(&mut self.config.output_stream.get())
    }
}

/// A type that contains a progress bar that can be updated using `with_{parameter}` syntax.
pub trait WithProgress: Sized {
    fn update_progress<U>(self, update: U) -> Self
    where
        U: FnOnce(Progress) -> Progress;

    /// Change the style of the bar disposition.
    fn with_bar_position(self, bar_position: BarPosition) -> Self {
        self.update_progress(move |mut p| {
            p.config.bar_position = bar_position;
            p
        })
    }

    /// Change the width of the progress bar.
    fn with_bar_width(self, bar_width: usize) -> Self {
        self.update_progress(move |mut p| {
            p.config.bar_width = bar_width;
            p
        })
    }

    /// Change the width of the text the displayed informations should try to
    /// fit in. The terminal width will be detected by default.
    fn with_display_width(self, display_width: usize) -> Self {
        self.update_progress(move |mut p| {
            p.config.display_width = Some(display_width);
            p
        })
    }

    /// Specify extra informations to display.
    fn with_extra_infos<S>(self, extra_infos: S) -> Self
    where
        S: Into<String>,
    {
        self.update_progress(move |mut p| {
            p.config.extra_infos = extra_infos.into();
            p
        })
    }

    /// Change the output stream the progress bar is displayed in. By default
    /// standart output is used.
    fn with_output_stream(self, output_stream: OutputStream) -> Self {
        self.update_progress(move |mut p| {
            p.config.output_stream = output_stream;
            p
        })
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
    fn with_prefix<S>(self, prefix: S) -> Self
    where
        S: Into<String>,
    {
        self.update_progress(move |mut p| {
            p.config.prefix = prefix.into();
            p
        })
    }

    /// Change the minimum delay between two display updates.
    fn with_refresh_delay(self, refresh_delay: Duration) -> Self {
        self.update_progress(move |mut p| {
            p.config.refresh_delay = refresh_delay;
            p
        })
    }

    /// Change the character used to draw the body of the progress bar.
    fn with_shape_body(self, shape_body: char) -> Self {
        self.update_progress(move |mut p| {
            p.config.shape_body = shape_body;
            p
        })
    }

    /// Change the character used to draw the head of the progress bar.
    fn with_shape_head(self, shape_head: char) -> Self {
        self.update_progress(move |mut p| {
            p.config.shape_head = shape_head;
            p
        })
    }

    /// Change the character used to draw the background of the progress bar.
    fn with_shape_void(self, shape_void: char) -> Self {
        self.update_progress(move |mut p| {
            p.config.shape_void = shape_void;
            p
        })
    }
}

impl WithProgress for Progress {
    fn update_progress<U>(self, update: U) -> Self
    where
        U: FnOnce(Progress) -> Progress,
    {
        update(self)
    }
}
