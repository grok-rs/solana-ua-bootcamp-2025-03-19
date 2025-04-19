use anchor_lang::prelude::*;

declare_id!("F7nfFyvQgoePiLTM8gB1FHPX95QqhVh7jqJ2m6kvuYxy");
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorites(context: Context<SetFavorites>, number: u64, color: String) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);
        msg!(
            "User {}'s favorite number is {} and favorite color is: {}",
            user_public_key,
            number,
            color
        );

        context
            .accounts
            .favorites
            .set_inner(Favorites { number, color });
        Ok(())
    }

    pub fn update_favorites(
        context: Context<UpdateFavorites>,
        args: UpdateFavoritesArgs,
    ) -> Result<()> {
        let favorites = &mut context.accounts.favorites;

        if let Some(number) = args.number {
            favorites.number = number;
        }
        if let Some(color) = args.color {
            favorites.color = color;
        }

        let user_public_key = context.accounts.user.key();
        msg!(
            "User {}'s favorite number is {} and favorite color is: {}",
            user_public_key,
            favorites.number,
            favorites.color
        );

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateFavoritesArgs {
    pub number: Option<u64>,
    pub color: Option<String>,
}

#[derive(Accounts)]
pub struct UpdateFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"favorites", user.key().as_ref()],
        bump,
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
