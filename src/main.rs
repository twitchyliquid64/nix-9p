use radix_trie::Trie;
use {rs9p::srv::srv_async_unix, std::path::PathBuf};

#[macro_use]
mod utils;
mod fs;

fn read_manifest(path: String) -> Result<Trie<String, ()>, std::io::Error> {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut out = Trie::new();

    for line in reader.lines() {
        let line = line?;
        out.insert(line.clone(), ());
    }

    Ok(out)
}

async fn unpfs_main(args: Vec<String>) -> rs9p::Result<i32> {
    if args.len() < 3 {
        eprintln!(
            "Usage: {} <socket-path> <mountpoint> [<path-manifest>]",
            args[0]
        );
        return Ok(-1);
    }

    let (addr, mountpoint) = (&args[1], PathBuf::from(&args[2]));
    if !tokio::fs::metadata(&mountpoint).await?.is_dir() {
        return res!(io_err!(Other, "mount point must be a directory"));
    }
    if !mountpoint.is_absolute() {
        return res!(io_err!(
            Other,
            "mount point must be provided as an absolute path"
        ));
    }

    let mut path_allowlist: Option<Trie<String, ()>> = None;
    if args.len() >= 4 {
        path_allowlist = Some(read_manifest(args[3].clone()).unwrap())
    }

    println!("[*] Ready to accept clients: {}", addr);
    srv_async_unix(
        fs::Nixfs {
            realroot: mountpoint,
            readwrite: false,
            path_allowlist,
        },
        addr,
    )
    .await
    .and(Ok(0))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = std::env::args().collect();
    let exit_code = unpfs_main(args).await.unwrap_or_else(|e| {
        eprintln!("Error: {:?}", e);
        -1
    });

    std::process::exit(exit_code);
}
