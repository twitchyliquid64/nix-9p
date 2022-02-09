# nix-9p

Runs a 9p filesystem server on a unix domain socket, so you can
expose a subset of the hosts' nix store to Firecracker VMs.

## Usage

Assuming a firecracker vsock configuration of:

```json
  "vsock": {
    "vsock_id": "fs",
    "uds_path": "/tmp/nixfs-socket",
    "guest_cid": 3
  }
```

and a port of 1234 (the default if you use [`mk-init`](https://github.com/twitchyliquid64/minikernel/blob/c90ea0e9e6f0959a922433d95ec3e6478fe45323/mk-init.go#L117) ), launch like:

`nix-9p /tmp/nixfs-socket_1234 /nix/store`

If you run it like the above, your exposing the entire nix store to your VM. Instead, you can pass a path to a file listing path prefixes which should be visible/readable.

## TODO

 * [x] Profile to find performance hotspots
 * [x] Implement the ability to expose only a subset of the store
