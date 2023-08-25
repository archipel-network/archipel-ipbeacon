# Archipel IP Beacon

> Neighbour discovery protocol implementation for [Archipel Core](https://github.com/EpicKiwi/archipel-core).

This is a modified version of [dtn7-rs](https://github.com/dtn7/dtn7-rs) neighbour discovery protocol implmentation. Adds contact to  [Archipel Core](https://github.com/EpicKiwi/archipel-core).

## Install

Make sure you have [installed rust](https://www.rust-lang.org/tools/install) on your system

Clone this repository (with optional --depth 1 parameter for smaller repository size)

```sh-session
git clone https://github.com/EpicKiwi/archipel-ipbeacon.git --depth 1
```

Build

```sh-session
cargo build --bin daemon --release
```

Install (as root)

```sh-session
./install.sh
```

Start system-wide neighbor discovery service (as root)

```sh-session
systemctl start archipel-ipbeacon
```

## Development

See [Protocol specs](./doc/0-protocol-specs.md) for a description of implemented protocol.