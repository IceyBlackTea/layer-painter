/*
 * @Author: IceyBlackTea
 * @Date: 2021-12-16 08:35:39
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-07 22:22:37
 * @FilePath: /layer-painter/wasm/src/main.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

mod components;
mod canvas;
mod utils;

use components::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
