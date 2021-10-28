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

use crate::ref_or_owned::*;
use std::cell::RefCell;
use std::error::Error;
use downcast_rs::{Downcast, impl_downcast};
use std::collections::hash_map::DefaultHasher;

trait MyTrait: Downcast {
    fn do_something(&self);
    fn do_mutable(&mut self);
}
impl_downcast!(MyTrait);

struct Implementor {
    calls: RefCell<u8>,
    mut_calls: u8
}
impl Implementor {
    fn calls(&self) -> u8 {
        *self.calls.borrow()
    }
    fn mut_calls(&self) -> u8 {
        self.mut_calls
    }
}
impl Default for Implementor {
    fn default() -> Self {
        Self {
            calls: RefCell::new(0),
            mut_calls: 0
        }
    }
}
impl MyTrait for Implementor {
    fn do_something(&self) {
        self.calls.replace_with(|&mut counter| counter + 1);
    }

    fn do_mutable(&mut self) {
        self.mut_calls += 1;
    }
}

struct TestRefOrOwned<'t> {
    implementor: RefOrOwned<'t, Implementor>
}
impl<'t> TestRefOrOwned<'t> {
    fn new<T>(implementor: T) -> Self where T: Into<RefOrOwned<'t, Implementor>> {
        Self {
            implementor: implementor.into()
        }
    }
}

struct TestRefMutOrOwned<'t> {
    implementor: RefMutOrOwned<'t, Implementor>
}
impl<'t> TestRefMutOrOwned<'t> {
    fn new<T>(implementor: T) -> Self where T: Into<RefMutOrOwned<'t, Implementor>> {
        Self {
            implementor: implementor.into()
        }
    }
}

struct TestRefOrBox<'t> {
    my_trait: RefOrBox<'t, dyn MyTrait>
}
impl<'t> TestRefOrBox<'t> {
    fn new<T>(my_trait: T) -> Self where T: Into<RefOrBox<'t, dyn MyTrait>>  {
        Self {
            my_trait : my_trait.into()
        }
    }
}

struct TestRefMutOrBox<'t> {
    my_trait: RefMutOrBox<'t, dyn MyTrait>
}
impl<'t> TestRefMutOrBox<'t> {
    fn new<T>(my_trait: T) -> Self where T: Into<RefMutOrBox<'t, dyn MyTrait>>  {
        Self {
            my_trait : my_trait.into()
        }
    }
}

fn downcast_to_implementor(implementor: Box<dyn MyTrait>) -> Implementor {
    match implementor.downcast::<Implementor>() {
        Ok(value) => *value,
        Err(_) => panic!("Wrong MyTrait implementation")
    }
}

//
// Implicit Deref
// From<&T>, From<&mut T>, From<T>, and From<Box<T>>
//

#[test]
fn test_ref_or_owned_with_ref() {
    let implementor = Implementor::default();
    let test_ref_or_owned = TestRefOrOwned::new(&implementor);
    implementor.do_something();
    test_ref_or_owned.implementor.do_something();
    assert_eq!(2, implementor.calls());
    assert_eq!(0, implementor.mut_calls());
}

#[test]
fn test_ref_or_owned_with_ownership() {
    let implementor = Implementor::default();
    let test_ref_or_owned = TestRefOrOwned::new(implementor);
    test_ref_or_owned.implementor.do_something();
    let implementor = test_ref_or_owned.implementor;
    assert_eq!(1, implementor.calls());
    assert_eq!(0, implementor.mut_calls());
}

#[test]
fn test_ref_mut_or_owned_with_ref() {
    let mut implementor = Implementor::default();
    let mut test_ref_or_owned = TestRefMutOrOwned::new(&mut implementor);
    test_ref_or_owned.implementor.do_mutable();
    assert_eq!(0, implementor.calls());
    assert_eq!(1, implementor.mut_calls());
}

#[test]
fn test_ref_mut_or_owned_with_ownership() {
    let implementor = Implementor::default();
    let mut test_ref_or_owned = TestRefMutOrOwned::new(implementor);
    test_ref_or_owned.implementor.do_mutable();
    let implementor = test_ref_or_owned.implementor;
    assert_eq!(0, implementor.calls());
    assert_eq!(1, implementor.mut_calls());
}

#[test]
fn test_ref_or_box_with_ref() {
    let implementor = Implementor::default();
    let test_ref_or_box = TestRefOrBox::new(&implementor as &dyn MyTrait);
    implementor.do_something();
    test_ref_or_box.my_trait.do_something();
    assert_eq!(2, implementor.calls());
    assert_eq!(0, implementor.mut_calls());
}

#[test]
fn test_ref_or_box_with_box() -> Result<(), Box<dyn Error>> {
    let implementor: Box<dyn MyTrait> = Box::new(Implementor::default());
    let test_ref_or_box = TestRefOrBox::new(implementor);
    test_ref_or_box.my_trait.do_something();

    let implementor = match test_ref_or_box.my_trait {
        RefOrBox::Borrowed(_) => panic!("Wrong RefOrBox variant"),
        RefOrBox::Owned(value) => downcast_to_implementor(value)
    };
    assert_eq!(1, implementor.calls());
    assert_eq!(0, implementor.mut_calls());
    Ok(())
}

#[test]
fn test_ref_mut_or_box_with_ref() {
    let mut implementor = Implementor::default();
    let mut test_ref_mut_or_box = TestRefMutOrBox::new(&mut implementor as &mut dyn MyTrait);
    test_ref_mut_or_box.my_trait.do_mutable();
    implementor.do_mutable();
    assert_eq!(0, implementor.calls());
    assert_eq!(2, implementor.mut_calls());
}

#[test]
fn test_ref_mut_or_box_with_box() -> Result<(), Box<dyn Error>> {
    let implementor: Box<dyn MyTrait> = Box::new(Implementor::default());
    let mut test_ref_or_box = TestRefMutOrBox::new(implementor);
    test_ref_or_box.my_trait.do_mutable();

    let implementor = match test_ref_or_box.my_trait {
        RefMutOrBox::Borrowed(_) => panic!("Wrong RefMutOrBox variant"),
        RefMutOrBox::Owned(value) => downcast_to_implementor(value)
    };
    assert_eq!(0, implementor.calls());
    assert_eq!(1, implementor.mut_calls());
    Ok(())
}

//
// into_owned() tests
//

#[cfg(feature = "trait-clone")]
trait CloneTrait: dyn_clone::DynClone {}

#[derive(Clone, Default)]
struct ClonableStruct {}

#[cfg(feature = "trait-clone")]
impl CloneTrait for ClonableStruct {}

#[test]
fn ref_or_owned_into_owned() {
    let clonable = ClonableStruct::default();
    let clonable = RefOrOwned::Borrowed(&clonable);
    let _cloned: ClonableStruct = clonable.into_owned();
}

#[test]
#[cfg(feature = "trait-clone")]
fn ref_or_box_into_owned() {
    let clonable = ClonableStruct::default();
    let clonable: RefOrBox<dyn CloneTrait> = RefOrBox::from(&clonable as &dyn CloneTrait);
    let _cloned: Box<dyn CloneTrait> = clonable.into_owned();
}

//
// Deref, AsRef, AsMut, Borrow, and BorrowMut
//

#[test]
fn ref_or_owned_as_ref() {
    let implementor = Implementor::default();
    let implementor = RefOrBox::from(&implementor);
    let _my_trait: &Implementor = implementor.deref();
    let _my_trait: &Implementor = implementor.as_ref();
    let _my_trait: &Implementor = implementor.borrow();
}

#[test]
fn ref_mut_or_owned_as_mut() {
    let mut implementor = Implementor::default();
    let mut implementor = RefMutOrBox::from(&mut implementor);
    let _my_trait: &Implementor = implementor.deref();
    let _my_trait: &Implementor = implementor.as_ref();
    let _my_trait: &Implementor = implementor.borrow();
    let _my_trait: &mut Implementor = implementor.as_mut();
    let _my_trait: &mut Implementor = implementor.borrow_mut();
}

#[test]
fn ref_or_box_as_ref() {
    let implementor = Implementor::default();
    let implementor: RefOrBox<dyn MyTrait> = RefOrBox::from(&implementor as &dyn MyTrait);
    let _my_trait: &dyn MyTrait = implementor.deref();
    let _my_trait: &dyn MyTrait = implementor.as_ref();
    let _my_trait: &dyn MyTrait = implementor.borrow();
}

#[test]
fn ref_mut_or_box_as_mut() {
    let mut implementor = Implementor::default();
    let mut implementor: RefMutOrBox<dyn MyTrait> = RefMutOrBox::from(&mut implementor as &mut dyn MyTrait);
    let _my_trait: &dyn MyTrait = implementor.deref();
    let _my_trait: &dyn MyTrait = implementor.as_ref();
    let _my_trait: &dyn MyTrait = implementor.borrow();
    let _my_trait: &mut dyn MyTrait = implementor.as_mut();
    let _my_trait: &mut dyn MyTrait = implementor.borrow_mut();
}

//
// Default, Hash, and Display
// PartialEq, Eq, PartialOrd, and Ord
//

trait BeanTrait: Display {
    fn data(&self) -> u8;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Bean {
    data: u8
}

impl Bean {
    fn new(data: u8) -> Self {
        Self { data }
    }
}

impl BeanTrait for Bean {
    fn data(&self) -> u8 {
        self.data
    }
}

impl PartialEq<dyn BeanTrait> for dyn BeanTrait {
    fn eq(&self, other: &dyn BeanTrait) -> bool {
        self.data().eq(&other.data())
    }
}

impl PartialOrd<dyn BeanTrait> for dyn BeanTrait {
    fn partial_cmp(&self, other: &dyn BeanTrait) -> Option<Ordering> {
        self.data().partial_cmp(&other.data())
    }
}

impl Default for Bean {
    fn default() -> Self {
        Self {
            // Don't generate u8::MAX to avoid overflows when incrementing
            data: fastrand::u8(..u8::MAX)
        }
    }
}

impl Display for Bean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Bean: {:?}", self.data))
    }
}

fn eval_partial_eq<P: PartialEq<P>>(param1: &P, param2: &P) -> bool {
    param1.eq(param2)
}

fn eval_eq<P: Eq>(param1: &P, param2: &P) -> bool {
    param1.eq(param2)
}

fn eval_partial_ord<P: PartialOrd<P>>(param1: &P, param2: &P) -> Ordering {
    let ordering = param1.partial_cmp(param2);
    ordering.expect("No ordering defined")
}

fn eval_ord<P: Ord>(param1: &P, param2: &P) -> Ordering {
    param1.cmp(param2)
}

fn eval_hash<H: Hash>(param: &H) -> u64 {
    let mut hasher = DefaultHasher::new();
    param.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn ref_or_owned_std_traits() {
    let generated: RefOrOwned<Bean> = RefOrOwned::default();
    let incremented = RefOrOwned::Owned(Bean::new(generated.data + 1));

    let _fmt = format!("Is: {}", &generated);
    let _hash = eval_hash(&generated);

    assert!(eval_partial_eq(&generated, &generated));
    assert!(eval_eq(&generated, &generated));
    assert!(!eval_partial_eq(&generated, &incremented));
    assert!(!eval_eq(&generated, &incremented));

    assert_eq!(Ordering::Equal, eval_partial_ord(&generated, &generated));
    assert_eq!(Ordering::Equal, eval_ord(&generated, &generated));
    assert_eq!(Ordering::Less, eval_partial_ord(&generated, &incremented));
    assert_eq!(Ordering::Less, eval_ord(&generated, &incremented));
    assert_eq!(Ordering::Greater, eval_partial_ord(&incremented, &generated));
    assert_eq!(Ordering::Greater, eval_ord(&incremented, &generated));
}

#[test]
fn ref_mut_or_owned_std_traits() {
    let generated: RefMutOrOwned<Bean> = RefMutOrOwned::default();
    let incremented = RefMutOrOwned::Owned(Bean::new(generated.data + 1));

    let _fmt = format!("Is: {}", &generated);
    let _hash = eval_hash(&generated);

    assert!(eval_partial_eq(&generated, &generated));
    assert!(eval_eq(&generated, &generated));
    assert!(!eval_partial_eq(&generated, &incremented));
    assert!(!eval_eq(&generated, &incremented));

    assert_eq!(Ordering::Equal, eval_partial_ord(&generated, &generated));
    assert_eq!(Ordering::Equal, eval_ord(&generated, &generated));
    assert_eq!(Ordering::Less, eval_partial_ord(&generated, &incremented));
    assert_eq!(Ordering::Less, eval_ord(&generated, &incremented));
    assert_eq!(Ordering::Greater, eval_partial_ord(&incremented, &generated));
    assert_eq!(Ordering::Greater, eval_ord(&incremented, &generated));
}

#[test]
fn ref_or_box_std_traits() {
    let generated: Box<dyn BeanTrait> = Box::new(Bean::default());
    let generated = RefOrBox::Owned(generated);
    let incremented: Box<dyn BeanTrait> = Box::new(Bean::new(generated.data() + 1));
    let incremented = RefOrBox::Owned(incremented);

    let _fmt = format!("Is: {}", &generated);

    assert!(eval_partial_eq(&generated, &generated));
    assert!(!eval_partial_eq(&generated, &incremented));

    assert_eq!(Ordering::Equal, eval_partial_ord(&generated, &generated));
    assert_eq!(Ordering::Less, eval_partial_ord(&generated, &incremented));
    assert_eq!(Ordering::Greater, eval_partial_ord(&incremented, &generated));
}

#[test]
fn ref_mut_or_box_std_traits() {
    let generated: Box<dyn BeanTrait> = Box::new(Bean::default());
    let generated = RefMutOrBox::Owned(generated);
    let incremented: Box<dyn BeanTrait> = Box::new(Bean::new(generated.data() + 1));
    let incremented = RefMutOrBox::Owned(incremented);

    let _fmt = format!("Is: {}", &generated);

    assert!(eval_partial_eq(&generated, &generated));
    assert!(!eval_partial_eq(&generated, &incremented));

    assert_eq!(Ordering::Equal, eval_partial_ord(&generated, &generated));
    assert_eq!(Ordering::Less, eval_partial_ord(&generated, &incremented));
    assert_eq!(Ordering::Greater, eval_partial_ord(&incremented, &generated));
}
