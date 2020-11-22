use crate::block::Block;
use crate::walnuts::Walnuts;
use structopt::StructOpt;

mod walnuts;
mod block;
mod blockheader;
mod transaction;
mod cli;

fn main() {
    let opt = cli::Opt::from_args();
    println!("{:#?}", opt);

    match opt.cmd {
	cli::Command::Mine => {
	    // Init the genesis block
	    let genesis_block = Block::genesis_init();
	    // Create Walnuts
	    let mut mywalnuts = Walnuts::load().unwrap();
	    println!("check mine funtion: {:?}", Walnuts::updatewalnuts(&mut mywalnuts, genesis_block));
	    println!("print mywalnuts: {:#?}", Walnuts::load());	    
	}
	cli::Command::Fetch => {
	    println!("Hello Fetch!");
	}
	cli::Command::From => {
	    println!("Send money from A to B:");
	}
	cli::Command::To => {
	    println!("Send money from A to B:");
	}
    }
    
}
