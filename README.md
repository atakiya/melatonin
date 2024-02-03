# melatonin - BYOND version manager

A version manager for the [Build Your Own Net Dream (BYOND)](https://www.byond.com/) software.

## Installation

build from source via

```
cargo install melatonin
```

or, with [cargo binstall](https://github.com/cargo-bins/cargo-binstall)

```
cargo binstall melatonin
```

## Usage

To quickly get started from scratch:

1. `bvm setup` to setup the shims and PATH
2. `bvm install latest`
3. `cd your/byondproject/path/here`
4. `bvm pin latest`

And done!  
Now simply invoke your tools as usual, such as

-   `dm project.dme` - run the CLI compiler against your environment
-   `dd project.dmb` - run the CLI server against your built binary (BYOND 515+)
-   `dreammaker project.dme` - run the GUI editor/compiler

For the currently installed versions, run `bvm list`

For all commands and help, simply run `bvm --help`

## License

See [LICENSE.md](./LICENSE.md)

## Copyright notice

Copyright ©️ 2024 Avunia Takiya - All Rights Reserved  
Build Your Own Net Dream is Copyright ©️ BYOND Software
