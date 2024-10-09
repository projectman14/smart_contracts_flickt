use anchor_lang::prelude::*;
use std::mem::size_of;

pub mod constant;
pub mod error;
pub mod states;

use crate::{constant::*, error::*, states::*};

declare_id!("FHi9b593PHHVjcrvRBXjAmRd6JsHkWCoLmC8CHdkDUy3");

#[program]
pub mod social_media {
    use super::*;

    pub fn initialize_platform(ctx: Context<InitializePlatform>) -> Result<()> {
        let platform_account = &mut ctx.accounts.platform_account;
        platform_account.authority = ctx.accounts.authority.key();
        platform_account.post_count = 0;
        Ok(())
    }

    pub fn signup_user(
        ctx: Context<SignupUser>,
        name: String,
        avatar: String
    ) -> Result<()> {
        require!(!name.is_empty() && !avatar.is_empty(), PlatformError::EmptyUserData);

        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.name = name;
        user_profile.avatar = avatar;
        user_profile.post_count = 0;

        Ok(())
    }

    pub fn create_post(
        ctx: Context<CreatePost>,
        content: String
    ) -> Result<()> {
        require!(!content.is_empty(), PlatformError::EmptyPostContent);

        let platform_account = &mut ctx.accounts.platform_account;
        let post_account = &mut ctx.accounts.post_account;
        let user_profile = &mut ctx.accounts.user_profile;

        platform_account.post_count = platform_account.post_count.checked_add(1).unwrap();
        user_profile.post_count = user_profile.post_count.checked_add(1).unwrap();

        post_account.authority = ctx.accounts.authority.key();
        post_account.content = content;
        post_account.author = user_profile.key();
        post_account.likes = 0;
        post_account.comment_count = 0;
        post_account.created_at = ctx.accounts.clock.unix_timestamp;

        emit!(PostEvent {
            post_id: post_account.key(),
            author: user_profile.key(),
            content: post_account.content.clone(),
        });

        Ok(())
    }

    pub fn like_post(ctx: Context<LikePost>) -> Result<()> {
        let post_account = &mut ctx.accounts.post_account;
        let user_profile = &ctx.accounts.user_profile;

        require!(!post_account.likes_by.contains(&user_profile.key()), PlatformError::AlreadyLiked);

        post_account.likes = post_account.likes.checked_add(1).unwrap();
        post_account.likes_by.push(user_profile.key());

        emit!(LikeEvent {
            post_id: post_account.key(),
            user: user_profile.key(),
        });

        Ok(())
    }

    pub fn create_comment(
        ctx: Context<CreateComment>,
        content: String
    ) -> Result<()> {
        require!(!content.is_empty(), PlatformError::EmptyCommentContent);

        let post_account = &mut ctx.accounts.post_account;
        let comment_account = &mut ctx.accounts.comment_account;
        let user_profile = &ctx.accounts.user_profile;

        post_account.comment_count = post_account.comment_count.checked_add(1).unwrap();

        comment_account.authority = ctx.accounts.authority.key();
        comment_account.content = content;
        comment_account.author = user_profile.key();
        comment_account.post = post_account.key();
        comment_account.created_at = ctx.accounts.clock.unix_timestamp;

        emit!(CommentEvent {
            comment_id: comment_account.key(),
            post_id: post_account.key(),
            author: user_profile.key(),
            content: comment_account.content.clone(),
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(
        init,
        seeds = [PLATFORM_TAG],
        bump,
        payer = authority,
        space = size_of::<PlatformAccount>() + 8
    )]
    pub platform_account: Account<'info, PlatformAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SignupUser<'info> {
    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = size_of::<UserProfile>() + USER_NAME_LENGTH + USER_URL_LENGTH + 8
    )]
    pub user_profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(content: String)]
pub struct CreatePost<'info> {
    #[account(mut, seeds = [PLATFORM_TAG], bump)]
    pub platform_account: Account<'info, PlatformAccount>,

    #[account(
        init,
        seeds = [POST_TAG, platform_account.post_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = size_of::<PostAccount>() + TEXT_LENGTH + 8
    )]
    pub post_account: Account<'info, PostAccount>,

    #[account(mut, has_one = authority)]
    pub user_profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct LikePost<'info> {
    #[account(mut)]
    pub post_account: Account<'info, PostAccount>,

    pub user_profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(content: String)]
pub struct CreateComment<'info> {
    #[account(mut)]
    pub post_account: Account<'info, PostAccount>,

    #[account(
        init,
        seeds = [COMMENT_TAG, post_account.key().as_ref(), post_account.comment_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = size_of::<CommentAccount>() + TEXT_LENGTH + 8
    )]
    pub comment_account: Account<'info, CommentAccount>,

    pub user_profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>,
}

#[event]
pub struct PostEvent {
    pub post_id: Pubkey,
    pub author: Pubkey,
    pub content: String,
}

#[event]
pub struct LikeEvent {
    pub post_id: Pubkey,
    pub user: Pubkey,
}

#[event]
pub struct CommentEvent {
    pub comment_id: Pubkey,
    pub post_id: Pubkey,
    pub author: Pubkey,
    pub content: String,
}