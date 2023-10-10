extern crate env_logger;
extern crate fuse;
extern crate libc;
extern crate time;

use opendal::Result;

use std::env;

use log;

mod dalfs;
mod inode;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let scheme_osstring = env::args_os().nth(2).expect("Need an OpenDAL scheme");
    let scheme = scheme_osstring.to_str().unwrap();
    let op = config::get_operator_from_env(&scheme)?;
    log::debug!("operator: {:?}", op);

    let fs = dalfs::DalFs {
        op: op,
        inodes: inode::InodeStore::new(0o550, 1000, 1000),  // Temporarilly hardcode
    };

    env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    fuse::mount(fs, &mountpoint, &[]).unwrap();

    Ok(())
}
