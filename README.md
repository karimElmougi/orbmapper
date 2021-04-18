# orbmapper

Daemon to remap the Razer Orbweaver Chroma. Creates a virtual input device from which the 
remapped keys will be sent.

The mapping can be edited through the `KEY_MAP` constant in `main.rs`.

## Installation

```sh
$ cargo install
```

## Usage

```sh
$ orbmapper
```

That's it! Now your Orbweaver's keystrokes should be getting remapped according to `KEY_MAP`.

