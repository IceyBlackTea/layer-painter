/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-07 22:34:02
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-16 13:16:00
 * @FilePath: /layer-painter/wasm/src/components/pane.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use crate::components::app::Msg as AppMsg;
use crate::components::button::Button;

use web_sys::{DragEvent, HtmlDivElement, HtmlInputElement, HtmlLabelElement};
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

pub enum Msg {
    // event
    ToggleLayerMenu,
    CloseLayerMenu,
    ShowLayerNameInput,
    ShowLayerOpacityInput,
    ToggleDstLayerNameInput,
    Refresh,

    // drag
    DragStart,
    DragOver,
    DragLeave,
    DragDrop(usize),

    // image
    PrevImage,
    NextImage,
    // SelectImage,
    MoveImage,
    DeleteImage,

    // layer
    RenameLayer,
    MoveLayerIndex(usize, usize),
    ToggleLayerShow,
    ToggleLayerFit,
    SetLayerOpacity,
    DuplicateLayer,
    DeleteLayer,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub index: usize,
    pub layer_name: String,
    pub layer_selected: usize,
    pub layer_show: bool,
    pub layer_fit: bool,
    pub images_len: usize,
    pub image_name: String,
    pub image_state: bool,
    pub layer_opacity: f64,
    pub callback: Option<Callback<AppMsg>>,
}

pub struct Pane {
    pane_div_node: NodeRef,
    layer_name_label_node: NodeRef,
    layer_name_input_node: NodeRef,
    layer_opacity_label_node: NodeRef,
    layer_opacity_input_node: NodeRef,
    dst_layer_name_div_node: NodeRef,
    dst_layer_name_input_node: NodeRef,
    buttons_div_node: NodeRef,
}

impl Component for Pane {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            pane_div_node: NodeRef::default(),
            layer_name_label_node: NodeRef::default(),
            layer_name_input_node: NodeRef::default(),
            layer_opacity_label_node: NodeRef::default(),
            layer_opacity_input_node: NodeRef::default(),
            dst_layer_name_div_node: NodeRef::default(),
            dst_layer_name_input_node: NodeRef::default(),
            buttons_div_node: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let index = ctx.props().index;
        match msg {
            // event
            Msg::ToggleLayerMenu => {
                let groups = self.buttons_div_node.cast::<HtmlDivElement>().unwrap();
                if groups.class_name().as_str() == "hidden" {
                    groups.set_class_name("relative my-2");
                } else {
                    groups.set_class_name("hidden");
                }
            }

            Msg::CloseLayerMenu => {
                let buttons_div_node = self.buttons_div_node.cast::<HtmlDivElement>().unwrap();
                buttons_div_node.set_class_name("hidden");
            }

            Msg::ShowLayerNameInput => {
                let layer_name_label_node =
                    self.layer_name_label_node.cast::<HtmlDivElement>().unwrap();
                let layer_name_input_node = self
                    .layer_name_input_node
                    .cast::<HtmlInputElement>()
                    .unwrap();
                layer_name_label_node.set_class_name("hidden");
                layer_name_input_node.set_class_name("input-text");
                layer_name_input_node.focus().unwrap();
            }

            Msg::ShowLayerOpacityInput => {
                let layer_opacity_label_node = self
                    .layer_opacity_label_node
                    .cast::<HtmlLabelElement>()
                    .unwrap();
                let layer_opacity_input_node = self
                    .layer_opacity_input_node
                    .cast::<HtmlInputElement>()
                    .unwrap();
                layer_opacity_label_node.set_class_name("hidden");
                layer_opacity_input_node.set_class_name("input-float");
                layer_opacity_input_node.focus().unwrap();
            }

            Msg::ToggleDstLayerNameInput => {
                let dst_layer_name_div_node = self
                    .dst_layer_name_div_node
                    .cast::<HtmlDivElement>()
                    .unwrap();
                if dst_layer_name_div_node.class_name().as_str() == "hidden" {
                    dst_layer_name_div_node.set_class_name("");
                    dst_layer_name_div_node.focus().unwrap();
                } else {
                    dst_layer_name_div_node.set_class_name("hidden");
                }
            }

            Msg::Refresh => {
                let layer_name_label_node =
                    self.layer_name_label_node.cast::<HtmlDivElement>().unwrap();
                let layer_name_input_node = self
                    .layer_name_input_node
                    .cast::<HtmlInputElement>()
                    .unwrap();
                let layer_opacity_label_node = self
                    .layer_opacity_label_node
                    .cast::<HtmlLabelElement>()
                    .unwrap();
                let layer_opacity_input_node = self
                    .layer_opacity_input_node
                    .cast::<HtmlInputElement>()
                    .unwrap();
                let dst_layer_name_div_node = self
                    .dst_layer_name_div_node
                    .cast::<HtmlDivElement>()
                    .unwrap();
                let dst_layer_name_input_node = self
                    .dst_layer_name_input_node
                    .cast::<HtmlInputElement>()
                    .unwrap();

                layer_name_label_node.set_class_name("layer-name-div");
                layer_name_input_node.set_class_name("hidden");
                layer_opacity_label_node.set_class_name("layer-name-label ml-2");
                layer_opacity_input_node.set_class_name("hidden");
                dst_layer_name_div_node.set_class_name("hidden");
                dst_layer_name_input_node.set_value("");
            }

            Msg::DragStart => {
                ctx.link().send_message(Msg::CloseLayerMenu);
            }

            Msg::DragOver => {
                let pane_div = self.pane_div_node.cast::<HtmlDivElement>().unwrap();
                pane_div.set_class_name("outline-dashed outline-2 outline-blue-400 mx-1 my-2 py-2 hover:cursor-grabbing");
            }

            Msg::DragLeave => {
                let pane_div = self.pane_div_node.cast::<HtmlDivElement>().unwrap();
                pane_div.set_class_name("mx-1 my-2 py-2 hover:cursor-grab");
            }

            Msg::DragDrop(layer_index) => {
                let dst_layer_index = ctx.props().index;
                if layer_index != dst_layer_index {
                    ctx.link().send_message(Msg::MoveLayerIndex(layer_index, dst_layer_index));
                }
                let pane_div = self.pane_div_node.cast::<HtmlDivElement>().unwrap();
                pane_div.set_class_name("mx-1 my-2 py-2 hover:cursor-grab");
            }

            others => {
                let app_msg = match others {
                    // image
                    Msg::PrevImage => AppMsg::PrevImage(index),
                    Msg::NextImage => AppMsg::NextImage(index),
                    // Msg::SelectImage(index) => {
                    //     AppMsg::SelectImage(index)
                    // }
                    Msg::MoveImage => {
                        let dst_layer_name_input_node = self
                            .dst_layer_name_input_node
                            .cast::<HtmlInputElement>()
                            .unwrap();
                        let dst_layer_name = dst_layer_name_input_node.value();

                        ctx.link().send_message(Msg::Refresh);
                        AppMsg::MoveImage(index, dst_layer_name)
                    }
                    Msg::DeleteImage => AppMsg::DeleteImage(index),
                    Msg::RenameLayer => {
                        let layer_name_input_node = self
                            .layer_name_input_node
                            .cast::<HtmlInputElement>()
                            .unwrap();
                        let new_layer_name = layer_name_input_node.value();
                        ctx.link().send_message(Msg::Refresh);
                        AppMsg::RenameLayer(index, new_layer_name)
                    }
                    // layer
                    Msg::MoveLayerIndex(src_layer_index, dst_layer_index) => {
                        AppMsg::MoveLayerIndex(src_layer_index, dst_layer_index)
                    }
                    Msg::ToggleLayerShow => {
                        ctx.link().send_message(Msg::CloseLayerMenu);
                        AppMsg::ToggleLayerShow(index)
                    },
                    Msg::ToggleLayerFit => AppMsg::ToggleLayerFit(index),
                    Msg::SetLayerOpacity => {
                        let layer_opacity_input_node = self
                            .layer_opacity_input_node
                            .cast::<HtmlInputElement>()
                            .unwrap();
                        let mut opacity = layer_opacity_input_node.value_as_number();
                        if opacity.is_nan() {
                            opacity = ctx.props().layer_opacity;
                        }

                        ctx.link().send_message(Msg::Refresh);

                        AppMsg::SetLayerOpacity(index, opacity)
                    }
                    Msg::DuplicateLayer => AppMsg::DuplicateLayer(index),
                    Msg::DeleteLayer => AppMsg::DeleteLayer(index),

                    _ => AppMsg::Error(String::from("Error")),
                };
                ctx.props().callback.clone().unwrap().emit(app_msg);
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();

        let image_name_msg = format!(
            "No.{} / {}: {}{}",
            props.layer_selected + 1,
            props.images_len,
            props.image_name,
            match props.image_state {
                true => "",
                false => " - Not loaded",
            },
        );

        let index = ctx.props().index;

        let drag_start = ctx.link().callback(move |e: DragEvent| {
            e.data_transfer()
                .unwrap()
                .set_data("text/plain", index.to_string().as_str())
                .unwrap();
            Msg::DragStart
        });

        let allow_drop = ctx.link().callback(|e: DragEvent| {
            e.prevent_default();
            Msg::DragOver
        });

        let drag_drop = ctx.link().callback(move |e: DragEvent| {
            let data = e.data_transfer().unwrap().get_data("text/plain").unwrap();
            Msg::DragDrop(data.to_string().parse::<usize>().unwrap())
        });

        html! {
            <div
                id={props.index.to_string()}
                class="mx-1 my-2 py-2 hover:cursor-grab"
                ref={self.pane_div_node.clone()}
                draggable="true"
                ondragstart={drag_start}
                ondragover={allow_drop}
                ondragleave={ctx.link().callback(|_| Msg::DragLeave)}
                ondrop={drag_drop}
            >
                <div class="flex flex-row justify-between items-center">
                    <div>
                        <input
                            class="hidden"
                            ref={self.layer_name_input_node.clone()}
                            type="text"
                            placeholder={props.layer_name.clone()}
                            value={props.layer_name.clone()}
                            onfocusout={ctx.link().callback(|_| Msg::RenameLayer)}
                        />
                        <div ref={self.layer_name_label_node.clone()} class="layer-name-div">
                            <label
                                class="layer-name-label"
                                for={format!("layer-name-btn-{}", props.index)}
                            >
                                <span class="truncate">{props.layer_name.clone()}</span>
                                <input
                                    id={format!("layer-name-btn-{}", props.index)}
                                    type="button"
                                    onclick={ctx.link().callback(|_| Msg::ShowLayerNameInput)}
                                />
                            </label>
                        </div>
                    </div>
                    <div class="mr-4">
                        <Button
                            id={format!("{}-show-btn", props.index)}
                            class="btn btn-blue"
                            onclick={ctx.link().callback(|_| Msg::ToggleLayerShow)}
                            title={
                                if props.layer_show {
                                    "show"
                                } else {
                                    "hide"
                                }
                            }
                        >
                            <svg
                                class="icon-svg"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none" viewBox="0 0 24 24" stroke="currentColor"
                            >
                                {
                                    if props.layer_show {
                                        html!{
                                            <>
                                                <path
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    stroke-width="2"
                                                    d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                                />
                                                <path
                                                    stroke-linecap="round"
                                                    stroke-linejoin="round"
                                                    stroke-width="2"
                                                    d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                                                />
                                            </>
                                        }
                                    } else {
                                        html!{
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21"
                                            />
                                        }
                                    }
                                }
                            </svg>
                        </Button>
                        <Button
                            id={format!("{}-prev-btn", props.index)}
                            class="btn btn-blue"
                            onclick={ctx.link().callback(|_| Msg::PrevImage)}
                            title="prev"
                            disabled={!props.layer_show}
                        >
                            <svg
                                class="icon-svg"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none" viewBox="0 0 24 24" stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M11 15l-3-3m0 0l3-3m-3 3h8M3 12a9 9 0 1118 0 9 9 0 01-18 0z"
                                />
                            </svg>
                        </Button>
                        <Button
                            id={format!("{}-next-btn", props.index)}
                            class="btn btn-blue"
                            onclick={ctx.link().callback(|_| Msg::NextImage)}
                            title="next"
                            disabled={!props.layer_show}
                        >
                            <svg
                                class="icon-svg"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none" viewBox="0 0 24 24" stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M13 9l3 3m0 0l-3 3m3-3H8m13 0a9 9 0 11-18 0 9 9 0 0118 0z"
                                />
                            </svg>
                        </Button>
                        <Button
                            id={format!("{}-menu-btn", props.index)}
                            class="btn btn-blue"
                            onclick={ctx.link().callback(|_| Msg::ToggleLayerMenu)}
                            disabled={!props.layer_show}
                        >
                            <svg
                                class="icon-svg"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none" viewBox="0 0 24 24" stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"
                                />
                            </svg>
                        </Button>
                    </div>
                </div>
                <div
                    class="hidden"
                    ref={self.buttons_div_node.clone()}
                >
                    <div class="flex items-center justify-start h-10 ml-1 px-4">
                        <div class="w-64 overflow-x-scroll">
                            <span class="input-label truncate">{image_name_msg}</span>
                        </div>
                        <div class="flex items-center">
                            <span class="input-label">{"opacity: "}</span>
                            <label
                                class="layer-name-label ml-2"
                                ref={self.layer_opacity_label_node.clone()}
                                for={format!("layer-opacity-btn-{}", props.index)}
                            >
                                <span class="input-label">{props.layer_opacity}</span>
                                <input type="button"
                                    id={format!("layer-opacity-btn-{}", props.index)}
                                    onclick={ctx.link().callback(|_| Msg::ShowLayerOpacityInput)}
                                />
                            </label>
                            <input
                                class="hidden"
                                ref={self.layer_opacity_input_node.clone()}
                                type="number"
                                placeholder={format!("{}", props.layer_opacity)}
                                value={props.layer_opacity.to_string()}
                                onfocusout={ctx.link().callback(|_| Msg::SetLayerOpacity)}
                            />
                        </div>
                    </div>
                    <div class="flex justify-evenly items-center px-4 mt-3">
                        <button
                            class="btn btn-blue"
                            onclick={ctx.link().callback(|_| Msg::ToggleLayerFit)}
                            title={
                                if props.layer_fit {
                                    "fit"
                                } else {
                                    "unfit"
                                }
                            }
                        >
                            <svg
                                class="icon-svg"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none" viewBox="0 0 24 24" stroke="currentColor"
                            >
                                {
                                    if props.layer_fit {
                                        html!{
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M14 10l-2 1m0 0l-2-1m2 1v2.5M20 7l-2 1m2-1l-2-1m2 1v2.5M14 4l-2-1-2 1M4 7l2-1M4 7l2 1M4 7v2.5M12 21l-2-1m2 1l2-1m-2 1v-2.5M6 18l-2-1v-2.5M18 18l2-1v-2.5"
                                            />
                                        }
                                    } else {
                                        html!{
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"
                                            />
                                        }
                                    }
                                }
                            </svg>
                        </button>
                        <button
                            class="btn btn-blue"
                            onclick={ctx.link().callback(|_| Msg::ToggleDstLayerNameInput)}
                            tilte="move image"
                        >
                            <svg
                                class="icon-svg"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none" viewBox="0 0 24 24" stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2" d="M14.121 14.121L19 19m-7-7l7-7m-7 7l-2.879 2.879M12 12L9.121 9.121m0 5.758a3 3 0 10-4.243 4.243 3 3 0 004.243-4.243zm0-5.758a3 3 0 10-4.243-4.243 3 3 0 004.243 4.243z"
                                />
                            </svg>
                        </button>
                        <button
                            class="btn btn-blue"
                            onclick={ctx.link().callback(|_| Msg::DeleteImage)}
                            title="delete image"
                        >
                        <svg
                            class="icon-svg"
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none" viewBox="0 0 24 24" stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                            />
                        </svg>
                        </button>
                        <button
                            class="btn btn-blue"
                            onclick={ctx.link().callback(|_| Msg::DuplicateLayer)}
                            title="duplicate layer"
                        >
                            <svg
                                class="icon-svg"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none" viewBox="0 0 24 24" stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2"
                                />
                            </svg>
                        </button>
                        <button
                            class="btn btn-blue"
                            onclick={ctx.link().callback(|_| Msg::DeleteLayer)}
                            title="delete layer"
                        >
                            <svg
                                class="icon-svg"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none" viewBox="0 0 24 24" stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M9 13h6M3 17V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z"
                                />
                            </svg>
                        </button>
                    </div>
                </div>
                <div class="px-6">
                    <div class="hidden" ref={self.dst_layer_name_div_node.clone()}>
                        <span class="input-label">{"Move Image to Layer: "}</span>
                        <input
                            class="input-text"
                            ref={self.dst_layer_name_input_node.clone()}
                            type="text"
                            placeholder="Cancel"
                            onfocusout={ctx.link().callback(|_| Msg::MoveImage)}
                        />
                    </div>
                </div>
            </div>
        }
    }
}
