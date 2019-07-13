# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased


## 0.5.0 - 2019-07-13

## Changed
 - Put all parser functions in single `parser` module.

## 0.4.0 - 2019-07-13

### Added
 - Expose all levels of parser. This is useful when parsing individual events, rather than a file.

### Changed
 - Rewrote parsers using nom 5 (no more macros!)
 - Midi file renamed from `Midi` to `SimpleMidiFile`.


## 0.2.0

### Added
 - Expose fields as public where they should be
