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

macro_rules! ref_or_owned_impls {
    ($typename:ident) => {
        impl<T: Default> Default for $typename<'_, T> {
            fn default() -> Self {
                Self::Owned(T::default())
            }
        }

        impl<T> Deref for $typename<'_, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                match self {
                    Self::Borrowed(borrowed_value) => *borrowed_value,
                    Self::Owned(owned_value) => owned_value
                }
            }
        }

        impl<T> From<T> for $typename<'_, T> {
            fn from(value: T) -> Self {
                Self::Owned(value)
            }
        }

        impl<T> $typename<'_, T> where T: Clone {
            /// Obtains an owned value of T.
            ///
            /// If the data is borrowed, it will be cloned and return.
            /// If the data is owned, the owned value will be moved out.
            ///
            /// Note: Sometimes, type inference will fail and it may be
            /// necessary to specify the owned type T.
            ///
            /// ```rust
            /// # use polymorph::ref_or_owned::RefOrOwned;
            /// #[derive(Clone, Default)]
            /// struct ClonableStruct {}
            ///
            /// let clonable = ClonableStruct::default();
            /// // Note: If RefOrOwned::Borrowed is changed to RefOrOwned::from, compilation fails due to type inference
            /// let clonable = RefOrOwned::Borrowed(&clonable);
            /// let _cloned: ClonableStruct = clonable.into_owned();
            /// ```
            pub fn into_owned(self) -> T {
                match self {
                   Self::Borrowed(borrowed_value) => borrowed_value.clone(),
                   Self::Owned(owned_value) => owned_value
               }
            }
        }

        impl<T> AsRef<T> for $typename<'_, T> {
            #[inline]
            fn as_ref(&self) -> &T {
                self.deref()
            }
        }

        impl<T> Borrow<T> for $typename<'_, T> {
            #[inline]
            fn borrow(&self) -> &T {
                self.deref()
            }
        }

        impl<T: PartialEq<U>, U> PartialEq<$typename<'_, U>> for $typename<'_, T> {
            #[inline]
            fn eq(&self, other: &$typename<'_, U>) -> bool {
               self.deref().eq(other.deref())
            }

            #[inline]
            fn ne(&self, other: &$typename<'_, U>) -> bool {
                self.deref().ne(other.deref())
            }
        }

        impl<T: Eq> Eq for $typename<'_, T> {}

        impl<T: PartialOrd<U>, U> PartialOrd<$typename<'_, U>> for $typename<'_, T> {
            #[inline]
            fn partial_cmp(&self, other: &$typename<'_, U>) -> Option<Ordering> {
                self.deref().partial_cmp(other.deref())
            }
        }

        impl<T: Ord> Ord for $typename<'_, T> {
            #[inline]
            fn cmp(&self, other: &Self) -> Ordering {
                self.deref().cmp(other.deref())
            }
        }

        impl<T: Hash> Hash for $typename<'_, T> {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.deref().hash(state)
            }
        }

        impl<T: Display> Display for $typename<'_, T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                self.deref().fmt(f)
            }
        }
    }
}

macro_rules! ref_or_box_impls {
    ($typename:ident) => {

        impl<T: ?Sized> Deref for $typename<'_, T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                match self {
                    Self::Borrowed(borrowed_value) => *borrowed_value,
                    Self::Owned(owned_box) => owned_box.deref()
                }
            }
        }

        impl<T: ?Sized> From<Box<T>> for $typename<'_, T> {
            fn from(value: Box<T>) -> Self {
                Self::Owned(value)
            }
        }

        #[cfg(feature = "trait-clone")]
        impl<T: ?Sized> $typename<'_, T> where T: dyn_clone::DynClone {
            /// Obtains an owned value of T. This requires the "trait-clone"
            /// feature and relies on the dyn-clone crate.
            ///
            /// If the data is borrowed, it will be cloned and return.
            /// If the data is owned, the owned value will be moved out.
            ///
            /// ```rust
            /// # use polymorph::ref_or_owned::RefOrBox;
            /// use dyn_clone::DynClone;
            ///
            /// trait Calculator: DynClone {
            ///   fn calculate(&mut self) -> u64;
            /// }
            ///
            /// fn clone_then_calculate<'c, C>(clonable: C) -> u64
            ///   where C: Into<RefOrBox<'c, dyn Calculator>> {
            ///
            ///   let mut clone = clonable.into().into_owned();
            ///   clone.calculate()
            /// }
            /// ```
            pub fn into_owned(self) -> Box<T> {
                match self {
                   Self::Borrowed(borrowed_value) => dyn_clone::clone_box(borrowed_value),
                   Self::Owned(owned_value) => owned_value
               }
            }
        }

        impl<T: ?Sized> AsRef<T> for $typename<'_, T> {
            #[inline]
            fn as_ref(&self) -> &T {
                self.deref()
            }
        }

        impl<T: ?Sized> Borrow<T> for $typename<'_, T> {
            #[inline]
            fn borrow(&self) -> &T {
                self.deref()
            }
        }

        impl<T: ?Sized + PartialEq<U>, U: ?Sized> PartialEq<$typename<'_, U>> for $typename<'_, T> {
            #[inline]
            fn eq(&self, other: &$typename<'_, U>) -> bool {
               self.deref().eq(other.deref())
            }

            #[inline]
            fn ne(&self, other: &$typename<'_, U>) -> bool {
                self.deref().ne(other.deref())
            }
        }

        impl<T: ?Sized + PartialOrd<U>, U: ?Sized> PartialOrd<$typename<'_, U>> for $typename<'_, T> {
            #[inline]
            fn partial_cmp(&self, other: &$typename<'_, U>) -> Option<Ordering> {
                self.deref().partial_cmp(other.deref())
            }
        }

        impl<T: ?Sized + Display> Display for $typename<'_, T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                self.deref().fmt(f)
            }
        }
    }
}

pub(crate) use ref_or_owned_impls;
pub(crate) use ref_or_box_impls;
