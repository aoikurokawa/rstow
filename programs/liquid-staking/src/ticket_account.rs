use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct TicketAccountData {
    pub state_account: Pubkey,
    pub beneficiary: Pubkey,
    pub lamport_amount: u64,
    pub created_epoch: u64,
}
