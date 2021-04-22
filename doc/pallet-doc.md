 # Quadratic Funding Pallet

 The Quadratic Funding Pallet provides functionality for [the crowdfund matching mechanism explained in Web3 Open Grant #268](https://github.com/w3f/Open-Grants-Program/pull/268), including open-source project submission, funding round commencement, end user contribution and funding round finalization.
 ## Overview

 ### Terminology
Before diving into the detailed functionality of the Quadratic Funding pallet, let us clarify the terms and concepts used in the project.

- **The campaign:** The crowd-funding process of Quadratic Funding. One round of campaign usually lasts four weeks.

 - **Project:** The open source software program that will contribute to Polkadot society as public goods and participate in the funding campaign of the Quadratic Funding. 

 - **Users:** End users who are holders of Polkadot(DOT) and willing to participate in the campaign as contributors 

 - **The Committee:** The group of judges who are responsible to reviewing applications, examining contribution result and finalizing the campaign round.

### Introduction
Quadratic funding(referred to as QF) is basically a crowdfunding campaign wherein users match contributions from everyday citizens with a pool raised from bigger donors, which in this case, Web3 foundation. Each campaign usually last 4 weeks, during which users could contribute any amount to sponsor the projects they like. 

Prior to the campaign start date, the committee of QF will view all applications of projects and vote upon what get admitted into the campaign. There usually will be 8-12 projects within each campaign but the number may vary. 

When the campaign concludes, users are no longer able to make contribution to projects, and the committee will review the final result. If the committee agrees with the result, they will vote the finalize the campaign, which when happens the funding amount of each project will be secured for dispensing. If the committee finds any foul play, they can vote to remove one or more projects from the campaign before finalization. Upon removal, the funding amount of the campaign will be re-calculated and re-distributed to the rest of the participating projects. After reviewing, the committee can vote to finalize the new result.
 ### Storages
 
 The Quadratic Funding pallet saves data in these fields.
 - `Projects` - List of participating projects.
 - `ProjectCount` - Number of projects submitted to a campaign.
 - `Rounds` -  List of the past and the current round of campaigns.
 - `RoundCount` - Total number of campaign including the past and the current.
 - `MaxGrantCountPerRound` - The maximum number of grants allowed in one campaign.
 - `WithdrawalExpiration` - The withdrawal expiration period in block number. If grant funds are not withdrawn within a long period of time the grant will expire and funds will be unfrozen for the pallet to re-use.
 - `IsIdentityRequired` - Whether on-chain identity of an address is required for functions. Currently this variable is only used by project creation. If turned on, the creator is required to have an on-chain identity for submission.

 ### Structs
 - `Round` - Holds the data for each campaign. There is only one campaign can happen at any given time.
    ```
    pub struct Round<AccountId, Balance, BlockNumber> {
      start: BlockNumber,
      end: BlockNumber,
      matching_fund: Balance,
      grants: Vec<Grant<AccountId, Balance, BlockNumber>>,
      is_canceled: bool,
      is_finalized: bool,
    }
    ```

 - `Project` - An open source software program application submitted to a campaign
    ```
    pub struct Project<AccountId> {
      name: Vec<u8>,
      logo: Vec<u8>,
      description: Vec<u8>,
      website: Vec<u8>,
      owner: AccountId,
    }
    ```

 - `Grant` - When admitted into a campaign, a project becomes a grant which allows users to contribute funds to.
    ```
    pub struct Grant<AccountId, Balance, BlockNumber> {
      project_index: ProjectIndex,
      contributions: Vec<Contribution<AccountId, Balance>>,
      is_approved: bool,
      is_canceled: bool,
      is_withdrawn: bool,
      withdrawal_expiration: BlockNumber,
      matching_fund: Balance,
    }
    ```

 - `Contribution` - The contribution users made to a grant project.
    ```
    pub struct Contribution<AccountId, Balance> {
      account_id: AccountId,
      value: Balance,
    }
    ```

 ## Interface

 ### Dispatchable Functions

 - `pub fn create_project(origin, name: Vec<u8>, logo: Vec<u8>, description: Vec<u8>, website: Vec<u8>)` - Create a project by a developer.

 - `pub fn fund(origin, fund_balance: BalanceOf<T>)` - Donate to the QF pallet account, usually called by the committee but can be called by any one. Tokens donated via this method will be part of the matching fund. Regular users should call contribute() instead of this method.

 - `pub fn schedule_round(origin, start: T::BlockNumber, end: T::BlockNumber, matching_fund: BalanceOf<T>, project_indexes: Vec<ProjectIndex>)` - Schedule a round of campaign by the committee. The method will decide what projects are included as grant projects.

 - `pub fn cancel_round(origin, round_index: RoundIndex)` - Cancel a whole round of campaign.

 - `pub fn finalize_round(origin, round_index: RoundIndex)` - Finalize a round by the committee. When called, the function calculate matching funds for each grant project.

 - `pub fn contribute(origin, project_index: ProjectIndex, value: BalanceOf<T>)` - Contribute to a grant project during a campaign, called by users.

 - `pub fn approve(origin, round_index: RoundIndex, project_index: ProjectIndex)` - Make allocated grant funds available for the developer team to withdraw, called by the committee. The method allows for fund dispensing upon milestone delivery. For example, 300 DOTs were allocated to a team's funding during campaign finalization, and the committee can approve 100 DOTs after the first month, and 100 DOTs every month after. 

 - `pub fn cancel(origin, round_index: RoundIndex, project_index: ProjectIndex)` - Cancel a project by the committee, usually due to the reason that the project is found at foul play or withdrawal. This method can be called after the campaign start block and before campaign finalization. When this method is called, no more contribution could be made and the previous contribution from users will be returned. When cancelled, the grant project will receive no matching fund during finalization.
 
 - `pub fn set_max_grant_count_per_round(origin, max_round_grants: u32)` - Set the maximum allowed number of grants in one round. The method is not expected to be called frequently since the default value of 60 should be sufficient.

 - `pub fn set_withdrawal_expiration(origin, withdrawal_period: T::BlockNumber)` - Set withdrawal expiration in block number. After approved, if the funds is not withdrawn by the developer team within a certain number of blocks, the approval will be undone and the funds will return to the pallet address for reuse. This is to make sure funds approved is not locked for good and the developer team deliver the milestones within the planned duration.

 - `pub fn withdraw(origin, round_index: RoundIndex, project_index: ProjectIndex)` - Withdraw approved funds by the developer team. 

 - `pub fn set_is_identity_required(origin, is_identity_needed: bool)` - Set whether an on-chain identity is required for project submission, called by the commitee.

 ## Genesis config

Genesis config is defined in node/src/chain_spec.rs. The default values of the genesis config are:

- `init_max_grant_count_per_round: 60`
    The maximum number of grants in one campaign. 

- `init_withdrawal_expiration: 1000`
    The withdrawal expiration period in block number.

- `init_is_identity_required: false`
    Whether on-chain identity of an address is required for functions.
 ## Assumptions

 * The number of selected projects in each campaign is less than MaxGrantCountPerRound.

 * When calling the schedule_round function, the start block number of the new campaign is greater than the end block number of the last campaign.

