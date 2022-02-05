use {
    rs9p::srv::srv_async_unix,
    std::path::PathBuf,
};

#[macro_use]
mod utils;
mod fs;


async fn unpfs_main(args: Vec<String>) -> rs9p::Result<i32> {
    if args.len() < 3 {
        eprintln!("Usage: {} proto!address!port mountpoint", args[0]);
        eprintln!("  where: proto = tcp | unix");
        return Ok(-1);
    }

    let (addr, mountpoint) = (&args[1], PathBuf::from(&args[2]));
    if !tokio::fs::metadata(&mountpoint).await?.is_dir() {
        return res!(io_err!(Other, "mount point must be a directory"));
    }

    println!("[*] Ready to accept clients: {}", addr);
    srv_async_unix(
        fs::Nixfs {
            realroot: mountpoint,
            readwrite: false,
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