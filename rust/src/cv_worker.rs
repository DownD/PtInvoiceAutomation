use super::cv_pipeline::manager::CVPipelineManager;
use super::cv_pipeline::stages::camera_stage::OpenCVCameraSource;
use super::cv_pipeline::stages::display_recorder_stage::DisplaySource;
use super::cv_pipeline::stages::egui_dispatcher_stage::BGRConvertToEguiStage;
use super::cv_pipeline::stages::wechat_qr_detect_stage::WeChatQRCodeDecoderStage;
use super::qr_code;
use std::cell::RefCell;
use std::sync::mpsc;
use std::rc::{Rc};
use log::{info, warn};
use super::constants::SourceType;


// TODO: Replace refcell with lifetimes
pub struct CVWorker{

    display_source : Rc<RefCell<DisplaySource>>,
    camera_source : Rc<RefCell<OpenCVCameraSource>>,    
    egui_img_converter : Rc<RefCell<BGRConvertToEguiStage>>,
    qr_decoder_stage : Rc<RefCell<WeChatQRCodeDecoderStage>>,

    rx_source : mpsc::Receiver<SourceType>,
    rx_camera_focus : mpsc::Receiver<u8>,
    tx_img : mpsc::Sender<Box<egui::ColorImage>>,
    tx_qr : mpsc::Sender<Box<qr_code::QRCode>>,

    pipeline : CVPipelineManager,
}


impl CVWorker{
    pub fn create_pipeline(tx_img : mpsc::Sender<Box<egui::ColorImage>>, tx_qr : mpsc::Sender<Box<qr_code::QRCode>>, rx_focus : mpsc::Receiver<u8>, rx_source : mpsc::Receiver<SourceType>) -> Self {
        let mut pipeline_manager = CVPipelineManager::new();

        let rc_source_display = Rc::new(RefCell::new(DisplaySource::primary().unwrap()));
        let rc_source_camera = Rc::new(RefCell::new(OpenCVCameraSource::new(Some(0)).unwrap()));
        let rc_egui_img_converter = Rc::new(RefCell::new(BGRConvertToEguiStage::new()));
        let rc_qr_decoder_stage = Rc::new(RefCell::new(WeChatQRCodeDecoderStage::new()));
        //let mut qr_decoder_stage = Box::new(QRCodeDecoderStage::new());
        
        info!("Pipelines stages have been created");
        pipeline_manager.set_source(rc_source_camera.clone());
        pipeline_manager.add_stage(rc_qr_decoder_stage.clone());
        pipeline_manager.add_stage(rc_egui_img_converter.clone());
    
        
        Self {
            pipeline : pipeline_manager,
            camera_source : rc_source_camera,
            egui_img_converter : rc_egui_img_converter,
            qr_decoder_stage : rc_qr_decoder_stage,
            display_source : rc_source_display,

            rx_camera_focus : rx_focus,
            rx_source : rx_source,
            tx_img : tx_img,
            tx_qr : tx_qr,
        }
    }

    fn handle_change_source(&mut self){
        if let Ok(source) = self.rx_source.try_recv(){
            match source {
                SourceType::Camera => {
                    self.pipeline.set_source(self.camera_source.clone());
                },
                SourceType::Display => {
                    self.pipeline.set_source(self.display_source.clone());
                }
            }
            info!("Source has been changed");
        }
    }

    fn handle_change_focus(&mut self){
        let focus = self.rx_camera_focus.try_iter().last();
        if let Some(focus) = focus {
            self.camera_source.borrow_mut().set_focus(focus).unwrap();
        }
    }

    fn handle_new_image(&mut self){
        
        let img = self.egui_img_converter.borrow_mut().pop_last_image();
        if let Some(img) = img {
            self.tx_img.send(img).unwrap();
        }
    }

    fn handle_new_qr(&mut self){
        let qr = self.qr_decoder_stage.borrow_mut().pop_last_qrs();
        if let Some(qr_vec) = qr {
            for qr in qr_vec {
                self.tx_qr.send(qr).unwrap();
            }
        }
    }
        
    fn handle_channels(&mut self){
        self.handle_change_source();
        self.handle_change_focus();
        self.handle_new_image();
        self.handle_new_qr();
    }

    pub fn run(&mut self){
        loop {
            if let Ok(frame) = self.pipeline.process(){
                self.handle_channels();
            }else{
                warn!("Fail to process frame, skipping frame");
            }
            self.handle_channels();
        }
    }
}