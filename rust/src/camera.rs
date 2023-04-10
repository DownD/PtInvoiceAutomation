use opencv::{prelude::*, videoio, imgproc};
use anyhow::{Result, anyhow};
use egui::ColorImage;
const DEFAULT_CAMERA_INDEX: i32 = 0;

pub struct OpenCVCamera{
    camera: videoio::VideoCapture,
    width: i32,
    height: i32
}

pub trait Camera{
    fn get_resolution(&self) -> (i32, i32);
    fn get_frame(&mut self) -> Result<ColorImage>;
}

impl OpenCVCamera{
    pub fn new(idx : Option<i32>) -> Result<Self> {
        let camera_index = idx.unwrap_or(DEFAULT_CAMERA_INDEX);
        let camera = videoio::VideoCapture::new(camera_index, videoio::CAP_V4L2)?;
        let opened = videoio::VideoCapture::is_opened(&camera)?;
        if !opened {
            return Err(anyhow!("Could not open camera {}", camera_index));
        }
        
        Ok(Self {
            width: camera.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32,
            height: camera.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32,
            camera: camera,
        })
    }
}

impl Camera for OpenCVCamera {
    fn get_resolution(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    //TODO: Check if is better to keep a buffer while reading the camera concurrently
    fn get_frame(&mut self) -> Result<ColorImage> {
        let mut frame = Mat::default();
        self.camera.read(&mut frame)?;

        // Convert frame into RGBA
        let mut rgba_frame = Mat::default();
        imgproc::cvt_color(&frame, &mut rgba_frame, imgproc::COLOR_BGR2RGBA, 0)?;

        let width = rgba_frame.size()?.width as usize;
        let height = rgba_frame.size()?.height as usize;
        
        let rga_raw_data = rgba_frame.data();

        if rga_raw_data.is_null() {
            return Err(anyhow!("Could not get frame data"));
        }
        // convert const* u8 to *u8
        let rga_raw_data = unsafe { std::slice::from_raw_parts(rga_raw_data, width*height *4) };
        //Transform the Mat object into a ColorImage object
        return Ok(ColorImage::from_rgba_unmultiplied([width,height], rga_raw_data));
    }
}