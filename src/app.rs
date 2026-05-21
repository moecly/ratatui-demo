pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }
}

struct Signal<S: Iterator> {
    source: S,
    points: Vec<S::Item>,
    tick_rate: usize,
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

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub progress: f64,
    progres_dir: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            title: "ratatui-demo",
            tabs: TabsState::new(vec!["Tab0", "Tab1", "Tab2"]),
            progress: 0.0,
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
    }
}
