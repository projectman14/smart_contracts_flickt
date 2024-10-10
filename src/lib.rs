use anchor_lang::prelude::*;

pub mod constants;
pub mod states;
use crate::{constants::*, states::*};

declare_id!("5zQDawApmuVVhLDtDh1oQbLZ9vUiZqzWTPagtbqH3nZc");

#[program]
pub mod post_creator {
    use super::*;

    pub fn create_post(
        ctx: Context<CreatePost>,
        description: String,
        url: String,
        post_id: u8,
    ) -> Result<()> {
        let user_post = &mut ctx.accounts.user_post;
        let authority = &mut ctx.accounts.authority;

        user_post.description = description;
        user_post.url = url;
        user_post.post_id = post_id;
        user_post.like_count = 0;
        user_post.authority = authority.key();

        Ok(())
    }

    pub fn like_post(ctx: Context<LikePost>) -> Result<()> {
        let user_post = &mut ctx.accounts.user_post;

        user_post.like_count = user_post.like_count.checked_add(1).unwrap();

        let authority = &mut ctx.accounts.authority;

        Ok(())
    }

    pub fn comment_post(ctx: Context<CommentPost>, content: String) -> Result<()> {
        let user_post = &mut ctx.accounts.user_post;
        let authority = &ctx.accounts.authority;

        let comment = Comment {
            author: authority.key(),
            content,
            timestamp: Clock::get()?.unix_timestamp,
        };

        user_post.comments.push(comment);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(description: String)]
pub struct CreatePost<'info> {
    #[account(
        init,
        seeds = [b"POSTSOMETHING", authority.key().as_ref(), description.as_bytes()],
        bump,
        payer = authority,
        space = 7137 + 8
    )]
    pub user_post: Account<'info, UserPost>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct LikePost<'info> {
    #[account(
        mut,
        seeds = [b"POSTSOMETHING", authority.key().as_ref(), user_post.description.as_bytes()],
        bump,
        has_one = authority
    )]
    pub user_post: Account<'info, UserPost>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct CommentPost<'info> {
    #[account(
        mut,
        seeds = [b"POSTSOMETHING", authority.key().as_ref(), user_post.description.as_bytes()],
        bump,
        has_one = authority
    )]
    pub user_post: Account<'info, UserPost>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
