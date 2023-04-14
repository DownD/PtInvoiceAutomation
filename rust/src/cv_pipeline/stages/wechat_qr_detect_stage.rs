use crate::cv_pipeline::Stage;
use super::super::super::qr_code::QRCode;
use opencv::{
    prelude::*,
    imgproc,
    types,
    core,
    wechat_qrcode::WeChatQRCode,
};
use anyhow::Result;
use log::{info};



pub struct WeChatQRCodeDecoderStage {
    qr_detector : WeChatQRCode,
    last_qr_codes: Option<Vec<Box<QRCode>>>
}

impl WeChatQRCodeDecoderStage{
    pub fn new() -> Self {
        Self {
            qr_detector: WeChatQRCode::new(
                "models/detect.prototxt", 
                "models/detect.caffemodel",
                "models/sr.prototxt", 
                "models/sr.caffemodel"      
            ).unwrap(),
            last_qr_codes: None
        }
    }
    pub fn pop_last_qrs(&mut self,) -> Option<Vec<Box<QRCode>>>{
        self.last_qr_codes.take()
    }

    fn create_qr_code(&mut self, img : &Mat, decoded_str : &str, rect: &core::Rect) -> Box<QRCode>{
        Box::new(QRCode::new(img.clone(), decoded_str.to_string(), *rect))
    }
}

impl Stage for WeChatQRCodeDecoderStage{
    fn process(&mut self, input: &mut Mat) -> Result<()>{
        let mut mat_result = types::VectorOfMat::new(); 
        let decoded_strings = self.qr_detector.detect_and_decode(input, &mut mat_result)?;

        let mut qr_codes = Vec::<Box<QRCode>>::new();

        for (qr_code_str, bbox) in decoded_strings.iter().zip(mat_result.iter()){
            let rect: core::Rect = imgproc::bounding_rect(&bbox).unwrap(); 
            info!("QR code rect: {:?}", rect);

            imgproc::rectangle(input, rect, core::Scalar::new(0f64,255f64,0f64,0f64), 1,1,0)?;
            qr_codes.push(self.create_qr_code(input, &qr_code_str, &rect));
        }

        if qr_codes.len()>0 {
            self.last_qr_codes = Some(qr_codes);
        }

        return Ok(());
    }
    fn get_name(&self) -> &str{
        return "WeChatQRCodeDecoderStage";
    }
}