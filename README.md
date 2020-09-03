# orbmapper

Daemon to remap the Razer Orbweaver Chroma. Creates a virtual input device from which the 
remapped keys will be sent.

The mapping can be edited through the `KEY_MAP` constant in `main.rs`.

## Usage

1) Disable the Orbweaver's keyboard input with `xinput`

```sh
$ xinput list 
|
⎣ Virtual core keyboard                         id=3    [master keyboard (2)]
    ↳ Razer Razer Orbweaver Chroma System Control       id=20   [slave  keyboard (3)]
    ↳ Razer Razer Orbweaver Chroma Consumer Control     id=22   [slave  keyboard (3)]
    ↳ Razer Razer Orbweaver Chroma Keyboard     id=18   [slave  keyboard (3)]
    ↳ Razer Razer Orbweaver Chroma              id=19   [slave  keyboard (3)]
```

```sh
$ xinput disable 19
```

2) Start the remapper as root

```sh
$ sudo ./orbmapper
```
