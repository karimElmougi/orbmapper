# orbmapper

Daemon to remap the Razer Orbweaver Chroma. Creates a virtual input device from
which the remapped keys will be sent.

## Installation

Simply download the latest binary on the [Release page] and put it in your path
like so:

```sh
$ curl -LO https://github.com/karimElmougi/orbmapper/releases/download/v0.1.0/orbmapper
$ chmod a+x orbmapper
$ sudo cp orbmapper /usr/bin/
```

## Installation from source

Note: Because orbmapper requires elevated priviledges, it needs to be installed
in a location that is accessible in the root user's PATH variable. This is why
a simple `cargo install` probably won't work.

```sh
$ git clone https://github.com/karimElmougi/orbmapper.git --branch v0.1.0
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

[Release page]: https://github.com/karimElmougi/orbmapper/releases
