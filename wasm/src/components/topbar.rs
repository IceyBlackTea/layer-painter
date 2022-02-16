/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-12 23:27:41
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-16 13:18:22
 * @FilePath: /layer-painter/wasm/src/components/topbar.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use yew::{html, Callback, Component, Context, Html, Properties};

pub enum Msg {
    ToggleDisplayMenus
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub callback: Option<Callback<()>>,
}

pub struct TopBar {}

impl Component for TopBar {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleDisplayMenus => {
                let callback = ctx.props().callback.clone().unwrap();
                callback.emit(());
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="flex flex-1 flex-col">
                    <nav class="h-16 border-b-2 px-4 flex justify-between">
                        <ul class="flex items-center">
                            <li class="w-10 h-10">
                                <a href="https://github.com/IceyBlackTea/layer-painter" target="_blank">
                                    <img
                                        class="w-full h-full rounded-full mx-auto"
                                        src="img/self-small.png"
                                        alt="IceyBlackTea"
                                    />
                                </a>
                            </li>
                        </ul>
                        <ul class="flex items-center">
                            <li>
                                <h1 class="pl-8 text-3xl text-gray-700 font-extrabold select-none">
                                    {"Layer Painter"}
                                </h1>
                            </li>
                        </ul>
                        <ul class="flex items-center">
                            <li class="pr-6">
                                <label class="hover:cursor-pointer" for="menu-btn">
                                    <svg
                                        class="h-6 w-6"
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none" viewBox="0 0 24 24" stroke="currentColor"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M4 6h16M4 12h16M4 18h16"
                                        />
                                    </svg>
                                    <input 
                                        class="hidden" type="button" id="menu-btn"
                                        onclick={ctx.link().callback(|_| Msg::ToggleDisplayMenus)}
                                    />
                                </label>
                            </li>
                        </ul>
                    </nav>
                </div>
            </>
        }
    }
}
