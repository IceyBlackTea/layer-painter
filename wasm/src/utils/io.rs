/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-06 21:36:16
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-08 00:07:31
 * @FilePath: /layer-painter/wasm/src/utils/io.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use crate::canvas::Image;

use wasm_bindgen::{JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{ImageBitmap, Request, RequestInit, RequestMode, Response};

pub async fn fetch_image(file_path: &str) -> Result<ImageBitmap, String> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(file_path, &opts).unwrap();

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();
    let resp = resp_value.dyn_into::<Response>().unwrap();

    let blob = JsFuture::from(resp.blob().unwrap())
        .await
        .unwrap()
        .dyn_into()
        .unwrap();
    
    let image_bitmap_promise = window.create_image_bitmap_with_blob(&blob).unwrap();

    let result = match JsFuture::from(image_bitmap_promise).await
    {
        Ok(blob) => {
            Ok(blob.dyn_into::<ImageBitmap>().unwrap())
        }

        Err(err) => {
            let err_msg = format!("{} can't be loaded, {:#?}", file_path, err);
            Err(err_msg)
        }
    };

    result
}

pub async fn load_images(files: Vec<gloo_file::File>) -> Result<Vec<Image>, String> {
    let mut images = Vec::new();

    for file in files.into_iter() {
        let data = gloo_file::futures::read_as_data_url(&file).await.unwrap();

        match fetch_image(data.as_str()).await {
            Ok(data) => {
                images.push(Image::new(&file.name(), Some(data.clone()), data.width() as f64, data.height() as f64));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(images)
}

pub async fn load_json(files: Vec<gloo_file::File>) -> Result<String, String> {
    for file in files.into_iter() {
        match gloo_file::futures::read_as_text(&file).await {
            Ok(data) => return Ok(data),
            Err(err) => return Err(format!("{:#?}", err))
        };
    }

    Ok(String::from("Read Json Failded!"))
}