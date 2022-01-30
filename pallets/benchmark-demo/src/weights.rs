
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_benchmark_demo.
pub trait WeightInfo {
	fn do_something(b: u32, ) -> Weight;
}

/// Weights for pallet_benchmark_demo using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn do_something(b: u32, ) -> Weight {
		(16_726_000 as Weight)
			// Standard Error: 0
			.saturating_add((8_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn do_something(b: u32, ) -> Weight {
		(16_726_000 as Weight)
			// Standard Error: 0
			.saturating_add((8_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
