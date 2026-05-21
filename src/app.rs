use rand::{RngExt, rngs::ThreadRng};
use ratatui::widgets::{Sparkline, SparklineBar};

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }
}

pub struct Signal<S: Iterator> {
    pub source: S,
    pub points: Vec<S::Item>,
    pub tick_rate: usize,
}

impl<S> Signal<S>
where
    S: Iterator,
{
    fn on_tick(&mut self) {
        self.points.drain(0..self.tick_rate);
        self.points
            .extend(self.source.by_ref().take(self.tick_rate));
    }
}

pub struct RandomSignal {
    rng: ThreadRng,
}

impl RandomSignal {
    pub fn new() -> Self {
        Self { rng: rand::rng() }
    }
}

impl Iterator for RandomSignal {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.random_range(0.0..20.0))
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub progress: f64,
    progres_dir: bool,
    pub sparkline: Signal<RandomSignal>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let mut sparkline_signal = RandomSignal::new();
        let mut sparkline_points = Vec::<<RandomSignal as Iterator>::Item>::new();
        let mut temp_vec = sparkline_signal.by_ref().take(1000).collect();
        sparkline_points.append(&mut temp_vec);
        App {
            title: "ratatui-demo",
            tabs: TabsState::new(vec!["Tab0", "Tab1", "Tab2"]),
            progress: 0.0,
            sparkline: Signal {
                source: sparkline_signal,
                points: sparkline_points,
                tick_rate: 1,
            },
            progres_dir: true,
        }
    }

    pub fn on_tick(&mut self) {
        match self.progres_dir {
            true => {
                self.progress = (self.progress + 0.001).min(1.0);
                if self.progress >= 1.0 {
                    self.progres_dir = false;
                }
            }
            false => {
                self.progress = (self.progress - 0.001).max(0.0);
                if self.progress <= 0.0 {
                    self.progres_dir = true;
                }
            }
        }

        self.sparkline.on_tick();
    }
}
