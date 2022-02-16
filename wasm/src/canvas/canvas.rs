/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-03 20:04:33
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-15 23:47:53
 * @FilePath: /layer-painter/wasm/src/canvas/canvas.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use crate::canvas::Layer;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canvas {
    #[serde(default = "string_default")]
    name: String,

    #[serde(default = "usize_default")]
    width: usize,

    #[serde(default = "usize_default")]
    height: usize,

    #[serde(default = "vec_default")]
    layers: Vec<Layer>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            name: String::from(""),
            width: 800,
            height: 600,
            layers: vec![],
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    pub fn layers(&self) -> Vec<Layer> {
        self.layers.clone()
    }

    pub fn get_mut_layers(&mut self) -> &mut Vec<Layer> {
        &mut self.layers
    }

    pub fn set_layers(&mut self, layers: Vec<Layer>) {
        self.layers = layers;
    }

    pub fn append_layer(&mut self, layer: Layer) -> &Self {
        self.layers.push(layer);
        self
    }

    pub fn insert_layer(&mut self, index: usize, layer: Layer) -> &Self {
        self.layers.insert(index, layer);
        self
    }

    pub fn get_layer_index(&self, name: &str) -> Option<usize> {
        self.layers.iter().position(|layer| layer.name() == name)
    }

    pub fn get_layer(&self, index: usize) -> Option<&Layer> {
        self.layers.get(index)
    }

    pub fn get_mut_layer(&mut self, index: usize) -> Option<&mut Layer> {
        self.layers.get_mut(index)
    }

    pub fn delete_layer(&mut self, index: usize) {
        self.layers.remove(index);
    }

    pub fn get_layers_len(&self) -> usize {
        self.layers.len()
    }

    pub fn swap_layer(&mut self, src_layer_index: usize, dst_layer_index: usize) {
        self.layers.swap(src_layer_index, dst_layer_index);
    }
}

fn string_default() -> String {
    String::new()
}

fn vec_default() -> Vec<Layer> {
    Vec::new()
}

fn usize_default() -> usize {
    0
}
