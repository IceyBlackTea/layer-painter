/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-06 21:37:33
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-15 23:51:25
 * @FilePath: /layer-painter/wasm/src/utils/canvas.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use crate::canvas::*;

pub fn canvas_append_new_fit_layer(canvas: &mut Canvas, name: &str) {
    let mut layer = Layer::empty(name);
    layer.set_dw(canvas.width() as f64);
    layer.set_dh(canvas.height() as f64);
    canvas.append_layer(layer);
}

pub fn canvas_insert_new_fit_layer(canvas: &mut Canvas, index: usize, name: &str) {
    let mut layer = Layer::empty(name);
    layer.set_dw(canvas.width() as f64);
    layer.set_dh(canvas.height() as f64);
    canvas.insert_layer(index, layer);
}

pub fn canvas_duplicate_layer(canvas: &mut Canvas, layer_index: usize) {
    let layers = canvas.get_mut_layers();
    let mut layer = layers.get(layer_index).unwrap().clone();
    layer.set_name(format!("{}-{}", layer.name(), "duplicate").as_str());

    layers.insert(layer_index, layer);
}

pub fn move_image_to_other_layer(
    canvas: &mut Canvas,
    src_layer_index: usize,
    dst_layer_index: usize,
) {
    let src_layer = canvas.get_mut_layer(src_layer_index).unwrap();
    let dst_layer_index = dst_layer_index;

    let image = src_layer.remove_selected_image();

    match image {
        Ok(image) => {
            if src_layer.images().len() == 0 {
                canvas.delete_layer(src_layer_index);
            }

            let dst_layer = canvas.get_mut_layer(dst_layer_index).unwrap();
            dst_layer.append_image(image);
            dst_layer.set_selected(dst_layer.images().len() - 1);
        }

        Err(_) => {}
    }
}

pub fn copy_image_data_from_canvas(canvas: &mut Canvas, other_canvas: Canvas) {
    for layer in canvas.get_mut_layers().iter_mut() {
        for other_layer in other_canvas.layers().iter() {
            for other_image in other_layer.images().iter() {
                for image in layer.get_mut_images().iter_mut() {
                    if image.name().as_str() == other_image.name().as_str() {
                        let data = &other_image.data();
                        match data {
                            Some(data) => image.set_data(data),
                            None => {}
                        }
                    }
                }
            }
        }
    }
}
