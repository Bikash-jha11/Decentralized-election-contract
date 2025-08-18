use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("6rLcNazsDmDGVZoC72biZc2Gib3iMCMwLvUrKNBrSNda");

#[program]
mod voting {
    use super::*;
    pub fn initialize(
        ctx: Context<InitializePoll>,
        _poll_id: u64,
        name: String,
        description: String,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        let poll_account = &mut ctx.accounts.poll_account;
        poll_account.start_time = start_time;
        poll_account.end_time = end_time; //one day
        poll_account.name = name;
        poll_account.description = description;
        poll_account.candidate_count = 0;
        Ok(())
    }

    pub fn initialize_candiate(
        ctx: Context<InitializeCandiate>,
        _candidate: String,
        name: String,
    ) -> Result<()> {
        let poll_account = &mut ctx.accounts.poll_account;
        poll_account.candidate_count += 1;

        ctx.accounts.candidate_account.name = name;
        Ok(())
    }
    pub fn vote(ctx: Context<Vote>, _poll_id: u64, _candidate: String) -> Result<()> {
        let candidate_account = &mut ctx.accounts.candidate_account;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time > (ctx.accounts.poll_account.end_time as i64) {
            return Err(ErrorCode::VotingEnded.into());
        }

        if current_time <= (ctx.accounts.poll_account.start_time as i64) {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        candidate_account.vote_count += 1;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id:u64,candiate:String)]
pub struct InitializeCandiate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
     init_if_needed,
     payer=signer,
     space=CandiateShape::INIT_SPACE,
     seeds=[poll_id.to_le_bytes().as_ref(),candiate.as_ref()],
     bump
   )]
    pub candidate_account: Account<'info, CandiateShape>,

    #[account(mut)]
    pub poll_account: Account<'info, PollShape>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct CandiateShape {
    #[max_len(32)]
    pub name: String,
    pub vote_count: u32,
}

#[derive(Accounts)]
#[instruction(poll_id:u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer=signer,
        space=PollShape::INIT_SPACE,
        seeds=[poll_id.to_le_bytes().as_ref(),b"poll"],
        bump
    )]
    pub poll_account: Account<'info, PollShape>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PollShape {
    #[max_len(32)]
    name: String,
    #[max_len(100)]
    description: String,
    start_time: u64,
    end_time: u64,
    candidate_count: u32,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, PollShape>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump)]
    pub candidate_account: Account<'info, CandiateShape>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
}
