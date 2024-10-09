use anchor_lang::prelude::*;

#[account]
pub struct PlatformAccount {
    pub authority: Pubkey,
    pub post_count: u64,
}

#[account]
pub struct UserProfile {
    pub authority: Pubkey,
    pub name: String,
    pub avatar: String,
    pub post_count: u64,
}

#[account]
pub struct PostAccount {
    pub authority: Pubkey,
    pub content: String,
    pub author: Pubkey,
    pub likes: u64,
    pub likes_by: Vec<Pubkey>,
    pub comment_count: u64,
    pub created_at: i64,
}

#[account]
pub struct CommentAccount {
    pub authority: Pubkey,
    pub content: String,
    pub author: Pubkey,
    pub post: Pubkey,
    pub created_at: i64,
}
