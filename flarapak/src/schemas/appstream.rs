// SPDX-FileCopyrightText: 2022 Fiana Fortressia
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use quick_xml::DeError;
use regex::Regex;

use serde::{
    de::{MapAccess, Visitor},
    Deserializer,
};
use serde_derive::{Deserialize, Serialize};

lazy_static::lazy_static! {
    /// See this regex in the playground:
    static ref PARSE_GLOB_PATTERN: Regex = Regex::new(r"(<.*?>(.*?)</(.*?)>)+").unwrap();
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct App {
    pub r#type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "developer_name")]
    pub developer_name: Option<Vec<String>>,
    #[serde(default, rename = "url")]
    pub urls: Vec<Url>,
    #[serde(rename = "screenshots")]
    pub screenshots: Screenshots,
    #[serde(rename = "description")]
    pub description: Option<MarkupText>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Description {}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ParagraphElement {
    p(MarkupText),
    ol(MarkupText),
    ul(MarkupText),
    code(String),
    em(String),
    String(MarkupText),
    Literal(String),
}

impl<'de> serde::Deserialize<'de> for MarkupText {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MarkupTextVisitor;
        impl<'de> Visitor<'de> for MarkupTextVisitor {
            type Value = MarkupText;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string")
            }
            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut elements = Vec::new();
                // into our map.

                while let Some((key, value)) = access.next_entry::<String, String>()? {
                    println!("{key}, {value}");
                    if key == "$value" {
                    } else if key == "code" || key == "em" {
                    } else if key == "$value" || key == "ol" || key == "ul" || key == "p" {
                        use ParagraphElement as E;
                        let element = if key == "$value" {
                            E::String
                        } else if key == "p" {
                            E::p
                        } else if key == "ol" {
                            E::ol
                        } else if key == "ul" {
                            E::ul
                        } else {
                            println!("{key}, {value}");
                            // Key must be either ul, ol, or $value
                            unreachable!()
                        };

                        let matches: Vec<&str> = PARSE_GLOB_PATTERN
                            .find_iter(&value)
                            .map(|r#match| r#match.as_str())
                            .collect();
                        if matches.len() == 0 {
                            elements.push(element(MarkupText {
                                elements: vec![ParagraphElement::Literal(value)],
                            }));
                        } else {
                            let markup = quick_xml::de::from_str::<MarkupText>(&value);

                            match markup {
                                Ok(markup) => {
                                    elements.push(element(markup));
                                }

                                Err(err) => {
                                    return Err(serde::de::Error::custom(err.to_string()));
                                }
                            }
                        }
                    } else {
                        println!("{key}, {value}");
                        unreachable!()
                    }
                }

                Ok(MarkupText { elements })
            }
        }
        deserializer.deserialize_any(MarkupTextVisitor)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TableElements {
    p(Vec<MarkupText>),
    code(String),
    em(String),

    String(String),
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Table {
    #[serde(rename = "li")]
    pub items: Vec<TableElements>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MarkupText {
    #[serde(rename = "$value")]
    pub elements: Vec<ParagraphElement>,
}

impl MarkupText {
    /*  pub fn parse_markups(&self) -> Vec<ParagraphElement> {
            self.elements

        }
    */
    pub fn parse_markup(str: &str) -> ParagraphElement {
        let patterns: Vec<&str> = PARSE_GLOB_PATTERN
            .find_iter(str)
            .map(|r#match| r#match.as_str())
            .collect();

        panic!("{patterns:#?}");
    }
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Screenshots {
    #[serde(default, rename = "screenshot")]
    pub screenshots: Vec<Screenshot>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Screenshot {
    #[serde(default, rename = "image")]
    pub images: Vec<Image>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Image {
    pub r#type: String,
    #[serde(default, rename = "$value")]
    pub url: String,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Url {
    pub r#type: String,
    #[serde(rename = "$value")]
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Components {
    #[serde(default, rename = "component")]
    pub components: Vec<App>,
    pub origin: String,
}

pub fn parse_from_str(str: &str) -> Result<Components, DeError> {
    quick_xml::de::from_str::<Components>(str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schemas::appstream::App;
    use ParagraphElement::*;
    #[test]
    pub fn parse_appstream() {
        let appstream = include_str!("./appstream_test.xml");
        let components = parse_from_str(appstream).unwrap();

        assert_eq!(
            components,
            Components {
                components: vec![App { r#type: "desktop".to_string(), id: "com.inochi2d.inochi-creator".to_string(), name: "Inochi Creator".to_string(), developer_name: Some(vec!["Luna Nielsen".to_string()]), urls: vec![Url { r#type: "bugtracker".to_string(), url: "https://github.com/Inochi2D/inochi-creator/issues".to_string() }, Url { r#type: "contribute".to_string(), url: "https://github.com/Inochi2D/inochi-creator".to_string() }, Url { r#type: "donation".to_string(), url: "https://inochi2d.com/#donate".to_string() }, Url { r#type: "help".to_string(), url: "https://discord.com/invite/abnxwN6r9v".to_string() }, Url { r#type: "homepage".to_string(), url: "https://lunafoxgirlvt.itch.io/inochi-creator".to_string() }, Url { r#type: "vcs-browser".to_string(), url: "https://github.com/Inochi2D/inochi-creator".to_string() }], screenshots: Screenshots { screenshots: vec![Screenshot { images: vec![Image { r#type: "source".to_string(), url: "https://user-images.githubusercontent.com/7032834/194462402-74c4a3e0-50ca-4b50-8e8d-164d97371f5a.png".to_string() }, Image { r#type: "thumbnail".to_string(), url: "https://dl.flathub.org/repo/screenshots/com.inochi2d.inochi-creator-stable/624x351/com.inochi2d.inochi-creator-9d1a0bb40d21203453edf0491e0574a3.png".to_string() }, Image { r#type: "thumbnail".to_string(), url: "https://dl.flathub.org/repo/screenshots/com.inochi2d.inochi-creator-stable/1248x702/com.inochi2d.inochi-creator-9d1a0bb40d21203453edf0491e0574a3.png".to_string() }, Image { r#type: "thumbnail".to_string(), url: "https://dl.flathub.org/repo/screenshots/com.inochi2d.inochi-creator-stable/112x63/com.inochi2d.inochi-creator-9d1a0bb40d21203453edf0491e0574a3.png".to_string() }, Image { r#type: "thumbnail".to_string(), url: "https://dl.flathub.org/repo/screenshots/com.inochi2d.inochi-creator-stable/224x126/com.inochi2d.inochi-creator-9d1a0bb40d21203453edf0491e0574a3.png".to_string() }, Image { r#type: "thumbnail".to_string(), url: "https://dl.flathub.org/repo/screenshots/com.inochi2d.inochi-creator-stable/752x423/com.inochi2d.inochi-creator-9d1a0bb40d21203453edf0491e0574a3.png".to_string() }, Image { r#type: "thumbnail".to_string(), url: "https://dl.flathub.org/repo/screenshots/com.inochi2d.inochi-creator-stable/1504x846/com.inochi2d.inochi-creator-9d1a0bb40d21203453edf0491e0574a3.png".to_string() }] }] }, description: Some(MarkupText { elements: vec![p(MarkupText { elements: vec![Literal("Inochi2D is a framework for realtime 2D puppet animation which can be used for VTubing, game development and digital animation.".to_string())] }), p(MarkupText { elements: vec![Literal("Inochi Creator is a tool that lets you create and edit Inochi2D puppets.".to_string())] })] }) }],
                origin: "flarapak-test".to_string()
            }
        )
    }
}
