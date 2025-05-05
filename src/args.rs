use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[clap(
    name = "made_in_heaven",
    rename_all = "kebab-case",
    rename_all_env = "screaming-snake"
)]
pub struct Args {
    #[clap(default_value = "0.0.0.0:10000", env)]
    pub host: SocketAddr,
}
