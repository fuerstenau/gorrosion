# Gorrosion

[![Build status][travis-badge]][travis-link]
[![Coverage status][coveralls-badge]][coveralls-link]
[![Crates.io][crates.io-badge]][crates.io-link]
[![Documentation][docs.rs-badge]][docs.rs-link]

## What is this?

A Go/weiqi/igo/baduk/goe engine written in Rust.
(As should have been obvious from the punny name.)

## How does it work?

Math.

## What are the alternatives?

Searching for “Go” on crates.io gives roughly a bazillion hits,
most of them not about the game.
So far, I was able to unearth the following rivals:
* [joseki](https://crates.io/crates/joseki) (Supports SGF)
* [libgo](https://crates.io/crates/libgo) (Supports GTP)
* [baduk](https://crates.io/crates/baduk) (Supports SGF)
* [go-bag](https://crates.io/crates/go-bag) (No features so far)

They mostly seem to be unmaintained, though.
(And they have less fun approaches to the necessary calculations.)

## Anything else you want to say?

One day in the future I might write some useful documentation.
Until then, feel free to e-mail me with questions.
(Well, after then, too, but not before consulting the docs.)

[travis-link]: https://travis-ci.org/fuerstenau/gorrosion
[travis-badge]: https://api.travis-ci.org/fuerstenau/gorrosion.svg?branch=master
[coveralls-link]: https://coveralls.io/github/fuerstenau/gorrosion?branch=master
[coveralls-badge]: https://coveralls.io/repos/github/fuerstenau/gorrosion/badge.svg?branch=master
[crates.io-link]: https://crates.io/crates/gorrosion
[crates.io-badge]: https://img.shields.io/crates/v/gorrosion.svg
[docs.rs-link]: https://docs.rs/gorrosion
[docs.rs-badge]: https://docs.rs/gorrosion/badge.svg
