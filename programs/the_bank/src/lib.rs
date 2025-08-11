use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("8fegMuK3ZrhxZNBh9woL1Nx8aiqHUpwscQQhfAaSvA2C");

#[account]
#[derive(InitSpace)]
pub struct TheBank {
pub bank_bump: u8,
pub state_bump: u8,
pub amount: u64 
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"bank", user.key().as_ref()],
        bump,
        space = 8 + TheBank::INIT_SPACE,
    )]
    pub bank: Account<'info, TheBank>,

    #[account(
        seeds = [b"bank_state", bank.key().as_ref()],
        bump,
    )]
     pub bankstate: SystemAccount<'info>,
    pub system_program: Program<'info, System>,

}

    impl<'info> Initialize<'info> {
        pub fn initialize(&mut self, bumps: &InitializeBumps, amount: u64) -> Result <()> {
            self.bank.bank_bump = bumps.bank;
            self.bank.state_bump = bumps.bankstate;
            self.bank.amount = amount;

            Ok(()) 
        }
    }


    #[derive(Accounts)]
    pub struct Deposit<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(
            mut,
            seeds = [b"bank", user.key().as_ref()],
            bump = bank.bank_bump,
        )]
        pub bank: Account<'info, TheBank>,

        #[account(
            mut,
            seeds = [b"bank_state", bank.key().as_ref()],
            bump = bank.state_bump,
        )]
        pub bankstate: SystemAccount <'info>,
        pub system_program: Program<'info, System>,
    }

    impl<'info> Deposit<'info> {
        pub fn deposit (&mut self, amount: u64) -> Result <()> {

            let cpi_program = self.system_program.to_account_info();

            let cpi_accounts = Transfer {
                from: self.user.to_account_info(),
                to: self.bankstate.to_account_info(),
            };

            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            transfer (cpi_ctx, amount)?;

            Ok(())


        }
    }

    #[derive(Accounts)]
    pub struct Withdraw<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(
            mut, 
            seeds = [b"bank", user.key().as_ref()],
            bump = bank.bank_bump,

        )]
        pub bank: Account <'info, TheBank>,

          #[account(
            mut,
            seeds = [b"bank_state", bank.key().as_ref()],
            bump = bank.state_bump,
          )]
           pub bankstate: SystemAccount <'info>,
        pub system_program: Program<'info, System>,
    }

    impl<'info> Withdraw<'info> {
        pub fn withdraw(&mut self, amount: u64) -> Result <()> {
            let cpi_program = self.system_program.to_account_info();

            let cpi_accounts = Transfer {
                from: self.bankstate.to_account_info(),
                to: self.user.to_account_info(),
            };

            let seeds = &[b"bank_state", self.bank.to_account_info().key.as_ref(), &[self.bank.state_bump]];

            let signer_seeds = &[&seeds[..]];
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer(cpi_ctx, amount)?;
            Ok(())
        }
    }

    #[derive(Accounts)]
    pub struct Close<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(
            mut, 
            seeds = [b"bank", user.key().as_ref()],
            bump = bank.bank_bump,
            close = user,
        )]
        pub bank: Account<'info, TheBank>,
        

        #[account(
            mut,
            seeds = [b"bank_state", bank.key().as_ref()],
            bump = bank.state_bump,
            
        )]
        pub bankstate: SystemAccount<'info>,
        pub system_program: Program<'info, System>,

    }

    impl<'info> Close<'info> {
        pub fn close (&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.bankstate.to_account_info(),
            to: self.user.to_account_info(),
        };
    

    let seeds = &[b"bank_state", self.bank.to_account_info().key.as_ref(), &[self.bank.state_bump]];

    let signer_seeds = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
    transfer(cpi_ctx, self.bankstate.lamports())?;

    Ok(())

    }
}


#[program]
pub mod the_bank {
    use super::*;

pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
    ctx.accounts.initialize(&ctx.bumps, amount)?;
    Ok(())

}
 pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    ctx.accounts.deposit(amount)?;
    Ok(())
 }
 pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    ctx.accounts.withdraw(amount)?;
    Ok(())
 }

pub fn close(ctx: Context<Close>, amount: u64) -> Result<()> {
    ctx.accounts.close(amount)?;
    Ok(())
} 


}
