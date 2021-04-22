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
Quadratic funding(referred to as QF) is basically a crowdfunding campaign wherein users match contributions from everyday citizens with a pool raised from bigger donors, which in this case, Web3 foundation. Each campaign usually last 4 weeks, during which users could contribute any amount to sponsor the projects they like. Prior the campaign start date, the committee of QF will view all applications of projects and vote upon what get admitted into the campaign. There usually will be 8-12 projects within each campaign but the number may vary. When the campaign concludes, users are no longer able to make contribution to projects, and the committee will review the final result. If the committee agrees with the result, they will vote the finalize the campaign, which when happens the funding amount of each project will be secured for dispensing. If the committee finds any foul play, they can vote to remove one or more projects from the campaign before finalization. Upon removal, the funding amount of the campaign will be re-calculated and re-distributed to the rest of the participating projects. After reviewing, the committee can vote to finalize the new result.
 ### Storages
 
 The Quadratic Funding pallet saves data in these fields.
 - `Projects` - List of participating projects
 - `ProjectCount` - Number of projects.
 - `Rounds` -  List of the past and current round of campaigns
 - `RoundCount` - Number of rounds.
 - `MaxGrantCountPerRound` - The maximum amount of funding one project can get from each round.
 - `WithdrawalExpiration` - The expiration period in block number. If grant fund is not withdrawn within a long period of time the grant will expire and unfrozen for the pallet to reuse.
 - `IsIdentityRequired` - Is identity of address required for functions, such as project submission.

 ### Structs
 - `Round` - Holds the data 
 - `Grant` - A grant in a round.
 - `Contribution` - User contribution to a project within a campaign
 - `Project` - An open source software program participating in the campaign
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

Genesis config is defined in node/src/chain_spec.rs. The default values of the genesis config are:

- init_max_grant_count_per_round: 60

- init_withdrawal_expiration: 1000

- init_is_identity_required: false

 ## Assumptions

 * The number of items in each round should be less than MaxRoundGrants.

 * When calling the schedule_round function, the start parameter of the new round should be greater than the end of the last valid round.

