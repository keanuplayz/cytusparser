mod core;

use crate::core::cytus2::TempoListItem;

use std::error::Error;

use serde_json::{json, to_string};

fn main() -> Result<(), Box<dyn Error>> {
    let test = json!({	"tick"	:	0	,	"value"	:	480000.0	});
    
    let test2 = to_string(&test)?;

    println!("JSON: {}", test2);

    let test3: TempoListItem = serde_json::from_str(&test2)?;

    println!("Thingy: {:?}", test3);

    let test4 = to_string(&test3)?;

    println!("JSON: {}", test4);

    Ok(())
}
