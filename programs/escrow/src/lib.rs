use anchor_lang::prelude::*;

declare_id!("6LZd2BokfNYboL3ZunXVnNHxoBVQ7XAo3Sh42vX8gdBa");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<InitializeEscrow>, amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.client = ctx.accounts.client.key();
        escrow.freelancer = ctx.accounts.freelancer.key();
        escrow.amount = amount;
        escrow.bump = ctx.bumps.vault;

        // Transfer SOL to the vault
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.client.key(),
            &ctx.accounts.vault.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.client.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        )?;
        Ok(())
    }

    pub fn release_funds(ctx: Context<ReleaseFunds>) -> Result<()> {
        let escrow = &ctx.accounts.escrow;
        require_keys_eq!(ctx.accounts.client.key(), escrow.client);

        let amount = escrow.amount;
        let seeds = &[
            b"detork-escrow",
            escrow.client.as_ref(),
            escrow.freelancer.as_ref(),
            &[escrow.bump],
        ];
        let signer_seed = &[&seeds[..]];
        // Transfer funds from PDA vault to freelancer
        **ctx
            .accounts
            .vault
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;
        **ctx
            .accounts
            .freelancer
            .to_account_info()
            .try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(init, payer=client, space=8+32+32+8+1)]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub client: Signer<'info>,

    pub freelancer: UncheckedAccount<'info>,

    #[account(mut, seeds=[b"detork-escrow", client.key().as_ref(), freelancer.key().as_ref()], bump)]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseFunds<'info> {
    #[account(mut, has_one=client)]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub client: Signer<'info>,

    #[account(mut)]
    pub freelancer: SystemAccount<'info>,

    #[account(mut, seeds=[b"detork-escrow", client.key().as_ref(), freelancer.key().as_ref()], bump=escrow.bump)]
    pub vault: UncheckedAccount<'info>,
}

#[account]
pub struct Escrow {
    pub client: Pubkey,
    pub freelancer: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
