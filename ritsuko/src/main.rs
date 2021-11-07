
use std::{fs::OpenOptions, io::Write, os::windows::prelude::{FromRawHandle, IntoRawHandle, OpenOptionsExt}};

use mio::windows::NamedPipe;

mod proto;

fn main() {
    let mut opts = OpenOptions::new();
    opts.read(true)
        .write(true)
        .custom_flags(1073741824);

    let file = opts.open(r"\\.\pipe\magi-workspace-manager").unwrap();
    let named_pipe = unsafe { NamedPipe::from_raw_handle(file.into_raw_handle()) };

    named_pipe.write(buf)
}
