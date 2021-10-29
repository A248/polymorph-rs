/*
 * Copyright © 2021 Anand Beh
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::ops::{Deref, DerefMut};
use std::borrow::{Borrow, BorrowMut};
use ref_or_owned_macros::*;
use std::fmt::{Display, Formatter};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

//!
//! Contains abstractions over references and ownership. Provides types
//! which may represent either a borrowed reference or an owned value.
//!
//! Both sized and unsized types may be used. References may be either mutable
//! or immutable. The right enum should be chosen on these bases.
//!

/// A type which can be either an immutable reference, or an owned value.
/// RefOrOwned requires sized types. For unsized types, use `RefOrBox` instead.
///
/// One prominent use case for `RefOrOwned` is in function return types,
/// which lets implementations be free to decide whether to return owned or borrowed values.
///
/// ```rust
/// # use polymorph::ref_or_owned::RefOrOwned;
/// struct MyStruct {}
///
/// fn func(my_struct: &MyStruct) -> RefOrOwned<'_, MyStruct> {
///     RefOrOwned::Borrowed(my_struct)
/// }
/// ```
///
/// The type implements `Deref` for `T`, allowing one to use it where
/// `&T` would be required. It also implements `From<&T>` and `From<T>`,
/// which enables ergonomic use in function parameters.
///
/// ```rust
/// # use polymorph::ref_or_owned::RefOrOwned;
///
/// struct MyStruct {}
/// impl MyStruct {
///   fn my_func(&self) -> u8 {
///     2
///   }
/// }
///
/// fn run_func<'t, T>(my_struct: T) -> u8
///   where T: Into<RefOrOwned<'t, MyStruct>> {
///
///   let my_struct = my_struct.into();
///   // my_struct now has type RefOrOwned<'t, MyStruct>
///   my_struct.my_func()
/// }
/// ```
#[derive(Debug)]
pub enum RefOrOwned<'t, T: 't> {
    Borrowed(&'t T),
    Owned(T)
}

impl<'t, T> From<&'t T> for RefOrOwned<'t, T> {
    fn from(value: &'t T) -> Self {
        Self::Borrowed(value)
    }
}

ref_or_owned_impls!(RefOrOwned);

/// A type which can be either a mutable reference, or an owned value.
/// RefMutOrOwned requires sized types. For unsized types, use `RefMutOrBox` instead.
///
/// One prominent use case for `RefMutOrOwned` is in function return types,
/// which lets implementations be free to decide whether to return owned or borrowed values.
///
/// ```rust
/// # use polymorph::ref_or_owned::RefMutOrOwned;
/// struct MyStruct {}
///
/// fn func(my_struct: &mut MyStruct) -> RefMutOrOwned<'_, MyStruct> {
///     RefMutOrOwned::Borrowed(my_struct)
/// }
/// ```
///
/// The type implements `Deref` and `DerefMut` for `T`, allowing one to use it where
/// `&mut T` would be required. It also implements `From<&mut T>` and `From<T>`,
/// which enables ergonomic use in function parameters.
///
/// ```rust
/// # use polymorph::ref_or_owned::RefMutOrOwned;
///
/// struct MyStruct {}
/// impl MyStruct {
///   fn my_func(&mut self) -> u8 {
///     2
///   }
/// }
///
/// fn run_func<'t, T>(my_struct: T) -> u8
///   where T: Into<RefMutOrOwned<'t, MyStruct>> {
///
///   let mut my_struct = my_struct.into();
///   // my_struct now has type RefMutOrOwned<'t, MyStruct>
///   my_struct.my_func()
/// }
/// ```
#[derive(Debug)]
pub enum RefMutOrOwned<'t, T: 't> {
    Borrowed(&'t mut T),
    Owned(T)
}

impl<'t, T> From<&'t mut T> for RefMutOrOwned<'t, T> {
    fn from(value: &'t mut T) -> Self {
        Self::Borrowed(value)
    }
}

impl<T> DerefMut for RefMutOrOwned<'_, T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Borrowed(borrowed_value) => *borrowed_value,
            Self::Owned(owned_value) => owned_value
        }
    }
}

impl<T> AsMut<T> for RefMutOrOwned<'_, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

impl<T> BorrowMut<T> for RefMutOrOwned<'_, T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

ref_or_owned_impls!(RefMutOrOwned);

/// A type which can be either an immutable reference, or an owned boxed value.
/// Box is used for the owned variant because this type is primarily intended for
/// use with unsized types, most particularly trait objects. For sized types,
/// it is strongly suggested to use `RefOrOwned` instead.
///
/// One prominent use case for `RefOrBox` is in function return types,
/// which lets implementations be free to decide whether to return owned or borrowed values.
///
/// ```rust
/// # use polymorph::ref_or_owned::RefOrBox;
/// trait MyTrait {}
///
/// fn func<'a>(my_trait: &'a dyn MyTrait) -> RefOrBox<'a, dyn MyTrait> {
///     RefOrBox::Borrowed(my_trait)
/// }
/// ```
///
/// The type implements `Deref` for `T`, allowing one to use it where
/// `&T` would be required. It also implements `From<&T>` and `From<Box<T>>`,
/// which enables ergonomic use in function parameters.
///
/// ```rust
/// # use polymorph::ref_or_owned::RefOrBox;
///
/// trait MyTrait {
///   fn my_func(&self) -> u8;
/// }
///
/// fn run_func<'t, T>(my_trait: T) -> u8
///   where T: Into<RefOrBox<'t, dyn MyTrait>> {
///
///   let my_trait = my_trait.into();
///   // my_trait now has type RefOrBox<'t, dyn MyTrait>
///   my_trait.my_func()
/// }
/// ```
#[derive(Debug)]
pub enum RefOrBox<'t, T: ?Sized + 't> {
    Borrowed(&'t T),
    Owned(Box<T>)
}

impl<'t, T: ?Sized> From<&'t T> for RefOrBox<'t, T> {
    fn from(value: &'t T) -> Self {
        Self::Borrowed(value)
    }
}

ref_or_box_impls!(RefOrBox);

/// A type which can be either a mutable reference, or an owned boxed value.
/// Box is used for the owned variant because this type is primarily intended for
/// use with unsized types, most particularly trait objects. For sized types,
/// it is strongly suggested to use `RefMutOrOwned` instead.
///
/// One prominent use case for `RefMutOrBox` is in function return types,
/// which lets implementations be free to decide whether to return owned or borrowed values.
///
/// ```rust
/// # use polymorph::ref_or_owned::RefMutOrBox;
/// trait MyTrait {}
///
/// fn func<'a>(my_trait: &'a mut dyn MyTrait) -> RefMutOrBox<'a, dyn MyTrait> {
///     RefMutOrBox::Borrowed(my_trait)
/// }
/// ```
///
/// The type implements `Deref` and `DerefMut` for `T`, allowing one to use it where
/// `&mut T` would be required. It also implements `From<&mut T>` and `From<Box<T>>`,
/// which enables ergonomic use in function parameters.
///
/// ```rust
/// # use polymorph::ref_or_owned::RefMutOrBox;
///
/// trait MyTrait {
///   fn my_func(&mut self) -> u8;
/// }
///
/// fn run_func<'t, T>(my_trait: T) -> u8
///   where T: Into<RefMutOrBox<'t, dyn MyTrait>> {
///
///   let mut my_trait = my_trait.into();
///   // my_trait now has type RefMutOrBox<'t, dyn MyTrait>
///   my_trait.my_func()
/// }
/// ```
#[derive(Debug)]
pub enum RefMutOrBox<'t, T: ?Sized + 't> {
    Borrowed(&'t mut T),
    Owned(Box<T>)
}

impl<'t, T: ?Sized> From<&'t mut T> for RefMutOrBox<'t, T> {
    fn from(value: &'t mut T) -> Self {
        Self::Borrowed(value)
    }
}

impl<T: ?Sized> DerefMut for RefMutOrBox<'_, T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Borrowed(borrowed_value) => *borrowed_value,
            Self::Owned(owned_box) => owned_box.deref_mut()
        }
    }
}

impl<T: ?Sized> AsMut<T> for RefMutOrBox<'_, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

impl<T: ?Sized> BorrowMut<T> for RefMutOrBox<'_, T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

ref_or_box_impls!(RefMutOrBox);

#[cfg(test)]
#[path = "ref_or_owned_tests.rs"]
mod ref_or_owned_tests;

#[path = "ref_or_owned_macros.rs"]
#[macro_use]
mod ref_or_owned_macros;
