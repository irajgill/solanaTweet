use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_reaction(ctx: Context<AddReactionContext>, reaction: ReactionType) -> Result<()> {
    // Initialize the reaction account
    let tweet_reaction = &mut ctx.accounts.tweet_reaction;
    let tweet = &mut ctx.accounts.tweet;
    
    // Set reaction fields
    tweet_reaction.reaction_author = ctx.accounts.reaction_author.key();
    tweet_reaction.parent_tweet = tweet.key();
    tweet_reaction.reaction = reaction.clone();
    tweet_reaction.bump = ctx.bumps.tweet_reaction;
    
    // Increment the appropriate counter on the tweet
    match reaction {
        ReactionType::Like => {
            require!(tweet.likes < u64::MAX, TwitterError::MaxLikesReached);
            tweet.likes += 1;
        },
        ReactionType::Dislike => {
            require!(tweet.dislikes < u64::MAX, TwitterError::MaxDislikesReached);
            tweet.dislikes += 1;
        }
    }
    
    Ok(())
}

#[derive(Accounts)]
pub struct AddReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    
    #[account(
        init,
        payer = reaction_author,
        space = 8 + Reaction::INIT_SPACE,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref()
        ],
        bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
    
    pub system_program: Program<'info, System>,
}
