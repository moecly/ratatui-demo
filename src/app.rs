pub struct App<'a> {
    pub title: &'a str,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        App {
            title: "ratatui-demo",
        }
    }
}
