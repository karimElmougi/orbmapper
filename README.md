# orbmapper

Daemon to remap the Razer Orbweaver Chroma. Creates a virtual input device from
which the remapped keys will be sent.

## Installation from source

Because orbmapper requires elevated priviledges, it needs to be installed in a
location that is accessible in the root user's PATH variable.

```sh
$ git clone https://github.com/karimElmougi/orbmapper.git
$ cd orbmapper
$ cargo build --release
$ sudo cp target/release/orbmapper /usr/bin/
```

## Usage

Either use my personal key map like this:

```sh
$ orbmapper
```

or define your own in TOML and pass it around like so:

```sh
$ orbmapper --config config.toml
```

See [config.toml](config.toml) as an example.

That's it! Now your Orbweaver's keystrokes should be getting remapped according
to your defined key map.

