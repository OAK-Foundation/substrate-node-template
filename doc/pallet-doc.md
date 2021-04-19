 # Balances Pallet

 The Balances pallet provides functionality for handling accounts and balances.

 ## Overview

 The Balances pallet provides functions for:

 - Getting and setting free balances.
 - Retrieving total, reserved and unreserved balances.
 - Repatriating a reserved balance to a beneficiary account that exists.
 - Transferring a balance between accounts (when not reserved).
 - Slashing an account balance.
 - Account creation and removal.
 - Managing total issuance.
 - Setting and managing locks.

 ### Terminology

 - **Existential Deposit:** The minimum balance required to create or keep an account open. This prevents
 "dust accounts" from filling storage. When the free plus the reserved balance (i.e. the total balance)
   fall below this, then the account is said to be dead; and it loses its functionality as well as any
   prior history and all information on it is removed from the chain's state.
   No account should ever have a total balance that is strictly between 0 and the existential
   deposit (exclusive). If this ever happens, it indicates either a bug in this pallet or an
   erroneous raw mutation of storage.

 - **Total Issuance:** The total number of units in existence in a system.

 - **Reaping an account:** The act of removing an account by resetting its nonce. Happens after its
 total balance has become zero (or, strictly speaking, less than the Existential Deposit).

 - **Free Balance:** The portion of a balance that is not reserved. The free balance is the only
   balance that matters for most operations.

 - **Reserved Balance:** Reserved balance still belongs to the account holder, but is suspended.
   Reserved balance can still be slashed, but only after all the free balance has been slashed.

 - **Imbalance:** A condition when some funds were credited or debited without equal and opposite accounting
 (i.e. a difference between total issuance and account balances). Functions that result in an imbalance will
 return an object of the `Imbalance` trait that can be managed within your runtime logic. (If an imbalance is
 simply dropped, it should automatically maintain any book-keeping such as total issuance.)

 - **Lock:** A freeze on a specified amount of an account's free balance until a specified block number. Multiple
 locks always operate over the same funds, so they "overlay" rather than "stack".

 ### Implementations

 The Balances pallet provides implementations for the following traits. If these traits provide the functionality
 that you need, then you can avoid coupling with the Balances pallet.

 - [`Currency`](frame_support::traits::Currency): Functions for dealing with a
 fungible assets system.
 - [`ReservableCurrency`](frame_support::traits::ReservableCurrency):
 Functions for dealing with assets that can be reserved from an account.
 - [`LockableCurrency`](frame_support::traits::LockableCurrency): Functions for
 dealing with accounts that allow liquidity restrictions.
 - [`Imbalance`](frame_support::traits::Imbalance): Functions for handling
 imbalances between total issuance in the system and account balances. Must be used when a function
 creates new funds (e.g. a reward) or destroys some funds (e.g. a system fee).

 ## Interface

 ### Dispatchable Functions

 - `transfer` - Transfer some liquid free balance to another account.
 - `set_balance` - Set the balances of a given account. The origin of this call must be root.

 ## Usage

 The following examples show how to use the Balances pallet in your custom pallet.

 ### Examples from the FRAME

 The Contract pallet uses the `Currency` trait to handle gas payment, and its types inherit from `Currency`:

 ```
 use frame_support::traits::Currency;
 # pub trait Config: frame_system::Config {
 # 	type Currency: Currency<Self::AccountId>;
 # }

 pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
 pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

 # fn main() {}
 ```

 The Staking pallet uses the `LockableCurrency` trait to lock a stash account's funds:

 ```
 use frame_support::traits::{WithdrawReasons, LockableCurrency};
 use sp_runtime::traits::Bounded;
 pub trait Config: frame_system::Config {
 	type Currency: LockableCurrency<Self::AccountId, Moment=Self::BlockNumber>;
 }
 # struct StakingLedger<T: Config> {
 # 	stash: <T as frame_system::Config>::AccountId,
 # 	total: <<T as Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance,
 # 	phantom: std::marker::PhantomData<T>,
 # }
 # const STAKING_ID: [u8; 8] = *b"staking ";

 fn update_ledger<T: Config>(
 	controller: &T::AccountId,
 	ledger: &StakingLedger<T>
 ) {
 	T::Currency::set_lock(
 		STAKING_ID,
 		&ledger.stash,
 		ledger.total,
 		WithdrawReasons::all()
 	);
 	// <Ledger<T>>::insert(controller, ledger); // Commented out as we don't have access to Staking's storage here.
 }
 # fn main() {}
 ```

 ## Genesis config

 The Balances pallet depends on the [`GenesisConfig`].

 ## Assumptions

 * Total issued balanced of all accounts should be less than `Config::Balance::max_value()`.
