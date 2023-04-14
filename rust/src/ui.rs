use eframe::egui;
use egui::{ColorImage,Slider, Color32, RichText};
use std::sync::mpsc;

use super::InvoiceManager;
use egui_extras::{TableBuilder, Column, StripBuilder, Size};
use log::{debug, warn, info};
use super::constants::SourceType;
use std::rc::Rc;
use crate::invoice::Invoice;

pub struct InvoiceUI {
    cam_texture: Option<egui::TextureHandle>,
    last_image: Option<Box<ColorImage>>,
    image_recv: Option<mpsc::Receiver<Box<ColorImage>>>,
    focus_sender: Option<mpsc::Sender<u8>>,
    source_sender: Option<mpsc::Sender<SourceType>>,
    inv_manager: InvoiceManager,

    invoice_search_cache: Vec<Rc<dyn Invoice>>,
    invoice_search_cache_sum: f64,
    last_invoice_search_cache_sum: f64,
    find_button_active: bool,
    
    focus_value: u8,
    last_focus_value: u8,
    source_display: SourceType,
    last_source_display: SourceType,

    highlighted_invoice_id: Option<String>,
}

impl InvoiceUI{
    pub fn new(inv_manager: InvoiceManager) -> Self {
        Self {
            image_recv: None,
            inv_manager: inv_manager,
            focus_sender: None,
            source_sender: None,
            last_focus_value: 0,

            //Cache temporary invoice table
            invoice_search_cache: Vec::new(),
            last_invoice_search_cache_sum: 0.0,
            invoice_search_cache_sum: 0.0,

            //Ui elements
            cam_texture: None,
            last_image: None,
            focus_value: 70,
            highlighted_invoice_id: None,
            source_display: SourceType::Camera,
            last_source_display: SourceType::Camera,
            find_button_active: false,
        }
    }

    //Updates last image by using the sync channel
    pub fn update_last_image(&mut self){
        if let Some(image_recv) = &self.image_recv {
            for image in image_recv.try_iter(){
                self.last_image = Some(image);
            }
        }
    }

    fn build_invoice_table<'a>(&self, ui: &mut egui::Ui, invoice_iter: impl Iterator<Item= &'a Rc<dyn Invoice>>){

        TableBuilder::new(ui)
        .striped(true)
        .column(Column::initial(150.0))
        .column(Column::initial(70.0))
        .column(Column::initial(120.0))
        .min_scrolled_height(0.0)
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("ID Fatura");
            });
            header.col(|ui| {
                ui.strong("Preço");
            });
            header.col(|ui| {
                ui.strong("Data de emissão");
            });
        })
        .body(|mut body| {
            for invoice in invoice_iter {
                let id = invoice.get_id().to_string();

                let mut invoice_color = Color32::WHITE;

                if let Some(highlighted_invoice_id) = &self.highlighted_invoice_id {
                    if id == *highlighted_invoice_id {
                        invoice_color = Color32::RED;
                    }
                }
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label(RichText::new(invoice.get_id().to_string()).color(invoice_color));
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(invoice.get_price().to_string()).color(invoice_color));
                    });
                    row.col(|ui| {
                        ui.label(RichText::new(invoice.get_emission_date().to_string()).color(invoice_color));
                    });
                });
            }
        });
    }

    pub fn set_thread_reciever(&mut self, image_recv : mpsc::Receiver<Box<ColorImage>>){
        self.image_recv = Some(image_recv);
    }

    pub fn set_focus_sender(&mut self, focus_recv : mpsc::Sender<u8>){
        self.focus_sender = Some(focus_recv);
    }

    pub fn set_source_sender(&mut self, source_sender : mpsc::Sender<SourceType>){
        self.source_sender = Some(source_sender);
        
    }

    fn handle_focus_value(&mut self){
        if self.last_focus_value == self.focus_value{
            return;
        }
        if let Some(focus_sender) = &self.focus_sender {

            focus_sender.send(self.focus_value).unwrap();
            self.last_focus_value = self.focus_value;
            debug!("Sent focus value {}", self.focus_value);
        }
    }

    fn handle_source(&mut self){

        if  self.source_display == self.last_source_display{
            return;
        }
        
        if let Some(source_sender) = &self.source_sender {
            source_sender.send(self.source_display).unwrap();
            debug!("Sent source value {:?}", self.source_display);
            self.last_source_display = self.source_display;
        }
    }

    fn handle_invoice_search(&mut self){

        if !self.find_button_active{
            return;
        }

        if self.invoice_search_cache_sum == self.last_invoice_search_cache_sum {
            return;
        }
        debug!("Searching for invoice with sum {}", self.invoice_search_cache_sum);
        self.invoice_search_cache = self.inv_manager.get_best_invoice_match(self.invoice_search_cache_sum);
        info!("Found {} invoices", self.invoice_search_cache.len());
        self.last_invoice_search_cache_sum = self.invoice_search_cache_sum;
    }

    fn ui_controller(&mut self) {
        self.handle_source();
        self.handle_focus_value();
        self.handle_invoice_search();
    }

}
impl eframe::App for InvoiceUI {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ui_controller();
        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_last_image();
            let qr_result = self.inv_manager.check_qr_channel();
            
            match qr_result {
                Ok(invoice) => {

                    if let Some(invoice) = invoice{
                        let invoice_id = invoice.get_id();
                        debug!("Found invoice with id {}", invoice_id);
                        self.highlighted_invoice_id = Some(invoice_id.to_string());
                    }
                },
                Err(error) => {
                    warn!("Fail to parse QR code {} ", error);
                }
            }
            

            if let Some(screenshot) = self.last_image.take() {
                if self.cam_texture.is_some(){
                    self.cam_texture.as_mut().unwrap().set(*screenshot,Default::default());
                }else{
                    self.cam_texture = Some(ui.ctx().load_texture(
                        "screenshot",
                        *screenshot,
                        Default::default(),
                    ));
                }
            }

            StripBuilder::new(ui)
            .size(Size::relative(0.58))
            .size(Size::relative(0.02))
            .size(Size::relative(0.35))
            .size(Size::relative(0.05))
            .vertical(|mut strip| {
                strip.strip(|builder| {
                    builder
                    .size(Size::exact(20.0))
                    .size(Size::remainder())
                    .vertical(|mut strip| {
                        strip.strip(|builder|{
                            builder
                            .size(Size::relative(0.25))
                            .size(Size::relative(0.57))
                            .size(Size::relative(0.18))
                            .horizontal(|mut strip|{
                                strip.cell(|ui|{
                                    ui.add(Slider::new(&mut self.focus_value, 0..=255).step_by(5.0).text("Focus"));
                                });
                                strip.cell(|_|{
                                });
                                strip.cell(|ui|{
                                    ui.horizontal(|ui| {
                                        ui.radio_value(&mut self.source_display, SourceType::Camera, "Camera");
                                        ui.radio_value(&mut self.source_display, SourceType::Display, "Ecrã");
                                    });
                                });
                            });
                        });
                        strip.cell(|ui|{
                            if let Some(texture) = self.cam_texture.as_ref() {
                                ui.image(texture, ui.available_size());
                            }
                            }
                        );
                    });
                });
                strip.cell(|ui|{
                    ui.separator();
                });
                strip.strip(|builder| {
                    builder
                    .size(Size::relative(0.45))
                    .size(Size::relative(0.10))
                    .size(Size::relative(0.45))
                    .horizontal(|mut strip| {
                        strip.cell(|ui|{
                            ui.push_id(1, |ui| {
                                self.build_invoice_table(ui, self.inv_manager.get_invoices());
                            });
                        });
                        strip.cell(|ui|{ui.add(egui::Separator::default().vertical());});
                        strip.cell(|ui|{
                            ui.push_id(2, |ui| {
                                StripBuilder::new(ui)
                                .size(Size::remainder().at_least(80.0)) // for the table
                                .size(Size::exact(10.0)) // for the source code link
                                .vertical(|mut strip| {
                                    strip.cell(|ui| {
                                        self.build_invoice_table(ui, self.invoice_search_cache.iter());
                                    });
                                });
                            });
                        });
                    });
                });
                strip.cell(|ui|{
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add(egui::DragValue::new(&mut self.invoice_search_cache_sum).suffix("€").min_decimals(2));
                            if ui.button("Encontrar Faturas").clicked() {
                                self.find_button_active = true;
                            }else{
                                self.find_button_active = false;
                            }
                        });
                    });
                });
            });
            ctx.request_repaint();
        });
    }
}