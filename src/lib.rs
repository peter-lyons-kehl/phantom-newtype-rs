// Copyright 2019 DFINITY
// Copyright 2024 Peter Lyons Kehl
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

#![no_std]
#![cfg_attr(
    any(
        feature = "unstable_generic_const_own_type",
        feature = "unstable_transmute_unchecked"
    ),
    allow(incomplete_features)
)]
#![cfg_attr(
    feature = "unstable_generic_const_own_type",
    feature(adt_const_params),
    feature(generic_const_exprs)
)]
#![cfg_attr(feature = "unstable_transmute_unchecked", feature(core_intrinsics))]

//#![feature(unsized_const_params)] // https://github.com/rust-lang/rust/issues/95174

mod amount;
mod displayer;
mod id;
mod instant;
pub mod prelude;
pub mod prelude_full;
mod to;

#[cfg(not(feature = "unstable_generic_const_own_type"))]
mod trait_flag;
#[cfg(feature = "unstable_generic_const_own_type")]
pub mod trait_flag;

//#[cfg(feature = "alloc")]
//extern crate alloc;

pub use displayer::{DisplayProxy, DisplayerOf};

#[cfg(feature = "unstable_generic_const_own_type")]
pub use id::Id as IdForFlags;

pub use to::{As, AsFrom, AsFromMut, AsMut, To, ToFrom, ToFromMut, ToMut};

// Short names. Also in mod prelude:
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type Id<Unit, Repr> = id::Id<{ trait_flag::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Unit, Repr>;
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type IdNoCopy<Unit, Repr> = id::Id<{ trait_flag::TRAIT_FLAGS_NO_COPY_IS_DEFAULT }, Unit, Repr>;
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type IdNoDefault<Unit, Repr> =
    id::Id<{ trait_flag::TRAIT_FLAGS_IS_COPY_NO_DEFAULT }, Unit, Repr>;
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type IdNoCopyNoDefault<Unit, Repr> =
    id::Id<{ trait_flag::TRAIT_FLAGS_NO_COPY_NO_DEFAULT }, Unit, Repr>;

// Long names. Also in mod prelude_full:
pub type IdIsCopyIsDefault<Unit, Repr> = Id<Unit, Repr>;
pub type IdIsCopyNoDefault<Unit, Repr> = IdNoDefault<Unit, Repr>;
pub type IdNoCopyIsDefault<Unit, Repr> = IdNoCopy<Unit, Repr>;

#[cfg(feature = "unstable_generic_const_own_type")]
pub use amount::Amount as AmountForFlags;

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type Amount<Unit, Repr> =
    amount::Amount<{ trait_flag::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Unit, Repr>;
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type AmountNoCopy<Unit, Repr> =
    amount::Amount<{ trait_flag::TRAIT_FLAGS_NO_COPY_IS_DEFAULT }, Unit, Repr>;
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type AmountNoDefault<Unit, Repr> =
    amount::Amount<{ trait_flag::TRAIT_FLAGS_IS_COPY_NO_DEFAULT }, Unit, Repr>;
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type AmountNoCopyNoDefault<Unit, Repr> =
    amount::Amount<{ trait_flag::TRAIT_FLAGS_NO_COPY_NO_DEFAULT }, Unit, Repr>;

// Long names. Also in mod prelude_full:
pub type AmountIsCopyIsDefault<Unit, Repr> = Amount<Unit, Repr>;
pub type AmountIsCopyNoDefault<Unit, Repr> = AmountNoDefault<Unit, Repr>;
pub type AmountNoCopyIsDefault<Unit, Repr> = AmountNoCopy<Unit, Repr>;

#[cfg(feature = "unstable_generic_const_own_type")]
pub use instant::Instant as InstantForFlags;

// Short names. Also in mod prelude:
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type Instant<Unit, Repr> =
    instant::Instant<{ trait_flag::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Unit, Repr>;
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type InstantNoCopy<Unit, Repr> =
    instant::Instant<{ trait_flag::TRAIT_FLAGS_NO_COPY_IS_DEFAULT }, Unit, Repr>;
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type InstantNoDefault<Unit, Repr> =
    instant::Instant<{ trait_flag::TRAIT_FLAGS_IS_COPY_NO_DEFAULT }, Unit, Repr>;
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub type InstantNoCopyNoDefault<Unit, Repr> =
    instant::Instant<{ trait_flag::TRAIT_FLAGS_NO_COPY_NO_DEFAULT }, Unit, Repr>;

// Long names. Also in mod prelude_full:
pub type InstantIsCopyIsDefault<Unit, Repr> = Instant<Unit, Repr>;
pub type InstantIsCopyNoDefault<Unit, Repr> = InstantNoDefault<Unit, Repr>;
pub type InstantNoCopyIsDefault<Unit, Repr> = InstantNoCopy<Unit, Repr>;
