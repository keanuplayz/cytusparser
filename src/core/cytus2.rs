use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub struct TempoListItem {
    tick: u32,
    value: f32
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(i8)]
pub enum ScanLineDirection {
    Down = -1,
    Up = 1
}

impl ScanLineDirection {
    pub fn up() -> Self {
        Self::Up
    }

    pub fn down() -> Self {
        Self::Down
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    scan_line_direction: ScanLineDirection,
    start_tick: u32,
    end_tick: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    id: u32,
    #[serde(rename = "type")]
    note_type: u32,
    x: f32,
    tick: u32,
    hold_tick: u32,
    page_index: u32,
    next_id: u32,
    has_sibiling: Option<bool>,
    is_forward: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    event_type: u32,
    args: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventOrderItem {
    tick: u32,
    event_list: Vec<Event>
}

/*#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    id: u32,
    note_type: NoteType
    x: f32,
    tick: u32,
    page_index: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NoteType {
    Tap,
    Hold { hold_tick: u32 },
    LongHold { hold_tick: u32 },
    DragHead { next_id: u32 },
    DragChild { next_id: u32 },
    Flick
}*/

/// A chart in the Cytus 2 format
#[derive(Debug, Serialize, Deserialize)]
pub struct Chart {
    #[serde(alias = "format version")]
    format_version: Option<u32>,
    time_base: u32,
    start_offset_time: u32,
    tempo_list: Vec<TempoListItem>,
    page_list: Vec<Page>,
    note_list: Vec<Note>,
    event_order_list: Option<Vec<EventOrderItem>>
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::f32;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !(($x - $y).abs() < $d) { panic!(); }
        }
    }

    #[test]
    fn test_tempo_list_item() {
        let tick = 0;
        let value: f32 = 480000.0;

        let test = json!({ "tick": tick, "value": 480000.0 });
    
        let test2 = serde_json::to_string(&test)
            .expect("Couldn't serialize test JSON");
    
        let test3: TempoListItem = serde_json::from_str(&test2).unwrap();

        assert_eq!(tick, test3.tick);
        assert_delta!(value, test3.value, f32::EPSILON)
    }
}