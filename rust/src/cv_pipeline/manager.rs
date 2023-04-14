use opencv::prelude::*;
use std::cell::RefCell;
use anyhow::{anyhow,Result};
use std::rc::{Rc};
use std::time::Instant;
use log::{debug};

use crate::cv_pipeline::{SourceStage,Stage};


//It uses internal mutability pattern
//This can be improved by using generics to only use internal mutability if needed

//Another approach is to build a context that is passed to the process function which retrieves a list of vectors
pub struct CVPipelineManager{
    start_stage: Option<Rc<RefCell<dyn SourceStage>>>,
    stages: Vec<Rc<RefCell<dyn Stage>>>
}


impl CVPipelineManager{
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
            start_stage: None,
        }
    }

    pub fn set_source(&mut self, source: Rc<RefCell<dyn SourceStage>>){
        self.start_stage = Some(source);
    }

    pub fn add_stage(&mut self, stage: Rc<RefCell<dyn Stage>>){
        self.stages.push(stage);
    }

    pub fn process(&mut self) -> Result<Box<Mat>>{

        let start = Instant::now();

        if let Some(ref mut start_stage) = self.start_stage {
            let mut frame = start_stage.borrow_mut().get_frame()?;
            debug!("Retrieving data from source took {:?}",start.elapsed());

            let start = Instant::now();
            self.process_image(frame.as_mut())?;
            debug!("Pipeline processing took {:?}",start.elapsed());

            return Ok(frame);
        }
        return Err(anyhow!("Process cannot be called if a start stage was not defined"));
    }

    pub fn process_image(&mut self, input: &mut Mat) -> Result<()> {
        for stage in self.stages.iter_mut() {
            let mut bor_stage = stage.borrow_mut();
            debug!("Processing stage: {}", bor_stage.get_name());
            bor_stage.process(input)?;
        }
        Ok(())
    }

}