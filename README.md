Plain Data Structures
=====================

[![Latest version](https://img.shields.io/crates/v/plain-ds.svg)](https://crates.io/crates/plain-ds)
![License](https://img.shields.io/crates/l/plain-ds.svg)
[![Tests](https://github.com/dvshapkin/plain-ds/actions/workflows/ci.yaml/badge.svg)](https://github.com/dvshapkin/plain-ds/actions/workflows/ci.yaml)
[![Documentation](https://docs.rs/plain-ds/badge.svg)](https://docs.rs/plain-ds)

A set of simple data structures that have proven useful in my projects. Expanded as needed.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
plain-ds = "0.1"
```

Or via `cargo add`:

```bash
cargo add plain-ds
```

## What's new in last version?

### [0.2.0]

### Added
- `SortedList` implementation

### Fixed
- known bugs

## Overview

As already mentioned, `plain-ds` is a set of simple data structures (DS) that have proven useful in my projects.
These data structures may not be the most efficient and productive solutions, but they have proven to be appropriate 
in certain specific situations for various reasons. I plan to expand this set as needed, as well as improve the quality 
and efficiency of existing DS.

**Key principles**:
- **Reliability**: Bugs free code you can trust.
- **Safety**: Predictable memory consumption. No memory leaks.
- **Quality and hi-coverage of testing**: Completeness and thoughtfulness of tests.
- **Clarity**: Detailed error messages and up-to-date documentation.

## What has already been implemented?
- `SinglyLinkedList` - singly-linked list implementation
- `SortedList` - sorted list implementation

## Contributing

We welcome:
* Bug reports
* Feature requests
* Documentation improvements

## Contact & Links

* Repository: https://github.com/dvshapkin/plain-ds
* Issues: https://github.com/dvshapkin/plain-ds/issues
* Documentation: https://docs.rs/plain-ds