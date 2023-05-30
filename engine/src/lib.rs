use std::error::Error;

pub struct RsEngine {}

impl RsEngine {
    pub fn new() -> Self {
        RsEngine {}
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
