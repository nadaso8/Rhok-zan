// module implements a shader which renders an circuit builder object
#[derive(Debug)]
pub struct Graph {
    current: bool,
}

impl Graph {
    fn new() -> Self {
        Self { current: true }
    }
}
