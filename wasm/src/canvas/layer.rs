/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-06 20:43:49
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-12 22:49:14
 * @FilePath: /layer-painter/wasm/src/canvas/layer.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use crate::canvas::Image;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    #[serde(default = "string_default")]
    name: String,

    #[serde(default = "usize_default")]
    selected: usize,

    #[serde(default = "vec_default")]
    images: Vec<Image>,

    #[serde(default = "bool_default")]
    show: bool,

    #[serde(default = "bool_default")]
    fit: bool,

    #[serde(default = "f64_default")]
    opacity: f64,

    #[serde(default = "f64_default")]
    dx: f64,

    #[serde(default = "f64_default")]
    dy: f64,

    #[serde(default = "f64_default")]
    dw: f64,

    #[serde(default = "f64_default")]
    dh: f64,
}

impl Layer {
    pub fn empty(name: &str) -> Self {
        Self {
            name: String::from(name),
            selected: 0,
            images: Vec::new(),
            show: true,
            fit: true,
            opacity: 1.0,
            dx: 0.0,
            dy: 0.0,
            dw: 0.0,
            dh: 0.0,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn set_selected(&mut self, index: usize) {
        if index >= self.images.len() {
            panic!(
                "select index (is {}) should be < len (is {})",
                index,
                self.images.len()
            )
        }

        self.selected = index;
    }

    pub fn prev(&mut self) {
        if self.images().len() == 0 {
            return
        }

        if self.selected == 0 {
            self.selected = self.images.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn next(&mut self) {
        if self.images().len() == 0 {
            return
        }
        
        if self.selected == self.images.len() - 1 {
            self.selected = 0;
        } else {
            self.selected += 1;
        }
    }

    pub fn images(&self) -> Vec<Image> {
        self.images.clone()
    }

    pub fn get_mut_images(&mut self) -> &mut Vec<Image> {
        &mut self.images
    }
    pub fn show(&self) -> bool {
        self.show
    }

    pub fn set_show(&mut self, show: bool) {
        self.show = show;
    }

    pub fn fit(&self) -> bool {
        self.fit
    }

    pub fn set_fit(&mut self, fit: bool) {
        self.fit = fit;
    }

    pub fn opacity(&self) -> f64 {
        self.opacity
    }

    pub fn set_opacity(&mut self, opacity: f64) {
        if opacity > 1.0 {
            self.opacity = 1.0;
        } else if opacity < 0.0 {
            self.opacity = 0.0;
        } else {
            self.opacity = opacity;
        }
    }

    pub fn dx(&self) -> f64 {
        return self.dx;
    }

    pub fn dy(&self) -> f64 {
        self.dy
    }

    pub fn dw(&self) -> f64 {
        self.dw
    }

    pub fn dh(&self) -> f64 {
        self.dh
    }

    pub fn set_dx(&mut self, dx: f64) {
        self.dx = dx;
    }

    pub fn set_dy(&mut self, dy: f64) {
        self.dy = dy;
    }

    pub fn set_dw(&mut self, dw: f64) {
        self.dw = dw;
    }

    pub fn set_dh(&mut self, dh: f64) {
        self.dh = dh;
    }

    pub fn append_image(&mut self, image: Image) {
        self.images.push(image);
    }

    pub fn append_images(&mut self, images: Vec<Image>) {
        self.images.extend(images);
    }

    pub fn get_selected_image(&self) -> Option<&Image> {
        self.images.get(self.selected)
    }

    pub fn remove_selected_image(&mut self) -> Result<Image, String> {
        if self.images().len() == 0 {
            return Err(String::from("select layer can't remove image, images len is 0"));
        }

        let image = self.images.remove(self.selected);
        if self.selected >= self.images.len() {
            self.selected = 0;
        }

        Ok(image)
    }
}

fn string_default() -> String {
    String::new()
}

fn vec_default() -> Vec<Image> {
    Vec::new()
}

fn usize_default() -> usize {
    0
}

fn bool_default() -> bool {
    true
}

fn f64_default() -> f64 {
    0.0
}
