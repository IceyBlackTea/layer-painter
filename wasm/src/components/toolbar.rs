/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-07 19:42:33
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-15 23:56:44
 * @FilePath: /layer-painter/wasm/src/components/toolbar.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use crate::components::app::{Msg as AppMsg, State};

use crate::utils::*;
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

use web_sys::HtmlInputElement;

pub enum Msg {
    Reset,
    Resize,
    Save,
    FetchImages(String, Vec<gloo_file::File>),
    FetchJson(Vec<gloo_file::File>),
    Refresh,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub width: usize,
    pub height: usize,
    pub callback: Option<Callback<AppMsg>>,
}

pub struct ToolBar {
    canvas_name_node: NodeRef,
    canvas_width_node: NodeRef,
    canvas_height_node: NodeRef,
    image_upload_node: NodeRef,
    json_upload_node: NodeRef,
    layer_name_node: NodeRef,
}

impl Component for ToolBar {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas_name_node: NodeRef::default(),
            canvas_width_node: NodeRef::default(),
            canvas_height_node: NodeRef::default(),
            image_upload_node: NodeRef::default(),
            json_upload_node: NodeRef::default(),
            layer_name_node: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Resize => {
                let width = self
                    .canvas_width_node
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value_as_number() as usize;

                let height = self
                    .canvas_height_node
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value_as_number() as usize;

                let resize_callback = ctx.props().callback.clone().unwrap();
                resize_callback.emit(AppMsg::Resize(width, height));
            }

            Msg::Reset => {
                let reset_callback = ctx.props().callback.clone().unwrap();
                ctx.link().send_message(Msg::Refresh);
                reset_callback.emit(AppMsg::Reset);
            }

            Msg::FetchImages(layer, files) => {
                let load_callback = ctx.props().callback.clone().unwrap();
                ctx.link().send_future(async move {
                    match load_images(files).await {
                        Ok(images) => {
                            load_callback.emit(AppMsg::LoadImages(layer, State::Success(images)));
                            Msg::Refresh
                        }
                        Err(err) => {
                            load_callback.emit(AppMsg::LoadImages(layer, State::Failed(err)));
                            Msg::Refresh
                        }
                    }
                });
            }

            Msg::FetchJson(files) => {
                let load_callback = ctx.props().callback.clone().unwrap();
                ctx.link().send_future(async move {
                    match load_json(files).await {
                        Ok(data) => {
                            load_callback.emit(AppMsg::LoadJson(data));
                            Msg::Refresh
                        }
                        Err(err) => {
                            load_callback.emit(AppMsg::Error(err));
                            Msg::Refresh
                        }
                    }
                });
            }

            Msg::Save => {
                let mut canvas_name = self
                    .canvas_name_node
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                if canvas_name.as_str() == "" {
                    canvas_name = String::from("untitled");
                }

                let save_callback = ctx.props().callback.clone().unwrap();
                save_callback.emit(AppMsg::Save(canvas_name));
            }

            Msg::Refresh => {
                self.image_upload_node
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .set_value("");

                self.json_upload_node
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .set_value("");
                self.layer_name_node
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .set_value("");
                self.canvas_name_node
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .set_value("");
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let image_upload_ref = self.image_upload_node.clone();
        let layer_name_ref = self.layer_name_node.clone();
        let upload_images = ctx.link().callback(move |_| {
            let mut result = Vec::new();
            let layer_name = layer_name_ref.cast::<HtmlInputElement>().unwrap().value();
            let input = image_upload_ref.cast::<HtmlInputElement>().unwrap();
            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(gloo_file::File::from);
                result.extend(files);
            }
            Msg::FetchImages(layer_name, result)
        });

        let json_upload_ref = self.json_upload_node.clone();
        let upload_json = ctx.link().callback(move |_| {
            let mut result = Vec::new();
            let input = json_upload_ref.cast::<HtmlInputElement>().unwrap();
            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(gloo_file::File::from);
                result.extend(files);
            }
            Msg::FetchJson(result)
        });

        html! {
            <div class="flex flex-auto flex-wrap flex-col items-center">
                <div class="flex flex-auto flex-nowrap flex-row items-center">
                    <div>
                        <label class="btn icon-btn-grey" for="reset-btn">
                            <svg 
                                class="icon-svg mr-0"
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none" viewBox="0 0 24 24" stroke="currentColor"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                                />
                            </svg>
                            <input
                                class="hidden"
                                type="button"
                                id="reset-btn"
                                value="Reset"
                                onclick={ctx.link().callback(|_| {Msg::Reset})}
                            />
                        </label>
                    </div>
                    <nav class="m-2 px-4 h-16 border-2 bg-white flex flex-nowrap shrink-0 justify-between items-center">
                        <ul class="shrink-0">
                            <li>
                                <label class="input-label">
                                    <span>{"Canvas Width"}</span>
                                </label>
                                <input
                                    class="input-number"
                                    ref={self.canvas_width_node.clone()}
                                    value={ctx.props().width.to_string()}
                                    type="number"
                                />
                            </li>
                        </ul>
                        <ul class="shrink-0">
                            <li>
                                <label class="input-label">
                                    <span>{"Canvas Height"}</span>
                                </label>
                                <input
                                    class="input-number"
                                    ref={self.canvas_height_node.clone()}
                                    value={ctx.props().height.to_string()}
                                    type="number"
                                />
                            </li>
                        </ul>
                        <ul class="shrink-0">
                            <li>
                                <label class="btn icon-btn-grey" for="resize-btn">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="icon-svg mr-0"
                                        fill="none" viewBox="0 0 24 24" stroke="currentColor"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4"
                                        />
                                    </svg>
                                    <input
                                        class="hidden"
                                        type="button"
                                        id="resize-btn"
                                        value="Resize"
                                        onclick={ctx.link().callback(|_| Msg::Resize)}
                                    />
                                </label>
                            </li>
                        </ul>
                    </nav>
                </div>
                <div class="flex flex-auto flex-nowrap shrink-0 flex-row items-center">
                    <nav class="m-2 px-4 flex shrink-0 flex-nowrap justify-between items-center bg-white h-16 border-2">
                        <ul class="shrink-0">
                            <li>
                                <label class="input-label">
                                    <span>{"Upload to Layer"}</span>
                                </label>
                                <input
                                    class="input-text"
                                    ref={self.layer_name_node.clone()}
                                    type="text"
                                    placeholder="New Layer"
                                />
                            </li>
                        </ul>
                        <ul class="shrink-0">
                            <li>
                                <label class="btn icon-btn-grey" for="image-upload-btn">
                                    <svg
                                        class="icon-svg mr-0"
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none" viewBox="0 0 22 22" stroke="currentColor"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                                        />
                                    </svg>
                                    <input
                                        ref={self.image_upload_node.clone()}
                                        id="image-upload-btn"
                                        style="display: none"
                                        type="file"
                                        multiple=true
                                        accept="image/png"
                                        onchange={upload_images}
                                    />
                                </label>
                            </li>
                        </ul>
                    </nav>
                    <nav class="m-2 px-4 flex flex-nowrap shrink-0 justify-between items-center bg-white h-16 border-2">
                        <ul class="shrink-0">
                            <label class="input-label">
                                <span>{"Canvas Name"}</span>
                            </label>
                            <input
                                class="input-text"
                                ref={self.canvas_name_node.clone()}
                                type="text"
                                placeholder="Untitled"
                            />
                        </ul>
                        <ul class="flex grow justify-end">
                            <li class="shrink-0 mr-2">
                                <label class="btn icon-btn-grey" for="save-button">
                                    <svg
                                        class="icon-svg mr-0"
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none" viewBox="0 0 22 22" stroke="currentColor"
                                    >
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                                    </svg>
                                    <input
                                        class="hidden" id="save-button" type="button"
                                        onclick={ctx.link().callback(|_| {Msg::Save})}
                                    />
                                </label>
                            </li>
                            <li class="shrink-0">
                                <label class="btn icon-btn-grey" for="json-upload-btn">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        class="icon-svg mr-0"
                                        fill="none" viewBox="0 0 22 22" stroke="currentColor"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
                                        />
                                    </svg>
                                    <input
                                        class="hidden"
                                        ref={self.json_upload_node.clone()}
                                        id="json-upload-btn"
                                        type="file"
                                        accept="application/JSON"
                                        onchange={upload_json}
                                    />
                                </label>
                            </li>
                        </ul>
                    </nav>
                </div>
            </div>
        }
    }
}
