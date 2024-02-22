use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint,Token, TokenAccount, Transfer};

use crate::{
    _main::main_state::MainState, 
    constants::{
        SEED_MAIN_STATE, 
        SEED_OFFER
    }, 
    offer::offer_state::OfferState, 
    utils::{tranfer_token_from_offeror_state, tranfer_token}, error::MyError, events
};

#[derive(AnchorSerialize, AnchorDeserialize,  Default,Clone, Copy, Debug)]
pub struct EditOfferInput{
    new_requested_token_amount: Option<u64>, 
    new_min_requested_token_amount: Option<u64>
}

pub fn edit_offer(ctx:Context<AEditOffer>, input: EditOfferInput) ->Result<()>{
    let offeror = ctx.accounts.offeror.to_account_info();
    // let offeror_ata = ctx.accounts.offeror_ata.to_account_info();
    let offer_state = &mut ctx.accounts.offer_state_account;
    // let offer_state_ata = ctx.accounts.offer_state_account_ata.to_account_info();
    // let fee_receiver_ata = ctx.accounts.fee_receiver_ata.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let main_state = &ctx.accounts.main_state_account;

    // let offered_token_decimal = Mint::try_deserialize(&mut &* ctx.accounts.offered_token.data.take())?.decimals;
    // let requested_token_decimal = Mint::try_deserialize(&mut &* ctx.accounts.requested_token.data.take())?.decimals;

    if !offer_state.is_active{
        return anchor_lang::err!(MyError::OfferNotActive);
    }
    if let Some(amount) = input.new_requested_token_amount.clone(){
        if amount == 0{
            return anchor_lang::err!(MyError::ZeroRequestedAmount);
        }
        offer_state.requested_amount = amount;
    }
    if let Some(amount) = input.new_min_requested_token_amount.clone(){
        if amount > offer_state.requested_amount{
            return anchor_lang::err!(MyError::TooHighAmount);
        }
        offer_state.min_requested_amount = amount;
    }
    
    // let (_, bump) = Pubkey::find_program_address(&[
    //         SEED_OFFER, 
    //         offer_state.offeror.as_ref(),
    //         offer_state.offered_token.key().as_ref(),
    //         offer_state.requested_token.key().as_ref(),
    //     ], 
    //     ctx.program_id
    // );

    //if let Some(amount) = input.inc_offered_token_amount{
    //    if amount > 0{
    //        let fees = (main_state.fee_rate * amount as f64) as u64;
    //        offer_state.offered_amount += amount;

    //        //NOTE: Tranfering Fees
    //        tranfer_token(
    //            offeror_ata.to_account_info(), 
    //            fee_receiver_ata.to_account_info(), 
    //            offeror.to_account_info(), 
    //            token_program.to_account_info(), 
    //            fees
    //        )?;

    //        //NOTE: Tranfering the offered_token amount to the program account
    //        tranfer_token(
    //            offeror_ata.to_account_info(), 
    //            offer_state_ata.to_account_info(),
    //            offeror.to_account_info(), 
    //            token_program.to_account_info(),
    //            amount
    //        )?;
    //    }
    //}

    //if let Some(amount) = input.dec_offered_token_amount{
    //    if amount > 0{
    //        offer_state.offered_amount -= amount;
    //        //NOTE: Tranfering the offered_token amount from the program account to offeror
    //        tranfer_token_from_offeror_state(
    //            offeror.to_account_info(), 
    //            offeror_ata.to_account_info(), 
    //            offer_state, 
    //            offer_state_ata.to_account_info(),
    //            token_program.to_account_info(), 
    //            amount
    //        )?;
    //    }
    //}

    emit!(events::OfferUpdated{
        offer_id: ctx.accounts.offer_state_account.key(),
        new_requested_token_amount: input.new_requested_token_amount,
        new_min_requested_token_amount: input.new_min_requested_token_amount,
    });
    Ok(())
}


#[derive(Accounts)]
pub struct AEditOffer<'info>{
    pub offeror:Signer<'info>,

    #[account(
        seeds = [SEED_MAIN_STATE],
        bump,
    )]
    pub main_state_account: Account<'info, MainState>,

    #[account(
        mut,
        seeds = [
            SEED_OFFER, 
            offer_state_account.init_time.to_le_bytes().as_ref(),
            offeror.key().as_ref(),
            offer_state_account.offered_token.key().as_ref(),
            offer_state_account.requested_token.key().as_ref(),
        ],
        bump,
    )]
    pub offer_state_account: Account<'info, OfferState>,

    ///CHECK:
    // #[account(
    //     mut,
    //     // token::mint = offered_token,
    //     // token::authority = offeror,
    // )]
    // // pub offeror_ata: Account<'info ,TokenAccount>,
    // pub offeror_ata: AccountInfo<'info>,
    
    // #[account(
    //     mut,
    //     token::mint = offered_token,
    //     token::authority = offer_state_account,
    // )]
    // pub offer_state_account_ata: Account<'info ,TokenAccount>,

    // #[account(
    //     mut,
    //     token::mint = offered_token,
    //     token::authority = main_state_account.fee_receiver,
    // )]
    // pub fee_receiver_ata: Account<'info ,TokenAccount>,

    pub token_program: Program<'info, Token>,
}
