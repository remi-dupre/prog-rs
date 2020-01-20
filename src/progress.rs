use std::boxed::Box;
use std::cmp::min;
use std::io::prelude::*;
use std::io::stdout;
use std::time::{Duration, Instant};

//   ____             __ _
//  / ___|___  _ __  / _(_) __ _
// | |   / _ \| '_ \| |_| |/ _` |
// | |__| (_) | | | |  _| | (_| |
//  \____\___/|_| |_|_| |_|\__, |
//                         |___/

pub struct ProgressConfig<'a> {
    bar_width:     u16,
    extra_infos:   String,
    output_stream: Box<dyn Write + 'a>,
    prefix:        String,
    refresh_delay: Duration,
    shape_body:    char,
    shape_head:    char,
    shape_void:    char,
}

impl<'a> Default for ProgressConfig<'a> {
    fn default() -> Self {
        Self {
            bar_width:     30,
            extra_infos:   String::new(),
            output_stream: Box::new(stdout()),
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

pub struct Progress<'a> {
    config:           ProgressConfig<'a>,
    last_update_time: Option<Instant>,
}

impl<'a> Default for Progress<'a> {
    fn default() -> Self {
        Self {
            config:           ProgressConfig::default(),
            last_update_time: None,
        }
    }
}

impl<'a> Progress<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_bar_width(mut self, bar_width: u16) -> Self {
        self.config.bar_width = bar_width;
        self
    }

    pub fn with_extra_infos(mut self, extra_infos: String) -> Self {
        self.config.extra_infos = extra_infos;
        self
    }

    pub fn with_output_stream(
        mut self,
        output_stream: impl Write + 'a,
    ) -> Self {
        self.config.output_stream = Box::new(output_stream);
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
            usize::from(self.config.bar_width) + 1,
            (progress * (self.config.bar_width + 1) as f32).round() as usize,
        );
        let mut void_length =
            usize::from(self.config.bar_width) + 1 - body_length;
        let mut head_length = 0;

        if void_length > 0 {
            void_length -= 1;
            head_length += 1;
        }

        (body_length, void_length, head_length)
    }

    pub fn update(&mut self, progress: f32) {
        if !self.need_refresh() {
            return;
        }

        self.last_update_time = Some(Instant::now());

        let (body, void, head) = self.bar_shape(progress);
        let body = self.config.shape_body.to_string().repeat(body);
        let head = self.config.shape_head.to_string().repeat(head);
        let void = self.config.shape_void.to_string().repeat(void);

        write!(
            &mut self.config.output_stream,
            "\r{} {:>5.1}% [{}{}{}] {}",
            self.config.prefix,
            100. * progress,
            body,
            head,
            void,
            self.config.extra_infos
        )
        .unwrap();
        stdout().flush().unwrap();
    }

    pub fn finished(&mut self) {
        self.last_update_time = None;
        self.update(1.0);
        writeln!(&mut self.config.output_stream).unwrap();
    }
}
