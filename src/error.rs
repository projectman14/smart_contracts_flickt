use anchor_lang::prelude::*;

#[error_code]
pub enum PlatformError {
    #[msg("User data cannot be empty")]
    EmptyUserData,
    #[msg("Post content cannot be empty")]
    EmptyPostContent,
    #[msg("Comment content cannot be empty")]
    EmptyCommentContent,
    #[msg("User has already liked this post")]
    AlreadyLiked,
}
