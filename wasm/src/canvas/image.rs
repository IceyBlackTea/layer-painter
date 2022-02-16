/*
 * @Author: IceyBlackTea
 * @Date: 2022-02-03 20:22:10
 * @LastEditors: IceyBlackTea
 * @LastEditTime: 2022-02-08 01:12:46
 * @FilePath: /layer-painter/wasm/src/canvas/image.rs
 * @Description: Copyright © 2021 IceyBlackTea. All rights reserved.
 */

use std::fmt;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use web_sys::ImageBitmap;

#[derive(Debug, Clone)]
pub struct Image {
    name: String,
    data: Option<ImageBitmap>,
    sx: f64,
    sy: f64,
    sw: f64,
    sh: f64,
}

impl Image {
    pub fn empty() -> Self {
        Image {
            name: String::new(),
            data: None,
            sx: 0.0,
            sy: 0.0,
            sw: 0.0,
            sh: 0.0,
        }
    }

    pub fn new(name: &str, data: Option<ImageBitmap>, sw: f64, sh: f64) -> Self {
        Image {
            name: String::from(name),
            data,
            sx: 0.0,
            sy: 0.0,
            sw,
            sh,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn data(&self) -> Option<ImageBitmap> {
        self.data.clone()
    }

    pub fn set_data(&mut self, data: &ImageBitmap) {
        self.data = Some(data.clone());
    }

    pub fn sx(&self) -> f64 {
        self.sx
    }

    pub fn set_sx(&mut self, sx: f64) {
        self.sx = sx;
    }

    pub fn sy(&self) -> f64 {
        self.sy
    }

    pub fn set_sy(&mut self, sy: f64) {
        self.sy = sy;
    }

    pub fn sw(&self) -> f64 {
        self.sw
    }

    pub fn set_sw(&mut self, sw: f64) {
        self.sw = sw;
    }

    pub fn sh(&self) -> f64 {
        self.sh
    }

    pub fn set_sh(&mut self, sh: f64) {
        self.sh = sh;
    }
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Image", 5)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("sx", &self.sx)?;
        s.serialize_field("sy", &self.sy)?;
        s.serialize_field("sw", &self.sw)?;
        s.serialize_field("sh", &self.sh)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Name,
            Sx,
            Sy,
            Sw,
            Sh,
        }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`name`, `sx`, `sy`, `sw` or `sh`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "name" => Ok(Field::Name),
                            "sx" => Ok(Field::Sx),
                            "sy" => Ok(Field::Sy),
                            "sw" => Ok(Field::Sw),
                            "sh" => Ok(Field::Sh),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ImageVisitor;

        impl<'de> Visitor<'de> for ImageVisitor {
            type Value = Image;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Image")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Image, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let name = match seq.next_element().unwrap() {
                    Some(name) => name,
                    None => String::from("")
                };

                let sx = match seq.next_element().unwrap() {
                    Some(sx) => sx,
                    None => 0.0
                };

                let sy = match seq.next_element().unwrap() {
                    Some(sy) => sy,
                    None => 0.0
                };

                let sw = match seq.next_element().unwrap() {
                    Some(sw) => sw,
                    None => 0.0
                };
                let sh = match seq.next_element().unwrap() {
                    Some(sh) => sh,
                    None => 0.0
                };

                Ok(Image {
                    name,
                    data: None,
                    sx,
                    sy,
                    sw,
                    sh,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Image, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut sx = None;
                let mut sy = None;
                let mut sw = None;
                let mut sh = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }

                        Field::Sx => {
                            if sx.is_some() {
                                return Err(de::Error::duplicate_field("sx"));
                            }
                            sx = Some(map.next_value()?);
                        }

                        Field::Sy => {
                            if sy.is_some() {
                                return Err(de::Error::duplicate_field("sy"));
                            }
                            sy = Some(map.next_value()?);
                        }
                        Field::Sw => {
                            if sw.is_some() {
                                return Err(de::Error::duplicate_field("sw"));
                            }
                            sw = Some(map.next_value()?);
                        }
                        Field::Sh => {
                            if sh.is_some() {
                                return Err(de::Error::duplicate_field("sh"));
                            }
                            sh = Some(map.next_value()?);
                        }
                    }
                }
                let name = match name {
                    Some(name) => name,
                    None => String::from("")
                };

                let sx = match sx {
                    Some(sx) => sx,
                    None => 0.0
                };

                let sy = match sy {
                    Some(sy) => sy,
                    None => 0.0
                };
                
                let sw = match sw {
                    Some(sw) => sw,
                    None => 0.0
                };
                
                let sh = match sh {
                    Some(sh) => sh,
                    None => 0.0
                };
                
                Ok(Image {
                    name,
                    data: None,
                    sx,
                    sy,
                    sw,
                    sh,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["name", "sx", "sy", "sw", "sh"];
        deserializer.deserialize_struct("Image", FIELDS, ImageVisitor)
    }
}
