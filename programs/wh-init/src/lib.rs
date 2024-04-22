use anchor_lang::prelude::*;
use wormhole_anchor_sdk::{token_bridge, wormhole};

declare_id!("3sUT6u9JLzfYyc6SW1e1Co7sgLKCn1Est8AGyLWfHitG");

#[program]
pub mod wh_init {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.owner = ctx.accounts.owner.key();
        config.wormhole_bridge = ctx.accounts.wormhole_bridge.key();
        config.token_bridge_config = ctx.accounts.token_bridge_config.key();
        msg!("Initialized Config.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [Config::SEED_PREFIX],
        bump,
        space = Config::SPACE,

    )]

    /// Config account, which saves Wormhole accounts.
    pub config: Account<'info, Config>,

    /// Wormhole program.
    pub wormhole_program: Program<'info, wormhole::program::Wormhole>,

    /// Token Bridge program.
    pub token_bridge_program: Program<'info, token_bridge::program::TokenBridge>,

    #[account(
        mut,
        seeds = [wormhole::BridgeData::SEED_PREFIX],
        bump,
        seeds::program = wormhole_program,
    )]
    /// Wormhole bridge data account (a.k.a. its config).
    pub wormhole_bridge: Account<'info, wormhole::BridgeData>,

    #[account(
        seeds = [token_bridge::Config::SEED_PREFIX],
        bump,
        seeds::program = token_bridge_program,
    )]
    /// Token Bridge config. Token Bridge program needs this account to
    /// invoke the Wormhole program to post messages. Even though it is a
    /// required account for redeeming token transfers, it is not actually
    /// used for completing these transfers.
    pub token_bridge_config: Account<'info, token_bridge::Config>,

    /// System program.
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Config {
    /// Program's owner.
    pub owner: Pubkey, // 32 bytes

    /// Wormhole's [BridgeData](wormhole_anchor_sdk::wormhole::BridgeData)
    /// address. Needed by the Wormhole program to post messages.
    pub wormhole_bridge: Pubkey, // 32 bytes

    /// [TokenBridge's Config](wormhole_anchor_sdk::token_bridge::Config)
    /// address. Needed by the TokenBridge to post messages to Wormhole.
    pub token_bridge_config: Pubkey, // 32 bytes
}

impl Config {
    // discriminator (8) first
    pub const SPACE: usize = 8 + (3 * 32);

    /// AKA `b"config"`.
    pub const SEED_PREFIX: &'static [u8] = b"config";
}
