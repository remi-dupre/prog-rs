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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BarPosition {
    Right,
    Left,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputStream {
    StdOut,
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
pub struct ProgressConfig {
    bar_position:  BarPosition,
    bar_width:     usize,
    display_width: Option<usize>,
    extra_infos:   String,
    output_stream: OutputStream,
    prefix:        String,
    refresh_delay: Duration,
    shape_body:    char,
    shape_head:    char,
    shape_void:    char,
}

impl Default for ProgressConfig {
    fn default() -> Self {
        Self {
            bar_position:  BarPosition::Left,
            bar_width:     40,
            display_width: None,
            extra_infos:   String::new(),
            output_stream: OutputStream::StdOut,
            prefix:        String::new(),
            refresh_delay: Duration::from_millis(200),
            shape_body:    '=',
            shape_head:    '>',
            shape_void:    ' ',
        }
    }
}

//  ____
// |  _ \ _ __ ___   __ _ _ __ ___  ___ ___
// | |_) | '__/ _ \ / _` | '__/ _ \/ __/ __|
// |  __/| | | (_) | (_| | | |  __/\__ \__ \
// |_|   |_|  \___/ \__, |_|  \___||___/___/
//                  |___/

#[derive(Clone, Debug)]
pub struct Progress {
    config:           ProgressConfig,
    last_update_time: Option<Instant>,
}

impl<'a> Default for Progress {
    fn default() -> Self {
        Self {
            config:           ProgressConfig::default(),
            last_update_time: None,
        }
    }
}

impl<'a> Progress {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_bar_position(mut self, bar_position: BarPosition) -> Self {
        self.config.bar_position = bar_position;
        self
    }

    pub fn with_bar_width(mut self, bar_width: usize) -> Self {
        self.config.bar_width = bar_width;
        self
    }

    pub fn with_display_width(mut self, display_width: usize) -> Self {
        self.config.display_width = Some(display_width);
        self
    }

    pub fn with_extra_infos(mut self, extra_infos: String) -> Self {
        self.config.extra_infos = extra_infos;
        self
    }

    pub fn with_output_stream(mut self, output_stream: OutputStream) -> Self {
        self.config.output_stream = output_stream;
        self
    }

    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.config.prefix = prefix;
        self
    }

    pub fn with_refresh_delay(mut self, refresh_delay: Duration) -> Self {
        self.config.refresh_delay = refresh_delay;
        self
    }

    pub fn with_shape_body(mut self, shape_body: char) -> Self {
        self.config.shape_body = shape_body;
        self
    }

    pub fn with_shape_head(mut self, shape_head: char) -> Self {
        self.config.shape_head = shape_head;
        self
    }

    pub fn with_shape_void(mut self, shape_void: char) -> Self {
        self.config.shape_void = shape_void;
        self
    }

    pub fn set_extra_infos(&mut self, extra_infos: String) {
        self.config.extra_infos = extra_infos
    }

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
        let mut void_length = self.config.bar_width - body_length + 1;
        let mut head_length = 0;

        if void_length > 0 {
            void_length -= 1;
            head_length += 1;
        }

        (body_length, void_length, head_length)
    }

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
        let required_width = self.config.bar_width
            + self.config.prefix.len()
            + self.config.extra_infos.len()
            + 13;
        let display_width = self.config.display_width.unwrap_or_else(|| {
            term_size::dimensions_stdout().map(|(w, _)| w).unwrap_or(80)
        });

        let (prefix, padding) = {
            if display_width >= required_width {
                (
                    &self.config.prefix[..],
                    " ".repeat(display_width - required_width),
                )
            } else if self.config.prefix.len()
                >= required_width - display_width
            {
                let prefix_len = self.config.prefix.len()
                    - (required_width - display_width);
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

    pub fn finished(&mut self) -> io::Result<()> {
        self.last_update_time = None;
        self.update(1.0)?;
        writeln!(&mut self.config.output_stream.get())
    }
}
