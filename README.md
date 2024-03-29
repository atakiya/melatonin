# 💊 Melatonin - BYOND version manager

[![][badge-status-test]](https://github.com/atakiya/melatonin/actions)
[![][badge-crates-version]](https://crates.io/crates/melatonin)
[![][badge-crates-license]](./LICENSE.md)

A version manager for the [Build Your Own Net Dream (BYOND)](https://www.byond.com/) software.

Contributions and improvements are very welcome, as this is my first Rust-based project.

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

### Container Image

Container images are released along with semver tags  
Images are tagged as (example semver `v1.2.3`)

-   `v1.2.3` - full version
-   `v1.2` - major and minor
-   `v1` - major
-   `latest`

Example `Dockerfile` for a BYOND project previously pinned with melatonin:

```Dockerfile
FROM ghcr.io/atakiya/melatonin

WORKDIR /app

# Assuming you're building from your project's root
COPY . .

# Setup shims and install the in the project pinned version of BYOND
RUN bvm setup \
	bvm install $(cat .byondversion)\

# Compile the project
RUN DreamMaker -verbose *.dme

# And run a dedicated server!
ENTRYPOINT [ "DreamDaemon" ]
CMD [ "*.dmb", "5513", "-invisible", "-trusted" ]
```

## License

[![GNU General Public License v3 or later](https://www.gnu.org/graphics/gplv3-127x51.png)](https://www.gnu.org/licenses/gpl-3.0.html)
See [LICENSE.md](./LICENSE.md) for more information.

## Copyright notice

Copyright ©️ 2024 Avunia Takiya - All Rights Reserved  
Build Your Own Net Dream is Copyright ©️ BYOND Software

[badge-status-test]: https://img.shields.io/github/actions/workflow/status/atakiya/melatonin/test.yaml?label=tests "CI/CD tests status"
[badge-crates-version]: https://img.shields.io/crates/v/melatonin "Crates.io Version"
[badge-crates-license]: https://img.shields.io/crates/l/melatonin "Crates.io License"
