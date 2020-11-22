use crate::block::Block;
use std::fs::File;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Error};
use std::io::ErrorKind;

#[derive(Serialize, Deserialize, Debug)]
pub struct Walnuts {
    blocks: Vec<Block>,
}

impl Walnuts {
    pub fn load() -> Result<Self> {
	let file = File::open("walnutsdata.json");
	match file {
	    Ok(file) => { 
		Ok(serde_json::from_reader(&file)?)
	    },
	    Err(e) => {
		if e.kind() == ErrorKind::NotFound {
		    Ok(Walnuts::new())
		} else {		 
		    Err(Error::from(e))
		}
	    }	    
	}
    }

    fn new() -> Self {
	Walnuts { blocks: Vec::new() }
    }

    
    pub fn updatewalnuts(&mut self, block: Block) -> Result<()> {
	self.blocks.push(block);
	Ok(self.write_to_database()?)
    }
    
    fn write_to_database(&mut self) -> Result<()> {
	let file = File::create("walnutsdata.json")?;
	serde_json::ser::to_writer_pretty(file, &self)?;

	Ok(())
    }
}
