# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## \[Unreleased\]

## \[0.2.1\] - 2023-12-09

### Changed

  - Rewrote the [readme](README.md) file and updated the examples.
  - Switched error handling from `std::error::Error` to `anyhow`.

## \[0.2.0\] - 2023-11-16

### Added

  - Added the GFDL (GNU Free Documentation License) to the [docs/](docs/) folder.
  - Added documentation for the JSON configuration.

### Changed

  - Instead of having hardcoded replacements, now they are determined by a JSON config file, modified by the user.

## \[0.1.0\] - 2023-10-13

### Added

  - Initial release of `file-normalizer-rs`.
