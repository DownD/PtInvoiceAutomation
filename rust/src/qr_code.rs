use opencv::prelude::*;
use opencv::core::Rect;


pub struct QRCode {
    image: Mat,
    data: String,
    rect: Rect,
}


impl QRCode {
    pub fn new(image: Mat, data: String, rect: Rect) -> Self {
        Self {
            image: image,
            data: data,
            rect: rect,
        }
    }

    pub fn get_image(&self) -> &Mat {
        &self.image
    }

    pub fn get_data(&self) -> &String {
        &self.data
    }

    pub fn get_rect(&self) -> &Rect {
        &self.rect
    }

    pub fn get_qr_code_only(&self) -> Result<Mat, opencv::Error>{
        return Mat::roi(&self.image, self.rect.clone());
    }
}
