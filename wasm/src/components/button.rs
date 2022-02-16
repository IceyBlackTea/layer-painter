/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-16 11:40:48
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-16 13:10:50
 * @FilePath: /layer-painter/wasm/src/components/button.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */
/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-12 23:27:41
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-16 00:03:19
 * @FilePath: /layer-painter/wasm/src/components/topbar.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use yew::{html, Children, Callback, Component, Context, Html, Properties};

pub enum Msg {
    Click,
}

#[derive(PartialEq, Properties)]
pub struct Props
{
    pub id: String,
    #[prop_or_default]
    pub class: &'static str,
    #[prop_or_default]
    pub title: &'static str,
    #[prop_or_default]
    pub onclick: Option<Callback<()>>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

pub struct Button {}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                let callback = ctx.props().onclick.clone().unwrap();
                callback.emit(());
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <label
                class={
                    if  !ctx.props().disabled {
                        ctx.props().class
                    } else {
                        "btn btn-disable"
                    }
                }
                title={ctx.props().title} 
                for={ctx.props().id.clone()}
            >
                { for ctx.props().children.iter() }
                <input 
                    class="hidden" 
                    id={ctx.props().id.clone()}
                    
                    disabled={ctx.props().disabled}
                    onclick={ctx.link().callback(|_| Msg::Click)}
                />
            </label>
        }
    }
}
