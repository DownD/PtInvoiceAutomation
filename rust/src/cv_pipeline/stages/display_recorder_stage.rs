use scrap::{Capturer, Display};
use crate::cv_pipeline::SourceStage;
use anyhow::{Result};
use opencv::{prelude::*, imgproc};
pub struct DisplaySource {
    capturer: Capturer,
}

impl DisplaySource{
    pub fn primary() -> Result<Self> {
        let display = Display::primary().unwrap();
        let capturer = Capturer::new(display).unwrap();

        Ok(Self {
            capturer: capturer
        })
    }
}

impl SourceStage for DisplaySource {
    fn get_frame(&mut self) -> Result<Box<Mat>> {
        let rows = self.capturer.height();
        let columns = self.capturer.width();
        let frame = self.capturer.frame()?;

        let mut bgr_frame = Mat::default();
        unsafe{
            let bgra_mat = Mat::new_rows_cols_with_data(rows as i32, columns as i32, opencv::core::CV_8UC4, frame.as_ptr() as *mut std::ffi::c_void, 0)?;

            imgproc::cvt_color(&bgra_mat, &mut bgr_frame, imgproc::COLOR_BGRA2BGR, 0)?;
        }

        return Ok(Box::new(bgr_frame));
    }
    fn get_name(&self) -> &str{
        return "DisplaySource";
    }
}