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

and a port of 1234 (the default if you use `nixfsconnect`), launch like:

`nix-9p /tmp/nixfs-socket_1234 /nix/store`

## TODO

[ ] Profile to find performance hotspots
[ ] Refactor internals to track paths against an allowlist
[ ] Implement the ability to expose only a subset of the store
