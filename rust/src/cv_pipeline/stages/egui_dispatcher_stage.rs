use opencv::{prelude::* ,imgproc};
use anyhow::{Result, anyhow};
use crate::cv_pipeline::Stage;
use egui::{ColorImage, Color32};
use std::mem::transmute;
pub struct BGRConvertToEguiStage{
    last_image: Option<Box<egui::ColorImage>>
}


impl BGRConvertToEguiStage{
    pub fn new() -> Self {
        Self {
            last_image: None
        }
    }

    pub fn pop_last_image(&mut self) -> Option<Box<egui::ColorImage>> {
        return self.last_image.take();
    }

    fn convert_rba_to_color_image(size: [usize; 2], rgba: *const u8) -> ColorImage {
        let color_32_ptr: *const Color32 = unsafe { transmute::<*const u8, * const Color32>(rgba)};
        let pixels_ref = unsafe { std::slice::from_raw_parts(color_32_ptr, size[0] * size[1]) };
        let pixels = pixels_ref.to_vec();

        ColorImage{ size, pixels }
    }
}

impl Stage for BGRConvertToEguiStage {
    // Is still possible to imporve this by avoiding copying data
    fn process(&mut self, input: &mut Mat) -> Result<()>{
        // Convert frame into RGBA
        let mut rgba_frame = Mat::default();
        imgproc::cvt_color(input, &mut rgba_frame, imgproc::COLOR_BGR2RGBA, 0)?;

        let width = rgba_frame.size()?.width as usize;
        let height = rgba_frame.size()?.height as usize;
        
        let rga_raw_data = rgba_frame.data();

        if rga_raw_data.is_null() {
            return Err(anyhow!("Could not get frame data"));
        }

        //Transform the Mat object into a ColorImage object
        let color_image = Box::new(BGRConvertToEguiStage::convert_rba_to_color_image([width,height], rga_raw_data));
        self.last_image = Some(color_image);

        Ok(())
    }
    fn get_name(&self) -> &str{
        return "ConvertToEguiStage";
    }
}


