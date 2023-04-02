use opencv::{prelude::*};
use anyhow::{Result};

pub mod manager;
pub mod stages;
pub trait Stage {
    fn process(&mut self, input: &mut Mat) -> Result<()>;
    fn get_name(&self) -> &str;
}

pub trait SourceStage {
    fn get_frame(&mut self) -> Result<Box<Mat>>;
    fn get_name(&self) -> &str;
}