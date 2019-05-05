use crate::drawing_style::DrawingStyle;
use crate::fig::diagram::Diagram;
use serde::{Deserialize, Serialize};
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Serialize)]
pub struct SerializableAppState<'a> {
    pub diagram: &'a Diagram,
    pub style: &'a DrawingStyle,
}

pub fn serialize(app_state: &SerializableAppState) -> String {
    let s: String = match serde_json::to_string(&app_state) {
        Ok(s) => s,
        Err(e) => panic!("error serializing app state {:?}", e),
    };
    return s;
}

#[derive(Debug, Deserialize)]
pub struct DeserializedAppState {
    pub diagram: Diagram,
    pub style: DrawingStyle,
}

fn get_dots_config_string(document_content: &str) -> Result<String, String> {
    let parser = EventReader::from_str(document_content);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { attributes, .. }) => {
                for attr in attributes {
                    if attr.name.local_name == "config" {
                        match attr.name.prefix {
                            Some(x) => {
                                if x == "dots" {
                                    return Ok(attr.value);
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
            Err(e) => {
                return Err(e.to_string());
            }
            _ => {}
        }
    }
    return Err(String::from(
        "did not encounter xml attribute dots:config in string",
    ));
}

pub fn get_state_from_document_string(
    document_content: &str,
) -> Result<DeserializedAppState, String> {
    let app_state: DeserializedAppState = match get_dots_config_string(document_content) {
        Ok(config_string) => match serde_json::from_str(&config_string) {
            Ok(app_state) => app_state,
            Err(e) => {
                return Err(e.to_string());
            }
        },
        Err(e) => {
            return Err(e);
        }
    };

    return Ok(app_state);
}
