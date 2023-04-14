use crate::cv_pipeline::Stage;
use super::super::super::qr_code::QRCode;
use opencv::{
    prelude::*,
    objdetect,
    imgproc,
    types,
    core,
};
use anyhow::Result;
use std::sync::mpsc;



pub struct QRCodeDecoderStage {
    qr_detector : objdetect::QRCodeDetector,
    qr_sender: Option<mpsc::Sender<Box<QRCode>>>
}

impl QRCodeDecoderStage{
    pub fn new() -> Self {
        Self {
            qr_detector: objdetect::QRCodeDetector::default().unwrap(),
            qr_sender: None
        }
    }

    pub fn send_qr_code(&self, img : &Mat, decoded_str : &str, rect: &types::VectorOfPoint){
        if let Some(qr_sender) = &self.qr_sender {
            let vec: core::Rect = imgproc::bounding_rect(rect).unwrap(); 
            let qr_code = Box::new(QRCode::new(img.clone(), decoded_str.to_string(), vec));
            qr_sender.send(qr_code).unwrap();
        }
    }
}

impl Stage for QRCodeDecoderStage{
    fn process(&mut self, input: &mut Mat) -> Result<()>{
        let mut res = types::VectorOfPoint::new(); 
        let mut recqr = Mat::default();
        let ret = self.qr_detector.detect_and_decode(input, &mut res, &mut recqr)?;

        let s = String::from_utf8_lossy(&ret).to_string();
        if res.len()>0 {
            imgproc::polylines(input, &res, true, core::Scalar::new(0f64,255f64,0f64,0f64), 1,1,0)?;
            self.send_qr_code(&input, &s, &res);
        }

        return Ok(());
    }
    fn get_name(&self) -> &str{
        return "QRCodeDecoderStage";
    }
}