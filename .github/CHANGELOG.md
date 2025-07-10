# Changelog

<!--
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).
-->

## [unreleased]

## [0.9.0] - 2025-07-10

### Added

- `-m` / `--mirror` flag to use the https://spacestation13.github.io/byond-builds mirror (#12)

## [0.8.0] - 2025-04-23

### Added

- `bvm prefix <VERSION>` command to display the install directory of the
  specified version (#10)

### Changed

- First installed version (while there is not other installed version) of BYOND will be set as the default global version (#10)
- `bvm uninstall <VERSION>` now accepts shorthand versions such as 'latest' (#10)
- Dependencies tagged on major version where applicable (#10)
- Reordered commands in the help output

### Fixed

- Global default version should now work properly (#10)

## [0.7.8] - 2024-03-11

Re-release of 0.7.7 code for CI/CD to build a container image.\
Now available at [ghcr.io/atakiya/melatonin](https://ghcr.io/atakiya/melatonin)

## [0.7.7] - 2024-03-05

### Added

- Linux support

## [0.6.4-beta] - 2024-02-06

### Added

- Feedback on missing BYOND tool
- Global (user) version pinning support via `bvm pin -g <version>`
- Indication of global and active version in `bvm list`
- Ability to supply path to `bvm pin`

## [0.5.0-beta] - 2024-02-03

### Added

- Windows support
- Download and installation of multiple BYOND versions
- Listing of installed versions
- Uninstallation of installed versions by version specifier
- Setup of environment
- Get and install latest stable or beta version without specifying version
- Pinning version per BYOND project
- Shimming of CLI executables

[unreleased]: https://github.com/atakiya/melatonin/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/atakiya/melatonin/releases/tag/v0.9.0
[0.8.0]: https://github.com/atakiya/melatonin/releases/tag/v0.8.0
[0.7.8]: https://github.com/atakiya/melatonin/releases/tag/v0.7.8
[0.7.7]: https://github.com/atakiya/melatonin/releases/tag/v0.7.7
[0.6.4-beta]: https://github.com/atakiya/melatonin/releases/tag/v0.6.4-beta
[0.5.0-beta]: https://github.com/atakiya/melatonin/releases/tag/v0.5.0-beta
