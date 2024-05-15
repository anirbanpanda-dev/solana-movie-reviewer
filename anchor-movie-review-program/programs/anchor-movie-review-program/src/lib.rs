use anchor_lang::prelude::*;

declare_id!("C7Xp9oaL3Se1wMw7KvqQD9DrWPpgGF2ph5F917T1LS4S");

#[program]
pub mod anchor_movie_review_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
