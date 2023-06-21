use clap::Parser;
use crate::utils::flags::structure::Args;
use lazy_static::lazy_static;

lazy_static! {
    static ref ARGS: Args = initiate();
}

fn initiate() -> Args {
    Args::parse()
}

pub fn get_arg() -> &'static Args {
    &*ARGS
}