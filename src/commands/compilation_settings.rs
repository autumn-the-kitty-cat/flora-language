#[derive(Debug)]
pub struct CompilationSettings {
    pub input_file: Option<String>,
    pub output_file: Option<String>,
}

impl CompilationSettings {
    pub fn new() -> Self {
        Self {
            input_file: None,
            output_file: None,
        }
    }
}
