use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    // Validate comment content length
    require!(comment_content.chars().count() <= COMMENT_LENGTH, TwitterError::CommentTooLong);
    
    // Initialize the comment account
    let comment = &mut ctx.accounts.comment;
    
    // Set comment fields
    comment.comment_author = ctx.accounts.comment_author.key();
    comment.parent_tweet = ctx.accounts.tweet.key();
    comment.content = comment_content;
    comment.bump = ctx.bumps.comment;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,
    
    #[account(
        init,
        payer = comment_author,
        space = 8 + 32 + 32 + 4 + COMMENT_LENGTH + 1, // discriminator + comment_author + parent_tweet + content + bump
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            {hash::hash(comment_content.as_bytes()).to_bytes().as_ref()},
            tweet.key().as_ref()
        ],
        bump
    )]
    pub comment: Account<'info, Comment>,
    
    pub tweet: Account<'info, Tweet>,
    
    pub system_program: Program<'info, System>,
}