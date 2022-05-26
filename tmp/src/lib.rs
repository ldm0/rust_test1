#[derive(Default, Debug)]
pub struct Run {
    pub a: bool,
    pub b: bool,
    pub c: Option<String>,
}

impl Run {
    pub fn run(&mut self) {
        if !self.a && !self.b {
            self.run();
        }
    }
}

pub fn foo() -> Run {
    let mut run = Run::default();
    run.a = false;
    run.b = false;
    run.run();
    run
}
