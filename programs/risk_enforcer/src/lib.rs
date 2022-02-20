use anchor_lang::prelude::*;
use mango::state::MangoAccount;

//#[cfg(feature = "mainnet")]
//declare_id!("GDDMwNyyx8uB6zrqwBFHjLLG3TBYk2F8Az4yrQC5RzMp");
//#[cfg(not(feature = "mainnet"))]
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod risk_enforcer {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, _sym: String) -> ProgramResult {
        let risk_account = &mut ctx.accounts.risk_account;
        risk_account.authority = *ctx.accounts.authority.key;
        Ok(())
    }                       

    pub fn reset_sequence_number(
        ctx: Context<ResetSequenceNumber>,
        sequence_num: u64
    ) -> ProgramResult {
        msg!("Resetting sequence number to {}", sequence_num);

        let risk_account = &mut ctx.accounts.risk_account;
        risk_account.sequence_num = sequence_num;

        Ok(())
    }

    pub fn update_risk_parameters(
        ctx: Context<UpdateRiskParameters>,
        max_open_orders: u8,
        max_position: u64,
        timeout_in_seconds: u64,
        fail_if_position_is_unexpected: bool
    ) -> ProgramResult {
        msg!("Updating risk paramters");
        let risk_account = &mut ctx.accounts.risk_account;
        risk_account.param_max_open_orders = max_open_orders;
        risk_account.param_max_position = max_position;
        risk_account.param_timeout_in_seconds = timeout_in_seconds;
        risk_account.param_fail_if_position_is_unexpected = fail_if_position_is_unexpected;
        Ok(())
    }

    pub fn check_risk_parameters(
        ctx: Context<CheckRiskParameters>,
        expected_position: u64
    ) -> ProgramResult {
        Ok(())
     }

    pub fn check_and_set_sequence_number(
        ctx: Context<CheckAndSetSequenceNumber>,
        sequence_num: u64
    ) -> ProgramResult {
        let risk_account = &mut ctx.accounts.risk_account;
        let last_known_sequence_num = risk_account.sequence_num;
        if sequence_num > last_known_sequence_num {
            msg!("Sequence in order | sequence_num={} | last_known={}", sequence_num, last_known_sequence_num);
            risk_account.sequence_num = sequence_num;
            return Ok(());
        }
        
        msg!("Sequence out of order | sequence_num={} | last_known={}", sequence_num, last_known_sequence_num);
        return Err(ErrorCode::SequenceOutOfOrder.into());
    }
}

#[derive(Accounts)]
#[instruction(sym: String)]
pub struct Initialize<'info> {
    #[account(init,
        payer=authority, 
        seeds=[sym.as_bytes(), authority.key().as_ref()], bump
    )]
    pub risk_account: Account<'info, RiskAccount>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ResetSequenceNumber<'info> {
    #[account(mut, has_one=authority)]
    pub risk_account: Account<'info, RiskAccount>,
    pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct CheckRiskParameters<'info> {
    #[account(mut, has_one=authority)]
    pub risk_account: Account<'info, RiskAccount>,
    pub authority: Signer<'info>,
    pub mango_account: UncheckedAccount<'info>
}

#[derive(Accounts)]
pub struct UpdateRiskParameters<'info> {
    #[account(mut, has_one=authority)]
    pub risk_account: Account<'info, RiskAccount>,
    pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct CheckAndSetSequenceNumber<'info> {
    #[account(mut, has_one=authority)]
    pub risk_account: Account<'info, RiskAccount>,
    pub authority: Signer<'info>
}

#[account]
#[derive(Default)]
pub struct RiskAccount {
    pub authority: Pubkey,
    pub sequence_num: u64,
    // Max number of concurrent open orders
    pub param_max_open_orders: u8,
    // Max position in base amount (500 SOL, or 1 BTC)
    pub param_max_position: u64,
    // Timeout will prevent new orders from being placed for x seconds.
    pub param_timeout_in_seconds: u64,
    // Should we fail if position != expected position?
    pub param_fail_if_position_is_unexpected: bool,
}

#[error]
pub enum ErrorCode {
    #[msg("Sequence out of order")]
    SequenceOutOfOrder,
    #[msg("Exceeded max open orders")]
    ExceededMaxOpenOrders
}
