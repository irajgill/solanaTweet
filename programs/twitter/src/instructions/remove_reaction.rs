use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    let tweet_reaction = &ctx.accounts.tweet_reaction;
    let tweet = &mut ctx.accounts.tweet;
    
    // Decrement the appropriate counter on the tweet
    match tweet_reaction.reaction {
        ReactionType::Like => {
            require!(tweet.likes > 0, TwitterError::MinLikesReached);
            tweet.likes -= 1;
        },
        ReactionType::Dislike => {
            require!(tweet.dislikes > 0, TwitterError::MinDislikesReached);
            tweet.dislikes -= 1;
        }
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    
    #[account(
        mut,
        close = reaction_author,
        has_one = reaction_author,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref()
        ],
        bump = tweet_reaction.bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}
