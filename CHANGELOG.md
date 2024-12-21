# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

---
## 0.4.0 - 2024-12-21

* Update to syntree 0.18.0
* Fix range error handling in embedder by formatting source output
* Adjust y2 attribute calculation in SvgDrawer to improve positioning

## 0.3.0 - 2024-09-07

* Update to syntree 0.17.4
* Provide a new Layouter APIs: `embed_with_source` and  `embed_with_source_and_display`. To be able
to use these APIs the node data doesn't need to implement `Virtualize`. See `example3` in the
examples folder:
```shell
cargo run --example example3
```
and then have a look at the generated `example3_1.svg` and `example3_2.svg` for the differences of
both variants.

## 0.2.0 - 2023-03-05

* Add CHANGELOG
* Update year in MIT license file
* Changed version format of syntree reference in Cargo.toml
* Cleaned up example2
* Merged PR #1 contributed by @udoprog. Thanks a lot!
  * This includes some minor API changes, thus we bump to version 0.2.0
* Modified .gitignore

## v0.1.0 - 2023-03-04

* First released version
