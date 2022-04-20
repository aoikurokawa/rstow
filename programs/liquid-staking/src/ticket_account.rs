use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct TicketAccountData {
    pub state_account: Pubkey, // instance of state this ticket belongs to
    pub beneficiary: Pubkey,   // main account where to send SOL when claimed
    pub lamport_amount: u64,   // amount this ticked is worth
    pub created_epoch: u64, // epoch when this account was created (epoch when deployed-unstake was requested)
}
