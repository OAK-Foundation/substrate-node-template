 # Open Grant Pallet

 The Open Grant pallet provides functionality for handling accounts and balances.

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

 ### Storages
 
 The Open grant pallet saves data in these fields.
 - `Projects` - Project list
 - `ProjectCount` - Number of projects.
 - `Rounds` -  Round list
 - `RoundCount` - Number of rounds.
 - `MaxRoundGrants` - In a round, the largest number of grants.
 - `WithdrawalPeriod` - Withdrawal expiration period.
 - `UnusedFund` - Unused funds.
 - `IsIdentityNeeded` - Whether to check identity.

 ### Structs
 - `Round` - Grant round.
 - `Grant` - A grant in a round.
 - `Contribution` - A Contribution for a grant.
 - `Project`
 
 The Open grant pallet saves data in these fields.
 - `Projects` - Project list
 - `ProjectCount` - Number of projects.
 - `Rounds` -  Round list
 - `RoundCount` - Number of rounds.
 - `MaxRoundGrants` - In a round, the largest number of grants.
 - `WithdrawalPeriod` - Withdrawal expiration period.
 - `UnusedFund` - Unused funds.
 - `IsIdentityNeeded` - Whether to check identity.

 ## Interface

 ### Dispatchable Functions

 - `pub fn create_project(origin, name: Vec<u8>, logo: Vec<u8>, description: Vec<u8>, website: Vec<u8>)` - Create project.

 - `pub fn fund(origin, fund_balance: BalanceOf<T>)` - Donate to the foundation account.

 - `pub fn schedule_round(origin, start: T::BlockNumber, end: T::BlockNumber, matching_fund: BalanceOf<T>, project_indexes: Vec<ProjectIndex>)` - Schedule a round.

 - `pub fn cancel_round(origin, round_index: RoundIndex)` - Cancel a round.

 - `pub fn finalize_round(origin, round_index: RoundIndex)` - Finalize a round. Calculate the matching funds for each project.

 - `pub fn contribute(origin, project_index: ProjectIndex, value: BalanceOf<T>)` - Contribute a grant.

 - `pub fn approve(origin, round_index: RoundIndex, project_index: ProjectIndex)` - Approve project. When the project is approve, the owner of the project can withdraw funds.

 - `pub fn withdraw(origin, round_index: RoundIndex, project_index: ProjectIndex)` - Withdrawal, including matching funds and crowd donation funds.

 - `pub fn cancel(origin, round_index: RoundIndex, project_index: ProjectIndex)` - Cancel a problematic project. When the project is cancelled, the people cannot donate to it, the foundation will not allocate funds to it, and the owner of the project will not be able to withdraw funds.

 - `pub fn set_max_round_grants(origin, max_round_grants: u32)` - Set max round grants.

 - `pub fn set_withdrawal_period(origin, withdrawal_period: T::BlockNumber)` - Set withdrawal period. After the project is approved, if the project party does not withdraw the funds after the deadline, it will not be able to withdraw the funds afterwards.

 - `pub fn set_is_identity_needed(origin, is_identity_needed: bool)` - Set whether to check identity.

 ## Genesis config

Genesis config is defined in node/src/chain_spec.rs

These are the default values:

- init_max_round_grants: 60

- init_withdrawal_period: 1000

- init_is_identity_needed: false

 ## Assumptions

 * The number of items in each round should be less than MaxRoundGrants.

 * When calling the schedule_round function, the start parameter of the new round should be greater than the end of the last valid round.

