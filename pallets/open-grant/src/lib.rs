#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{
	codec::{Decode, Encode},
	decl_module, decl_storage, decl_event, decl_error, traits::Get,
	traits::{ReservableCurrency, ExistenceRequirement, Currency, },
	debug, ensure,
};

use sp_runtime::{
	traits::{AccountIdConversion},
	ModuleId,
};

use frame_system::{ensure_signed, ensure_root, };
use sp_std::prelude::*;
use sp_std::{convert::{TryInto}};
use integer_sqrt::IntegerSquareRoot;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	// used to generate sovereign account
	// refer: https://github.com/paritytech/substrate/blob/743accbe3256de2fc615adcaa3ab03ebdbbb4dbd/frame/treasury/src/lib.rs#L92
	type ModuleId: Get<ModuleId>;

	/// The currency in which the crowdfunds will be denominated
	type Currency: ReservableCurrency<Self::AccountId>;

	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

pub type ProjectIndex = u32;
pub type GrantRoundIndex = u32;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
type ProjectOf<T> = Project<AccountIdOf<T>>;
type ContributionOf<T> = Contribution<AccountIdOf<T>, BalanceOf<T>>;
type GrantRoundOf<T> = GrantRound<AccountIdOf<T>, BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;
type GrantOf<T> = Grant<AccountIdOf<T>, BalanceOf<T>>;

/// Grant Round struct
#[derive(Encode, Decode, Default, PartialEq, Eq, Clone, Debug)]
pub struct GrantRound<AccountId, Balance, BlockNumber> {
	start: BlockNumber,
	end: BlockNumber,
	matching_fund: Balance,
	grants: Vec<Grant<AccountId, Balance>>,
	funder: AccountId,
}

impl<AccountId, Balance, BlockNumber> GrantRound<AccountId, Balance, BlockNumber> {
    fn new(start: BlockNumber, end: BlockNumber, matching_fund: Balance, project_indexes: Vec<ProjectIndex>, funder: AccountId) -> GrantRound<AccountId, Balance, BlockNumber> { 
		let mut grant_round  = GrantRound {
			start: start,
			end: end,
			matching_fund: matching_fund,
			grants: Vec::new(),
			funder: funder,
		};

		// Fill in the grants structure in advance
		for project_index in project_indexes {
			grant_round.grants.push(Grant {
				project_index: project_index,
				contributions: Vec::new(),
				is_allowed_withdraw: false,
				is_canceled: false,
				is_withdrawn: false,

			});
		}

		grant_round
	}
} 

// Grant in round
#[derive(Encode, Decode, Default, PartialEq, Eq, Clone, Debug)]
pub struct Grant<AccountId, Balance> {
	project_index: ProjectIndex,
	contributions: Vec<Contribution<AccountId, Balance>>,
	is_allowed_withdraw: bool,
	is_canceled: bool,
	is_withdrawn: bool,
}

/// Grant struct
#[derive(Encode, Decode, Default, PartialEq, Eq, Clone, Debug)]
pub struct Contribution<AccountId, Balance> {
	account_id: AccountId,
	value: Balance,
}

/// Project struct
#[derive(Encode, Decode, Default, PartialEq, Eq, Clone, Debug)]
pub struct Project<AccountId> {
	name: Vec<u8>,
	logo: Vec<u8>,
	description: Vec<u8>,
	website: Vec<u8>,
	/// The account that will receive the funds if the campaign is successful
	owner: AccountId,
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as OpenGrantModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Projects get(fn grants): map hasher(blake2_128_concat) ProjectIndex => Option<ProjectOf<T>>;
		ProjectCount get(fn project_count): ProjectIndex;

		GrantRounds get(fn grant_rounds): map hasher(blake2_128_concat) GrantRoundIndex => Option<GrantRoundOf<T>>;
		GrantRoundCount get(fn grant_round_count): GrantRoundIndex;

		MaxRoundProjects get(fn max_round_projects): u32;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where Balance = BalanceOf<T>, AccountId = <T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ProjectCreated(ProjectIndex),
		GrantRoundCreated(GrantRoundIndex),
		ContributeSucceed(AccountId, ProjectIndex, Balance, BlockNumber),
		GrantCanceled(GrantRoundIndex, ProjectIndex),
		GrantWithdrawn(GrantRoundIndex, ProjectIndex, Balance, Balance),
		GrantAllowedWithdraw(GrantRoundIndex, ProjectIndex),
		RoundCanceled(GrantRoundIndex),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// There was an overflow.
		Overflow,
		///
		RoundProcessing,
		StartBlockNumberInvalid,
		EndBlockNumberInvalid,
		EndTooEarly,
		NoActiveRound,
		NoActiveGrant,
		InvalidParam,
		GrantCanceled,
		GrantWithdrawn,
		GrantNotAllowWithdraw,
		InvalidAccount,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// Create project
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn create_project(origin, name: Vec<u8>, logo: Vec<u8>, description: Vec<u8>, website: Vec<u8>) {
			let who = ensure_signed(origin)?;

			debug::debug!("name: {:#?}", name);
			debug::debug!("logo: {:#?}", logo);
			debug::debug!("description: {:#?}", description);
			debug::debug!("website: {:#?}", website);

			// TODO: Validation
			ensure!(name.len() > 0, Error::<T>::InvalidParam);
			ensure!(logo.len() > 0, Error::<T>::InvalidParam);
			ensure!(description.len() > 0, Error::<T>::InvalidParam);
			ensure!(website.len() > 0, Error::<T>::InvalidParam);
			
			let index = ProjectCount::get();
			let next_index = index.checked_add(1).ok_or(Error::<T>::Overflow)?;

			// Create a grant 
			let project = ProjectOf::<T> {
				name: name,
				logo: logo,
				description: description,
				website: website,
				owner: who,
			};

			// Add grant to list
			<Projects<T>>::insert(index, project);
			ProjectCount::put(next_index);

			Self::deposit_event(RawEvent::ProjectCreated(index));
		}

		/// Schedule a round
		/// If the last round is not over, no new round can be scheduled
		/// grant_indexes: the grants were selected for this round
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn schedule_round(origin, start: T::BlockNumber, end: T::BlockNumber, matching_fund: BalanceOf<T>, project_indexes: Vec<ProjectIndex>) {
			ensure_root(origin.clone())?;
			let who = ensure_signed(origin)?;
			let now = <frame_system::Module<T>>::block_number();
			let index = GrantRoundCount::get();

			// The end block must be greater than the start block
			ensure!(end > start, Error::<T>::EndTooEarly);
			// Both the starting block number and the ending block number must be greater than the current number of blocks
			ensure!(start > now, Error::<T>::StartBlockNumberInvalid);
			ensure!(end > now, Error::<T>::EndBlockNumberInvalid);
			// Make sure the last round is over
			if index != 0 {
				let round = <GrantRounds<T>>::get(index-1).unwrap();
				ensure!(now > round.end, Error::<T>::RoundProcessing);
			}
			
			let next_index = index.checked_add(1).ok_or(Error::<T>::Overflow)?;

			let round = GrantRoundOf::<T>::new(start, end, matching_fund, project_indexes, who.clone());

			// Add grant round to list
			<GrantRounds<T>>::insert(index, round);
			GrantRoundCount::put(next_index);

			// Transfer matching fund to module account
			T::Currency::transfer(
				&who,
				&Self::account_id(),
				matching_fund,
				ExistenceRequirement::AllowDeath
			)?;

			Self::deposit_event(RawEvent::GrantRoundCreated(index));
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cancel_round(origin) {
			ensure_root(origin)?;
			let now = <frame_system::Module<T>>::block_number();
			let count = GrantRoundCount::get();
			let round = <GrantRounds<T>>::get(count-1).ok_or(Error::<T>::NoActiveRound)?;

			// Ensure current round is not processing
			ensure!(round.start > now, Error::<T>::RoundProcessing);

			GrantRoundCount::put(count-1);
			// Refund
			T::Currency::transfer(
				&Self::account_id(),
				&round.funder,
				round.matching_fund,
				ExistenceRequirement::AllowDeath
			)?;

			Self::deposit_event(RawEvent::RoundCanceled(count-1));
		}

		/// Contribute a grant
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn contribute(origin, project_index: ProjectIndex, value: BalanceOf<T>) {
			let who = ensure_signed(origin)?;
			let now = <frame_system::Module<T>>::block_number();
			
			// round list must be not none
			let round_index = GrantRoundCount::get();
			
			ensure!(round_index > 0, Error::<T>::NoActiveRound);
			
			// The round must be in progress
			let mut round = <GrantRounds<T>>::get(round_index-1).ok_or(Error::<T>::NoActiveRound)?;
			ensure!(round.start < now, Error::<T>::NoActiveRound);
			ensure!(round.end > now, Error::<T>::NoActiveRound);

			// Find grant by index
			let mut found_grant: Option<&mut GrantOf::<T>> = None;
			for grant in round.grants.iter_mut() {
				if grant.project_index == project_index {
					found_grant = Some(grant);
					break;
				}
			}

			let grant = found_grant.ok_or(Error::<T>::NoActiveGrant)?;
			ensure!(!grant.is_canceled, Error::<T>::GrantCanceled);

			// Find previous contribution by account_id
			// If you have contributed before, then add to that contribution. Otherwise join the list.
			let mut found_contribution: Option<&mut ContributionOf::<T>> = None;
			for contribution in grant.contributions.iter_mut() {
				debug::debug!("contribution.account_id: {:#?}", contribution.account_id);
				debug::debug!("who: {:#?}", who);
				if contribution.account_id == who {
					found_contribution = Some(contribution);
					break;
				}
			}

			match found_contribution {
				Some(contribution) => {
					contribution.value += value;
					debug::debug!("contribution.value: {:#?}", contribution.value);
				},
				None => {
					grant.contributions.push(ContributionOf::<T> {
						account_id: who.clone(),
						value: value,
					});
					debug::debug!("contributions: {:#?}", grant.contributions);
				}
			}

			// Transfer contribute to grant account
			T::Currency::transfer(
				&who,
				&Self::project_account_id(project_index),
				value,
				ExistenceRequirement::AllowDeath
			)?;

			debug::debug!("grant: {:#?}", grant);

			<GrantRounds<T>>::insert(round_index-1, round);

			Self::deposit_event(RawEvent::ContributeSucceed(who, project_index, value, now));
		}

		// Distribute fund from grant
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn allow_withdraw(origin, round_index: GrantRoundIndex, project_index: ProjectIndex) {
			ensure_root(origin.clone())?;
			let mut round = <GrantRounds<T>>::get(round_index).ok_or(Error::<T>::NoActiveRound)?;
			let grants = &mut round.grants;

			// The round must have ended
			let now = <frame_system::Module<T>>::block_number();
			ensure!(round.end < now, Error::<T>::RoundProcessing);

			// Find grant from list
			let mut found_grant: Option<&mut GrantOf::<T>> = None;
			for grant in grants.iter_mut() {
				if grant.project_index == project_index {
					found_grant = Some(grant);
					break;
				}
			}
			let mut grant = found_grant.ok_or(Error::<T>::NoActiveGrant)?;

			// Can't let users vote in the cancered round
			ensure!(!grant.is_canceled, Error::<T>::GrantCanceled);

			// set is_allowed_withdraw
			grant.is_allowed_withdraw = true;

			debug::debug!("round: {:#?}", round);

			<GrantRounds<T>>::insert(round_index, round.clone());

			Self::deposit_event(RawEvent::GrantAllowedWithdraw(round_index, project_index));
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn withdraw(origin, round_index: GrantRoundIndex, project_index: ProjectIndex) {
			let who = ensure_signed(origin)?;

			// Only project owner can withdraw
			let project = Projects::<T>::get(project_index).ok_or(Error::<T>::NoActiveGrant)?;
			ensure!(who == project.owner, Error::<T>::InvalidAccount);

			let mut round = <GrantRounds<T>>::get(round_index).ok_or(Error::<T>::NoActiveRound)?;
			let grants = &mut round.grants;

			// Calculate CLR(Capital-constrained Liberal Radicalism) for grant
			let mut grant_clrs = Vec::new();
			let mut total_clr = 0;
			let matching_fund = Self::balance_to_u128(round.matching_fund);

			let mut grant_index: Option<usize> = None;
			let mut contribution_amount = 0;

			// Calculate grant CLR
			for i in 0..grants.len() {
				let grant = &grants[i];
				let mut sqrt_sum = 0;
				for contribution in grant.contributions.iter() {
					let contribution_value = Self::balance_to_u128(contribution.value);
					debug::debug!("contribution_value: {}", contribution_value);
					sqrt_sum += contribution_value.integer_sqrt();
					contribution_amount += contribution_value;
				}
				debug::debug!("grant_sum: {}", sqrt_sum);
				let grant_clr = sqrt_sum * sqrt_sum;
				grant_clrs.push(grant_clr);
				total_clr += grant_clr;

				if grant.project_index == project_index {
					grant_index = Some(i);
				}
			}

			let grant_index = grant_index.ok_or(Error::<T>::NoActiveGrant)?;
			let mut grant = &mut grants[grant_index];

			// This grant must not have distributed funds
			ensure!(grant.is_allowed_withdraw, Error::<T>::GrantNotAllowWithdraw);
			ensure!(!grant.is_withdrawn, Error::<T>::GrantWithdrawn);
			ensure!(!grant.is_canceled, Error::<T>::GrantCanceled);

			// Calculate CLR
			let grant_clr = grant_clrs[grant_index];
			let grant_matching_fund = ((grant_clr as f64 / total_clr as f64) * matching_fund as f64) as u128;

			// Distribute CLR amount
			let grant_matching_fund = Self::u128_to_balance(grant_matching_fund);
			T::Currency::transfer(
				&Self::account_id(),
				&project.owner,
				grant_matching_fund,
				ExistenceRequirement::AllowDeath
			)?;

			// Distribute distribution
			let contribution_fund = Self::u128_to_balance(contribution_amount);
			T::Currency::transfer(
				&Self::project_account_id(project_index),
				&project.owner,
				contribution_fund,
				ExistenceRequirement::AllowDeath
			)?;

			// Set is_withdrawn
			grant.is_withdrawn = true;

			<GrantRounds<T>>::insert(round_index, round.clone());

			Self::deposit_event(RawEvent::GrantWithdrawn(round_index, project_index, grant_matching_fund, contribution_fund));
		}

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cancel(origin, round_index: GrantRoundIndex, project_index: ProjectIndex) {
			ensure_root(origin.clone())?;

			let mut round = <GrantRounds<T>>::get(round_index).ok_or(Error::<T>::NoActiveRound)?;
			let grants = &mut round.grants;

			let mut found_grant: Option<&mut GrantOf::<T>> = None;

			// Calculate grant CLR
			for grant in grants.iter_mut() {
				if grant.project_index == project_index {
					found_grant = Some(grant);
					break;
				}
			}

			let grant = found_grant.ok_or(Error::<T>::NoActiveGrant)?;

			// This grant must not have canceled
			ensure!(!grant.is_canceled, Error::<T>::GrantCanceled);
			ensure!(!grant.is_allowed_withdraw, Error::<T>::NoActiveGrant);

			for contribution in grant.contributions.iter() {
				T::Currency::transfer(
					&Self::project_account_id(project_index),
					&contribution.account_id,
					contribution.value,
					ExistenceRequirement::AllowDeath
				)?;
			}

			grant.is_canceled = true;

			<GrantRounds<T>>::insert(round_index, round);

			Self::deposit_event(RawEvent::GrantCanceled(round_index, project_index));
		}
	}
}

impl<T: Config> Module<T> {
	/// The account ID of the fund pot.
	///
	/// This actually does computation. If you need to keep using it, then make sure you cache the
	/// value and only call this once.
	pub fn account_id() -> T::AccountId {
		return T::ModuleId::get().into_account();
	}

	pub fn project_account_id(index: ProjectIndex) -> T::AccountId {
		T::ModuleId::get().into_sub_account(index)
	}

	pub fn u128_to_balance(cost: u128) -> BalanceOf<T> {
		TryInto::<BalanceOf::<T>>::try_into(cost).ok().unwrap()
	}

	pub fn balance_to_u128(balance: BalanceOf<T>) -> u128 {
		TryInto::<u128>::try_into(balance).ok().unwrap()
	}

}