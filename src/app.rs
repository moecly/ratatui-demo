pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub progress: f64,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            title: "ratatui-demo",
            tabs: TabsState::new(vec!["Tab0", "Tab1", "Tab2"]),
            progress: 0.0,
        }
    }
}
