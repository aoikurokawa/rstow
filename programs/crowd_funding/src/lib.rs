use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("8tY4ZQ8LzVgU27cxi1VvRu3WJc3jw531gNRWZWP6zWyL");

#[program]
pub mod crowd_funding {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        state.projects = Vec::new();
        Ok(())
    }

    pub fn create_project(ctx: Context<CreateProject>, _new_project: IProject) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        let current_project = &mut state.number_of_project;
        let next_project_id = *current_project + 1;

        state.projects.push(_new_project);
        Ok(())
    }

    pub fn support_project(
        ctx: Context<SupportProject>,
        project_id: u64,
        amount: u64,
    ) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        let all_projects = &mut state.projects;
        // let project: IProject;

        let now = Clock::get().unwrap().unix_timestamp;

        for iter_project in all_projects {
            if iter_project.project_id == project_id {
                // project = iter_project;
                if iter_project.deadline < now {
                    msg!("The project is over!");
                    // return Err(ProgramError::);
                }

                let rent_exemption =
                    Rent::get()?.minimum_balance(ctx.accounts.authority.data_len());
                if **ctx.accounts.authority.lamports.borrow() - rent_exemption < amount {
                    msg!("Insufficient balance");
                    return Err(ProgramError::InsufficientFunds);
                }

                anchor_lang::solana_program::system_instruction::transfer(
                    ctx.accounts.authority.key,
                    ctx.program_id,
                    amount,
                );
                // if let Some(x) = all_projects.get_mut(&project_id) {
                //     x.current_amount += amount
                // }
                iter_project.current_amount += amount;
            }
        }

        Ok(())
    }

    pub fn achieve_project(ctx: Context<AchieveProject>, project_id: u64) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        let all_projects = &mut state.projects;

        let now = Clock::get().unwrap().unix_timestamp;

        for iter_project in all_projects {
            if iter_project.project_id == project_id {
                // project = iter_project;
                if iter_project.deadline > now {
                    msg!("The project is under");
                    // return Err(ProgramError::);
                }

                let current_amount = iter_project.current_amount;

                if iter_project.goal_amount >= current_amount {
                    msg!("Succeeded your project");
                    anchor_lang::solana_program::system_instruction::transfer(
                        ctx.program_id,
                        ctx.accounts.authority.key,
                        current_amount,
                    );

                    iter_project.current_amount = 0;
                    iter_project.achieved = true;
                }
            }
        }

        Ok(())
    }

    pub fn delete_project(ctx: Context<CreateProject>, _project_id: u64) -> ProgramResult {
        let state = &mut ctx.accounts.state;
        let all_projects = &mut state.projects;

        let mut project_index: usize = 0;

        for (index, iter_project) in all_projects.iter().enumerate() {
            if iter_project.project_id == _project_id {
                // project = iter_project;
                if !iter_project.achieved {
                    msg!("Not achieved yet");
                    // return Err(ProgramError::);
                }

                project_index = index;

                msg!("The project has removed");
            }
        }

        all_projects.remove(project_index);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority)]
    pub state: Account<'info, State>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProject<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SupportProject<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AchieveProject<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct State {
    pub authority: Pubkey,
    pub number_of_project: u64,
    pub projects: Vec<IProject>,
}

#[account]
pub struct IProject {
    project_id: u64,
    representative: Pubkey,
    current_amount: u64,
    goal_amount: u64,
    deadline: i64,
    achieved: bool,
}
