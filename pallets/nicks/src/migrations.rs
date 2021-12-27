use super::*;
pub use pallet::*;
use sp_std::prelude::*;
use frame_support::{
	weights::Weight,
	pallet_prelude::*,
	traits::{Get, StorageVersion},
	storage::migration,
};

pub fn migrate<T: Config>() -> Weight {
	let version = StorageVersion::get::<Pallet<T>>();
	let mut weight: Weight = 0;

	if version < 1 {
		weight = weight.saturating_add(v1::migrate::<T>());
		StorageVersion::new(1).put::<Pallet<T>>();
	}

	weight
}

pub mod v1 {
	use super::*;
	
	pub(crate) fn migrate<T: Config>() -> Weight {
		let mut reads_writes = 0;

		let module_name = <crate::Pallet<T>>::name().as_bytes();
		let item_name = b"NameOf";

		let iter = migration::storage_key_iter::<T::AccountId, (Vec<u8>,BalanceOf<T>), Twox64Concat>(module_name, item_name);

		let name_count = migration::storage_key_iter::<T::AccountId, (Vec<u8>,BalanceOf<T>), Twox64Concat>(module_name, item_name).count() as u32;
		CountForNames::<T>::put(name_count);
		reads_writes += 1;

		for item in iter {
			if let Some(take_item) = migration::take_storage_item::<T::AccountId, (Vec<u8>,BalanceOf<T>), Twox64Concat>(module_name, item_name, item.0.clone()) {
				reads_writes += 1;
				let (nick, deposit) = take_item;
				let value = match nick.iter().rposition(|&x| x == b" "[0]) {
					Some(ndx) => (Nickname {
						first: nick[0..ndx].to_vec(),
						last: Some(nick[ndx + 1..].to_vec())
					}, deposit),
					None => (Nickname { first: nick, last: None }, deposit)
				};
				RealnameOf::<T>::insert(item.0, value);
			}
		}

		StorageVersion::new(1).put::<crate::Pallet<T>>();
		reads_writes += 1;

		T::DbWeight::get().reads_writes(reads_writes, reads_writes)
	}
}
