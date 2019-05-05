use crate::drawing_style::{DrawingStyle};
use crate::fig::diagram::Diagram;
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct SerializableAppState<'a> {
    pub diagram: &'a Diagram,
    pub style: &'a DrawingStyle,
}

pub fn serialize(app_state: &SerializableAppState) -> String {
    let s:String = match serde_json::to_string(&app_state) {
        Ok(s) => s,
        Err(e) => panic!("error serializing app state {:?}", e),
    };
    return s;
}