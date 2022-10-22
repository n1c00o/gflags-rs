# Gflags-rs

Rust implementation of [Google's Gflags](https://github.com/gflags/gflags).

## Introduction

Gflags, formerly Google Commandline Flags, is a useful library to implement and use
commandline flags in a software.

Commandline flags can be used when run an executable in the command, such as `foo --bar -B baz`.
Here, `--bar` and `-B` are two commandline flags nd `baz` is a commandline argument.

An application typically lists the flags that are allowed to pass and the argument they take.

At Google, Gflags allows engineers to scatter the flag definitions around the source code (instead
of listing all of them in `main()`), making it easy and clear at scale.

> See the original (and better) introduction written on [Gflag's online documentation](http://gflags.github.io/gflags/).

## Installation

TODO(n1c00o): add documentation

## License

Gflags-rs is governed by a BSD-style license which can be found in the [LICENSE](LICENSE) file.

The original [Gflags](https://github.com/gflags/gflags) is licensed under the BSD 3-Clause license.
