use std::path::{Path, PathBuf};

use dirs::{
    cache_dir,
    state_dir,
    data_dir,
    config_dir
};

pub struct Config<P: AsRef<Path>> {
    path: P,

}