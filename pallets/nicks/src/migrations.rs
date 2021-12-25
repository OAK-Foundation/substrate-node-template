
#![cfg_attr(not(feature = "std"), no_std)]

use super::*;
pub use pallet::*;
use sp_std::prelude::*;
use frame_support::{
	weights::Weight,
	pallet_prelude::*,
};
use frame_support::storage::migration;

pub mod v2 {
	use super::*;
	use crate::{Config};
	use frame_support::{
		traits::{Get, StorageVersion},
	};
	
	pub fn migrate<T: Config>() -> Weight {
		info!("Migrating nicks to version 2");

		let module_name = <crate::Pallet<T>>::name().as_bytes();
		let item_name = b"NameOf";

		let iter = migration::storage_key_iter::<T::AccountId, (Vec<u8>,BalanceOf<T>), Twox64Concat>(module_name, item_name);

		let name_count = migration::storage_key_iter::<T::AccountId, (Vec<u8>,BalanceOf<T>), Twox64Concat>(module_name, item_name).count() as u32;
		CountForNames::<T>::put(name_count);

		for item in iter {
			if let Some(take_item) = migration::take_storage_item::<T::AccountId, (Vec<u8>,BalanceOf<T>), Twox64Concat>(module_name, item_name, item.0.clone()) {
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

		StorageVersion::new(2).put::<crate::Pallet<T>>();

		T::DbWeight::get().reads(1)
	}
}
