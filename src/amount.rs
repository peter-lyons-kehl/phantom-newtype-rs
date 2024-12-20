// Copyright 2019 DFINITY
// Copyright 2023,2024 Peter Lyons Kehl
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::displayer::{DisplayProxy, DisplayerOf};
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
use crate::trait_flag::{self, TraitFlags};
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// `Amount<Unit>` provides a type-safe way to keep an amount of
/// some `Unit`.
///
///  E.g. the following code must not compile:
///
/// ```compile_fail
/// use phantom_newtype::Amount;
///
/// // These structs are just markers and have no semantic meaning.
/// enum Apples {}
/// enum Oranges {}
///
/// let trois_pommes = Amount::<Apples, u64>::from(3);
/// let five_oranges = Amount::<Oranges, u64>::from(5);
///
/// assert_eq!(8, (trois_pommes + five_oranges).get())
/// ```
///
/// `Amount<Unit, Repr>` defines common boilerplate to make type-safe
/// amounts more convenient.  For example, you can compare amounts:
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Amount;
///
/// enum Apples {}
/// type NumApples = Amount<Apples, u64>;
///
/// assert_eq!(true, NumApples::from(3) < NumApples::from(5));
/// assert_eq!(false, NumApples::from(3) > NumApples::from(5));
/// assert_eq!(true, NumApples::from(3) != NumApples::from(5));
/// assert_eq!(true, NumApples::from(5) == NumApples::from(5));
/// assert_eq!(false, NumApples::from(5) != NumApples::from(5));
///
/// assert_eq!(vec![NumApples::from(3), NumApples::from(5)].iter().max().unwrap(),
///            &NumApples::from(5));
/// ```
///
/// You can do simple arithmetics with amounts:
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Amount;
///
/// enum Apples {}
/// enum Oranges {}
///
/// let x = Amount::<Apples, u64>::from(3);
/// let y = Amount::<Oranges, u64>::from(5);
///
/// assert_eq!(x + x, Amount::<Apples, u64>::from(6));
/// assert_eq!(y - y, Amount::<Oranges, u64>::from(0));
/// ```
///
/// Multiplication of amounts is not supported: multiplying meters by
/// meters gives square meters. However, you can scale an amount by a
/// scalar or divide amounts:
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Amount;
///
/// enum Apples {}
///
/// let x = Amount::<Apples, u64>::from(3);
/// assert_eq!(x * 3, Amount::<Apples, u64>::from(9));
/// assert_eq!(1, x / x);
/// assert_eq!(3, (x * 3) / x);
/// ```
///
/// Note that the unit is only available at compile time, thus using
/// `Amount` instead of `u64` doesn't incur any runtime penalty:
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Amount;
///
/// enum Meters {}
///
/// let ms = Amount::<Meters, u64>::from(10);
/// assert_eq!(core::mem::size_of_val(&ms), core::mem::size_of::<u64>());
/// ```
///
/// Amounts can be serialized and deserialized with `serde`. Serialized
/// forms of `Amount<Unit, Repr>` and `Repr` are identical.
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// #[cfg(feature = "serde")] {
/// use phantom_newtype::Amount;
/// use serde::{Serialize, Deserialize};
/// use serde_json;
/// enum Meters {}
///
/// let repr: u64 = 10;
/// let m_10 = Amount::<Meters, u64>::from(repr);
/// assert_eq!(serde_json::to_string(&m_10).unwrap(), serde_json::to_string(&repr).unwrap());
///
/// let copy: Amount<Meters, u64> = serde_json::from_str(&serde_json::to_string(&m_10).unwrap()).unwrap();
/// assert_eq!(copy, m_10);
/// }
/// ```
///
/// You can also declare constants of `Amount<Unit, Repr>` using `new`
/// function:
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Amount;
/// enum Meters {}
/// type Distance = Amount<Meters, u64>;
/// const ASTRONOMICAL_UNIT: Distance = Distance::new(149_597_870_700);
///
/// assert!(ASTRONOMICAL_UNIT > Distance::from(0));
/// ```
///
/// Amounts can be sent between threads if the `Repr` allows it, no
/// matter which `Unit` is used.
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Amount;
///
/// type Cell = core::cell::RefCell<i64>;
/// type NumCells = Amount<Cell, i64>;
/// const N: NumCells = NumCells::new(1);
///
/// let n_from_thread = std::thread::spawn(|| &N).join().unwrap();
/// assert_eq!(N, *n_from_thread);
/// ```
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
#[repr(transparent)]
pub struct Amount<const TF: TraitFlags, Unit, Repr>(
    Repr,
    PhantomData<core::sync::atomic::AtomicPtr<Unit>>,
);

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Copy> Amount<TF, Unit, Repr> {
    // @TODO
    /// Returns the wrapped value.
    ///
    /// ```
    /// #![cfg_attr(
    ///     feature = "unstable_generic_const_own_type",
    ///     feature(generic_const_exprs)
    /// )]
    ///
    /// use phantom_newtype::Amount;
    ///
    /// enum Apples {}
    ///
    /// let three_apples = Amount::<Apples, u64>::from(3);
    /// assert_eq!(9, (three_apples * 3).get());
    /// ```
    pub fn get(&self) -> Repr {
        self.0
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> Amount<TF, Unit, Repr> {
    /// `new` is a synonym for `from` that can be evaluated in
    /// compile time. The main use-case of this functions is defining
    /// constants.
    pub const fn new(repr: Repr) -> Self {
        Self(repr, PhantomData)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit: Default, Repr> Amount<TF, Unit, Repr> {
    // @TODO similar but without &self
    //
    /// Provides a useful shortcut to access units of an amount if
    /// they implement the `Default` trait:
    ///
    /// ```
    /// #![cfg_attr(
    ///     feature = "unstable_generic_const_own_type",
    ///     feature(generic_const_exprs)
    /// )]
    ///
    /// use phantom_newtype::Amount;
    ///
    /// #[derive(Debug, Default)]
    /// struct Seconds;
    /// let duration = Amount::<Seconds, u64>::from(5);
    ///
    /// assert_eq!("5 Seconds", format!("{} {:?}", duration, duration.unit()));
    /// ```
    pub fn unit(&self) -> Unit {
        Default::default()
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> Amount<TF, Unit, Repr>
where
    Unit: DisplayerOf<Self>, //@TODO cleanup: Amount<TF, Unit, Repr>>,
{
    /// `display` provides a mechanism to implement a custom display
    /// for phantom types.
    ///
    /// ```
    /// #![cfg_attr(
    ///     feature = "unstable_generic_const_own_type",
    ///     feature(generic_const_exprs),
    /// )]
    ///
    /// use phantom_newtype::DisplayerOf;
    /// use core::fmt;
    ///
    /// struct Cents;
    /// struct YearUnit;
    /// // This causes ICE (with feature `unstable_generic_const_own_type`), see https://github.com/rust-lang/rust/issues/134044:
    /// #[cfg(not(feature = "unstable_generic_const_own_type"))]
    /// type Money = phantom_newtype::Amount<Cents, u64>;
    /// // No ICE:
    /// #[cfg(feature = "unstable_generic_const_own_type")]
    /// type Money = phantom_newtype::AmountForFlags<{phantom_newtype::trait_flag::TRAIT_FLAGS_IS_COPY_IS_DEFAULT}, Cents, u64>;
    ///
    /// impl DisplayerOf<Money> for Cents {
    ///   fn display(amount: &Money, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///     write!(f, "${}.{:02}", amount.get() / 100, amount.get() % 100)
    ///   }
    /// }
    ///
    /// assert_eq!(format!("{}", Money::from(1005).display()), "$10.05");
    /// ```
    pub fn display(&self) -> DisplayProxy<'_, Self, Unit> {
        DisplayProxy::new(self)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> From<Repr> for Amount<TF, Unit, Repr> {
    fn from(repr: Repr) -> Self {
        Self::new(repr)
    }
}

// Note that we only have to write the boilerplate trait
// implementation below because default implementations of traits put
// unnecessary restrictions on the type parameters. E.g. deriving
// `PartialEq<Wrapper<T>>` require `T` to implement `PartialEq`, which
// is not what we want: `T` is phantom in our case.

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Clone> Clone for Amount<TF, Unit, Repr> {
    fn clone(&self) -> Self {
        Amount(self.0.clone(), PhantomData)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Unit, Repr: Copy> Copy for Amount<{ trait_flag::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Unit, Repr> {}
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Unit, Repr: Copy> Copy for Amount<{ trait_flag::TRAIT_FLAGS_IS_COPY_NO_DEFAULT }, Unit, Repr> {}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Unit, Repr: Default> Default
    for Amount<{ trait_flag::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Unit, Repr>
{
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Unit, Repr: Default> Default
    for Amount<{ trait_flag::TRAIT_FLAGS_NO_COPY_IS_DEFAULT }, Unit, Repr>
{
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: PartialEq> PartialEq for Amount<TF, Unit, Repr> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Eq> Eq for Amount<TF, Unit, Repr> {}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: PartialOrd> PartialOrd for Amount<TF, Unit, Repr> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Ord> Ord for Amount<TF, Unit, Repr> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Hash> Hash for Amount<TF, Unit, Repr> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> Add for Amount<TF, Unit, Repr>
where
    Repr: AddAssign + Copy,
{
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        self.add_assign(rhs);
        self
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> AddAssign for Amount<TF, Unit, Repr>
where
    Repr: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.get()
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> SubAssign for Amount<TF, Unit, Repr>
where
    Repr: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.get()
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> Sub for Amount<TF, Unit, Repr>
where
    Repr: SubAssign + Copy,
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        self.sub_assign(rhs);
        self
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> MulAssign<Repr> for Amount<TF, Unit, Repr>
where
    Repr: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Repr) {
        self.0 *= rhs;
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> Mul<Repr> for Amount<TF, Unit, Repr>
where
    Repr: MulAssign + Copy,
{
    type Output = Self;

    fn mul(mut self, rhs: Repr) -> Self {
        self.mul_assign(rhs);
        self
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> Div<Self> for Amount<TF, Unit, Repr>
where
    Repr: Div<Repr> + Copy,
{
    type Output = <Repr as Div>::Output;

    fn div(self, rhs: Self) -> Self::Output {
        self.0.div(rhs.0)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> fmt::Debug for Amount<TF, Unit, Repr>
where
    Repr: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> fmt::Display for Amount<TF, Unit, Repr>
where
    Repr: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Derived serde `impl Serialize` produces an extra `unit` value for
// phantom data, e.g. `Amount::<Meters>::from(10)` is serialized
// into json as `[10, null]` by default.
//
// We want serialization format of `Repr` and the `Amount` to match
// exactly, that's why we have to provide custom instances.
#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Serialize> Serialize for Amount<TF, Unit, Repr> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<'de, const TF: TraitFlags, Unit, Repr> Deserialize<'de> for Amount<TF, Unit, Repr>
where
    Repr: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Repr::deserialize(deserializer).map(Self::new)
    }
}
