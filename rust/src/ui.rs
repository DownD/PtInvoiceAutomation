use eframe::egui;
use super::camera::Camera;
use super::camera::OpenCVCamera;
use egui::ColorImage;
pub struct InvoiceUI {
    cam_texture: Option<egui::TextureHandle>,
    screenshot: Option<ColorImage>,
    camera: Box<dyn Camera>,
}

impl InvoiceUI{
    pub fn new() -> Self {
        Self {
            cam_texture: None,
            screenshot: None,
            camera:Box::new(OpenCVCamera::new(None).unwrap())
        }
    }
}
impl eframe::App for InvoiceUI {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.screenshot = self.camera.get_frame().ok();
            if let Some(screenshot) = self.screenshot.take() {
                if self.cam_texture.is_some(){
                    self.cam_texture.as_mut().unwrap().set(screenshot,Default::default());
                }else{
                    self.cam_texture = Some(ui.ctx().load_texture(
                        "screenshot",
                        screenshot,
                        Default::default(),
                    ));
                }
            }

            ui.heading("My egui Application");
            if let Some(texture) = self.cam_texture.as_ref() {
                ui.image(texture, ui.available_size());
            }
            ctx.request_repaint();
        });
    }
}
