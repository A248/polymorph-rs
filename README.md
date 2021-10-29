
# Polymorph

[![crates.io version](https://img.shields.io/crates/v/polymorph)](https://crates.io/crates/polymorph)
[![apache2 license](https://img.shields.io/crates/l/polymorph)](https://www.gnu.org/licenses/license-recommendations.html)
[![docs.rs docs](https://img.shields.io/docsrs/polymorph)](https://docs.rs/polymorph)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

A set of utilities to better enable polymorphic behavior in Rust.

## Introduction

Rust is a wonderful language, with a strong emphasis on fast, static dispatch and statically-determined memory management. This is amazing, but it is hard to ease into from the perspective of a dynamic language.

This crate seeks to enable more dynamic programming while adhering to the core principles of Rust: only pay for what you need, and incur little overhead when you do so.

* Want to decide whether to return an owned value or a borrowed reference? Now you can do so, without having to change your function's return type.
* Want to use trait objects but don't like the fact that `Box<dyn MyTrait>` requires ownership? Now you can choose whether to return a borrowed or an owned trait object.

## Features

### Ref(Mut)OrOwned

`RefOrOwned<T>` is an enum over `&T` and `T`. This is similar to `Cow` in that it abstracts over ownership and borrowing. However, while `Cow` has a `ToOwned` requirement, `RefOrOwned` does not.

* `RefOrOwned<T>` implements `Deref` to `T`, as well as a broader family of standard Rust traits, so that you can work with it ergonomically and painlessly.
* There is also a `RefMutOrOwned` version, for when you need `&mut T`.
* `into_owned` is available where `T: Clone`.
* The type also implements `From<&T>` and `From<T>`, so that you can use `Into<RefOrOwned<T>>` to create highly-flexible function parameter.

### Ref(Mut)OrBox

`RefOrBox<T>` is an enum over `&T` and `Box<T>`. While similar to `RefOrOwned`, `RefOrBox` is intended for cases where T is unsized, most notably when T is a trait object.

* `RefOrBox<T>` implements standard traits where possible, including `Deref` to `T`.
* Optional support for the [dyn-clone](https://crates.io/crates/dyn-clone) crate is provided by the **trait-clone** feature. If `T: DynClone`, an `into_owned` method will be made available. More on this later.

`RefMutOrBox` is a version of `RefOrBox` which uses `&mut T` and can be dereferenced to a mutable value.

### Safety

* The library contains no unsafe code
* The library should never panic

## Dependency

Add this library to your Cargo.toml:

```toml
[dependencies]
polymorph = "0.1"
```

### Features

All of the available [Cargo features](https://stackoverflow.com/questions/58480205/how-do-you-enable-a-rust-crate-feature) provided by this crate. Each of these features must be enabled independently if it is desired.

**Trait-Clone**

To enable interoperability with the **dyn-clone** trait, turn on this feature.

```toml
[dependencies]
polymorph = { version = "0.1", features = ["trait-clone"]}
```

This will add a `RefOrBox::into_owned` method which returns a `Box<T>`, either by returning the owned box or cloning a borrowed value.

## Other Information

### Composability

Polymorph is well-combined with the `dyn-clone` and `enum-dispatch` crates for flexible and effective dynamic programming.

### Notes

Polymorph's structs and enums are paid close attention to ensure they implement standard traits. Code interoperability and reuse is best achieved when standard traits are correctly implemented.

If you ever happen to notice a place where a standard trait could be implemented, please open an issue in this repository.

### Licensing

Licensed under the Apache License v2.0. See the LICENSE.txt.
