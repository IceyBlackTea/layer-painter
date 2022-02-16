/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-07 22:12:31
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-16 13:51:39
 * @FilePath: /layer-painter/wasm/src/components/app.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use crate::canvas::*;
use crate::components::*;
use crate::utils::*;

use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlAnchorElement, HtmlCanvasElement, HtmlDivElement};
use yew::{html, props, Component, Context, Html, NodeRef};

#[derive(Debug)]
pub enum State {
    Success(Vec<Image>),
    Failed(String),
}

pub enum Msg {
    // io
    LoadImages(String, State),
    LoadJson(String),

    // menu
    ToggleDisplayMenus,

    // canvas control message
    Render,
    Reset,
    Save(String),
    Resize(usize, usize),

    // layer control message (layer_name)
    RenameLayer(usize, String),
    ToggleLayerShow(usize),
    ToggleLayerFit(usize),
    MoveLayerIndex(usize, usize),
    SetLayerOpacity(usize, f64),
    DuplicateLayer(usize),
    DeleteLayer(usize),

    // image control message (layer_name)
    PrevImage(usize),
    NextImage(usize),
    MoveImage(usize, String),
    DeleteImage(usize),

    Warn(String),
    Error(String),
}

pub struct App {
    canvas: Canvas,
    canvas_node: NodeRef,
    save_anchor_node: NodeRef,
    toolbar_div_node: NodeRef,
    layer_menu_div_node: NodeRef,
}

impl App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas: Canvas::new(),
            canvas_node: NodeRef::default(),
            save_anchor_node: NodeRef::default(),
            toolbar_div_node: NodeRef::default(),
            layer_menu_div_node: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // io
            Msg::LoadImages(layer_name, state) => match state {
                State::Success(images) => {
                    if images.len() != 0 {
                        let index = self.canvas.get_layer_index(layer_name.as_str());
                        match index {
                            Some(index) => {
                                let layer = self.canvas.get_mut_layer(index).unwrap();
                                layer.append_images(images);
                            }

                            None => {
                                let mut layer_name = layer_name;
                                if layer_name.as_str() == "" {
                                    layer_name = images.get(0).unwrap().name().clone();
                                }
                                canvas_append_new_fit_layer(&mut self.canvas, layer_name.as_str());

                                let layer = self
                                    .canvas
                                    .get_mut_layer(self.canvas.get_layers_len() - 1)
                                    .unwrap();
                                layer.append_images(images);
                            }
                        }
                        ctx.link().send_future(async { Msg::Render });
                    }
                    true
                }

                State::Failed(err) => {
                    ctx.link()
                        .send_message(Msg::Error(format!("Read Images Failed: {}", err)));
                    false
                }
            },

            Msg::LoadJson(data) => match serde_json::from_str::<Canvas>(data.as_str()) {
                Ok(canvas) => {
                    let mut canvas = canvas;
                    copy_image_data_from_canvas(&mut canvas, self.canvas.clone());
                    self.canvas = canvas;
                    ctx.link().send_future(async { Msg::Render });

                    true
                }

                Err(err) => {
                    ctx.link()
                        .send_message(Msg::Error(format!("Reading Save Json faild: {:#?}", err)));

                    false
                }
            },

            // menu
            Msg::ToggleDisplayMenus => {
                let canvas_menu = self.toolbar_div_node.cast::<HtmlDivElement>().unwrap();
                let layer_menu = self.layer_menu_div_node.cast::<HtmlDivElement>().unwrap();
                let layer_menu_state = layer_menu.class_name();
                if layer_menu_state.as_str() == "hidden" {
                    canvas_menu.set_class_name("");
                    layer_menu.set_class_name("m-4");
                } else {
                    canvas_menu.set_class_name("hidden");
                    layer_menu.set_class_name("hidden");
                }
                false
            }

            // canvas
            Msg::Render => {
                let canvas = self.canvas_node.cast::<HtmlCanvasElement>().unwrap();
                let cctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                cctx.clear_rect(
                    0.0,
                    0.0,
                    self.canvas.width() as f64,
                    self.canvas.height() as f64,
                );

                canvas.set_width(self.canvas.width() as u32);
                canvas.set_height(self.canvas.height() as u32);

                for layer in self.canvas.layers().iter() {
                    if layer.show() {
                        match layer.images().get(layer.selected()) {
                            Some(image) => match image.data() {
                                Some(data) => {
                                    if layer.opacity() < 1.0 {
                                        cctx.save();
                                        cctx.set_global_alpha(layer.opacity());
                                    }

                                    let sx = image.sx();
                                    let sy = image.sy();
                                    let sw = image.sw();
                                    let sh = image.sh();
                                    let dx = layer.dx();
                                    let dy = layer.dy();
                                    let dw = self.canvas.width() as f64;
                                    let dh = self.canvas.height() as f64;

                                    let mut scale = 1.0;

                                    let max = |x, y| {
                                        if x > y {
                                            x
                                        } else {
                                            y
                                        }
                                    };

                                    if layer.fit() {
                                        scale = 1.0 / max(sw / dw, sh / dh);
                                    }

                                    cctx.draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                                            &data,
                                            sx,
                                            sy,
                                            sw,
                                            sh,
                                            dx,
                                            dy,
                                            sw * scale,
                                            sh * scale
                                        )
                                        .unwrap();
                                    if layer.opacity() < 1.0 {
                                        cctx.restore();
                                    }
                                }
                                _ => {
                                    ctx.link().send_message(Msg::Warn(format!(
                                        "The layer: {} can't be rendered.",
                                        layer.name()
                                    )));
                                }
                            },

                            None => {
                                ctx.link().send_message(Msg::Warn(format!(
                                    "The layer: {} can't be rendered.",
                                    layer.name()
                                )));
                            }
                        }
                    }
                }

                true
            }

            Msg::Reset => {
                self.canvas = Canvas::new();
                let canvas = self.canvas_node.cast::<HtmlCanvasElement>().unwrap();
                let cctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                cctx.clear_rect(
                    0.0,
                    0.0,
                    self.canvas.width() as f64,
                    self.canvas.height() as f64,
                );

                true
            }

            Msg::Save(canvas_name) => {
                self.canvas.set_name(&canvas_name);

                let prefix = "data:application/JSON;charset=utf-8,";
                let save_data =
                    format!("{}{}", prefix, serde_json::to_string(&self.canvas).unwrap());

                let anchor = self.save_anchor_node.cast::<HtmlAnchorElement>().unwrap();

                anchor.set_href(save_data.as_str());

                anchor.set_download(format!("{}{}", canvas_name, ".json").as_str());

                anchor.click();

                false
            }

            Msg::Resize(width, height) => {
                let canvas = self.canvas_node.cast::<HtmlCanvasElement>().unwrap();
                let cctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();
                cctx.clear_rect(
                    0.0,
                    0.0,
                    self.canvas.width() as f64,
                    self.canvas.height() as f64,
                );

                self.canvas.set_width(width as usize);
                self.canvas.set_height(height as usize);

                ctx.link().send_future(async { Msg::Render });

                true
            }

            // layer
            Msg::RenameLayer(layer_index, new_layer_name) => {
                if new_layer_name.as_str() == "" {
                    return false;
                }

                let layer = self.canvas.get_mut_layer(layer_index);

                match layer {
                    Some(layer) => {
                        layer.set_name(new_layer_name.as_str());
                    }

                    None => {}
                }

                true
            }

            Msg::ToggleLayerShow(layer_index) => {
                let layer = self.canvas.get_mut_layer(layer_index);
                match layer {
                    Some(layer) => {
                        layer.set_show(!layer.show());
                        ctx.link().send_message(Msg::Render);
                    }

                    None => {}
                }

                true
            }

            Msg::ToggleLayerFit(layer_index) => {
                let layer = self.canvas.get_mut_layer(layer_index);
                match layer {
                    Some(layer) => {
                        layer.set_fit(!layer.fit());
                        ctx.link().send_message(Msg::Render);
                    }

                    None => {}
                }

                true
            }

            Msg::MoveLayerIndex(src_layer_index, dst_layer_index) => {
                self.canvas.swap_layer(src_layer_index, dst_layer_index);
                ctx.link().send_message(Msg::Render);
                true
            }

            Msg::SetLayerOpacity(layer_index, opacity) => {
                let layer = self.canvas.get_mut_layer(layer_index);
                match layer {
                    Some(layer) => {
                        layer.set_opacity(opacity);
                        ctx.link().send_message(Msg::Render);
                    }

                    None => {}
                }
                true
            }

            Msg::DuplicateLayer(layer_index) => {
                canvas_duplicate_layer(&mut self.canvas, layer_index);
                ctx.link().send_message(Msg::Render);
                true
            }

            Msg::DeleteLayer(layer_index) => {
                self.canvas.delete_layer(layer_index);
                ctx.link().send_message(Msg::Render);
                true
            }

            // image
            Msg::PrevImage(layer_index) => {
                let layer = self.canvas.get_mut_layer(layer_index);
                match layer {
                    Some(layer) => {
                        layer.prev();
                        ctx.link().send_message(Msg::Render);
                    }

                    None => {}
                }
                true
            }

            Msg::NextImage(layer_index) => {
                let layer = self.canvas.get_mut_layer(layer_index);
                match layer {
                    Some(layer) => {
                        layer.next();
                        ctx.link().send_message(Msg::Render);
                    }

                    None => {}
                }
                true
            }

            Msg::MoveImage(src_layer_index, dst_layer_name) => {
                if dst_layer_name.as_str() == "" {
                    return false;
                }

                let dst_layer_index = self.canvas.get_layer_index(dst_layer_name.as_str());
                match dst_layer_index {
                    Some(dst_layer_index) => {
                        if src_layer_index != dst_layer_index {
                            move_image_to_other_layer(
                                &mut self.canvas,
                                src_layer_index,
                                dst_layer_index,
                            );
                            ctx.link().send_message(Msg::Render);
                        }
                    }

                    None => {
                        canvas_insert_new_fit_layer(
                            &mut self.canvas,
                            src_layer_index,
                            dst_layer_name.as_str(),
                        );
                        move_image_to_other_layer(
                            &mut self.canvas,
                            src_layer_index + 1,
                            src_layer_index,
                        );
                        ctx.link().send_message(Msg::Render);
                    }
                }
                true
            }

            Msg::DeleteImage(layer_index) => {
                let layer = self.canvas.get_mut_layer(layer_index);
                match layer {
                    Some(layer) => {
                        if layer.images().len() > 1 {
                            layer.remove_selected_image().unwrap();
                            ctx.link().send_message(Msg::Render);
                        } else {
                            ctx.link().send_message(Msg::DeleteLayer(layer_index));
                        }

                        true
                    }

                    None => false,
                }
            }

            Msg::Warn(warn) => {
                // gloo_dialogs::alert(warn.as_str());
                log::warn!("{}", warn);
                false
            }
            Msg::Error(err) => {
                gloo_dialogs::alert(err.as_str());
                log::error!("{}", err);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let topbar_props = props!(TopBar::Properties {
            callback: Some(ctx.link().callback(|_| Msg::ToggleDisplayMenus)),
        });

        let toolbar_props = props!(ToolBar::Properties {
            width: self.canvas.width(),
            height: self.canvas.height(),
            callback: Some(ctx.link().callback(|msg| msg)),
        });

        html! {
            <div>
                <TopBar ..topbar_props/>
                <div ref={self.toolbar_div_node.clone()}>
                    <ToolBar ..toolbar_props/>
                </div>
                <div class="flex justify-center items-start">
                    <div class="border-2 m-4 overflow-scroll">
                        <canvas
                            ref={self.canvas_node.clone()}
                            width={self.canvas.width().to_string()}
                            height={self.canvas.height().to_string()}
                        />
                    </div>
                    <div ref={self.layer_menu_div_node.clone()} class="m-4">
                        <div>
                            <ul class="w-128 h-128 p-4">
                                <div class="fixed overflow-auto w-128 h-128 border-2">
                                {
                                    for self.canvas.layers().iter().enumerate().rev().map(|(index, layer)| {
                                        let (image_name, image_state) = match layer.get_selected_image() {
                                            Some(image) => {
                                                match image.data() {
                                                    Some(_) => (image.name(), true),
                                                    None => (image.name(), false)
                                                }
                                            },
                                            None => (String::from("No Image!"), false)
                                        };

                                        let pane_props = props!(Pane::Properties {
                                            index,
                                            layer_name: layer.name(),
                                            layer_selected: layer.selected(),
                                            layer_show: layer.show(),
                                            layer_fit: layer.fit(),
                                            images_len: layer.images().len(),
                                            image_name,
                                            image_state,
                                            layer_opacity: layer.opacity(),
                                            callback: Some(ctx.link().callback(|msg| msg))
                                        });

                                        html!{
                                            <Pane
                                                ..pane_props
                                            />
                                        }
                                    })
                                }
                                </div>
                            </ul>
                        </div>
                    </div>
                    <a class="hidden" ref={self.save_anchor_node.clone()}>{"save"}</a>
                </div>

            </div>
        }
    }
}

impl App {}
