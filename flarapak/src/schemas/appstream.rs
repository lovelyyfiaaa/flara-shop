use serde_derive::{Deserialize, Serialize};
/**
*/

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct App {
    pub r#type: String,
    #[serde(rename = "$unflatten=id")]
    pub id: String,
    #[serde(rename = "$unflatten=name")]
    pub name: String,
    #[serde(rename = "$unflatten=developer_name")]
    pub developer_name: Option<String>,
    #[serde(default, rename = "$unflatten=url")]
    pub urls: Vec<Url>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Url {
    pub r#type: String,
    #[serde(rename = "$value")]
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Components {
    #[serde(default, rename = "$unflatten=component")]
    pub components: Vec<App>,
}

#[cfg(test)]
mod tests {
    use super::Components;
    #[test]
    pub fn parse_appstream() {
        let appstream = include_str!("./appstream_test.xml");
        quick_xml::de::from_str::<Components>(appstream).unwrap();
    }
}
