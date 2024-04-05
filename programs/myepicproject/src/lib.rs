use anchor_lang::prelude::*;

declare_id!("8TS8VfRgbRgU7eHehprTC5sdxxfDA8EjV3NcV63hQKeH");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
