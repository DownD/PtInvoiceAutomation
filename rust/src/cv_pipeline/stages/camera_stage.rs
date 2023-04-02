use opencv::{prelude::*, videoio};
use anyhow::{Result, anyhow};
use crate::cv_pipeline::SourceStage;
const DEFAULT_CAMERA_INDEX: i32 = 0;
use log::{info, debug};

pub struct OpenCVCameraSource{
    camera: videoio::VideoCapture,
}

impl OpenCVCameraSource{
    pub fn new(idx : Option<i32>) -> Result<Self> {
        let camera_index = idx.unwrap_or(DEFAULT_CAMERA_INDEX);
        debug!("Opening camera {}", camera_index);
        let mut camera = videoio::VideoCapture::new(camera_index, videoio::CAP_ANY)?;
        debug!("Camera has been created");
        let opened = videoio::VideoCapture::is_opened(&camera)?;
        if !opened {
            return Err(anyhow!("Could not open camera {}", camera_index));
        }

        let fourcc = videoio::VideoWriter::fourcc('M', 'J', 'P', 'G')?; 

        camera.set(videoio::CAP_PROP_FOURCC, fourcc as f64)?;
        camera.set(videoio::CAP_PROP_FRAME_WIDTH, 1920.0)?;
        camera.set(videoio::CAP_PROP_FRAME_HEIGHT, 1080.0)?;
        camera.set(videoio::CAP_PROP_AUTOFOCUS, 0.0)?;
        camera.set(videoio::CAP_PROP_FPS, 30.0)?;
        info!("Camera index({}) opened with resolution {}x{}", camera_index, 1920, 1080);
   
        Ok(Self {
            camera: camera
        })
    }

    pub fn set_focus(&mut self, focus: u8) -> Result<()>{
        self.camera.set(videoio::CAP_PROP_FOCUS, focus as f64)?;
        info!("Camera focus set to {}", focus);
        return Ok(());
    }
}

impl SourceStage for OpenCVCameraSource {
    fn get_frame(&mut self) -> Result<Box<Mat>> {
        let mut frame = Box::new(Mat::default());
        self.camera.read(frame.as_mut())?;

        return Ok(frame);
    }
    fn get_name(&self) -> &str{
        return "CameraSource";
    }
}