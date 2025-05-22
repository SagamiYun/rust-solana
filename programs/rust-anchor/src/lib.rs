use anchor_lang::prelude::*;

declare_id!("BF1Q7hLntSidgYjCMzG298QpRbBb5daA672W7mNiWFVf");

#[program]
pub mod rust_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
