use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Welcome to Walnuts, your toy blockchain!")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Command
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Mine/create a block, and add it to Walnuts blockchain
    Mine,
    /// Fetch Walnuts
    Fetch,
    /// Send transaction From addr_A To addr_B
    From,
    To,
}
