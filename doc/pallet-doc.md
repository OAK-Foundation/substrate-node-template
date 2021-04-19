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

 - `create_project` - Create project.
 - `fund` - Donate to the foundation account.
 - `schedule_round` - Schedule a round.
 - `cancel_round` - Cancel a round.
 - `finalize_round` - Finalize a round. Calculate the matching funds for each project.
 - `contribute` - Contribute a grant.
 - `approve` - Approve project. When the project is approve, the owner of the project can withdraw funds.
 - `Withdraw` - Withdrawal, including matching funds and crowd donation funds.
 - `cancel` - Cancel a problematic project. When the project is cancelled, the people cannot donate to it, the foundation will not allocate funds to it, and the owner of the project will not be able to withdraw funds.
 - `set_max_round_grants` - Set max round grants.
 - `set_withdrawal_period` - Set withdrawal period. After the project is approved, if the project party does not withdraw the funds after the deadline, it will not be able to withdraw the funds afterwards.
 - `set_is_identity_needed` - Set whether to check identity.


 ## Genesis config

Genesis config is defined in node/src/chain_spec.rs

These are the default values:

- init_max_round_grants: 60

- init_withdrawal_period: 1000

- init_is_identity_needed: false

 ## Assumptions

 * The number of items in each round should be less than MaxRoundGrants.

 * When calling the schedule_round function, the start parameter of the new round should be greater than the end of the last valid round.

