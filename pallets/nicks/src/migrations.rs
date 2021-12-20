use super::*;

pub mod v2 {
	use super::*;
	use crate::{Config, Weight};
	use frame_support::traits::{Get, StorageVersion};

	pub fn migrate<T: Config>() -> Weight {
		info!("Migrating nicks to version 2");
		let name_count = NameOf::<T>::iter().count() as u32;
		CountForNames::<T>::put(name_count);
		StorageVersion::new(2).put::<crate::Pallet<T>>();
		info!("Completed nicks migration to version 2");

		T::DbWeight::get().reads(1)
	}
}
