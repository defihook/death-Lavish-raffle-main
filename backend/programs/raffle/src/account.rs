use anchor_lang::prelude::*;
use std::clone::Clone;
use std::result::Result;

use crate::constants::*;
use crate::error::*;

#[account]
#[derive(Default)]
pub struct GlobalPool {
    pub super_admin: Pubkey, // 32
}

#[account(zero_copy)]
pub struct RafflePool {
    // 72+160+32*1000+8 = 32240
    pub creator: Pubkey,                  //32
    pub nft_mint: Pubkey,                 //32
    pub token_mint_first: Pubkey,         //32
    pub token_mint_second: Pubkey,        //32
    pub count: u64,                       //8
    pub no_repeat: u64,                   //8
    pub max_entrants: u64,                //8
    pub end_timestamp: i64,               //8
    pub ticket_price_first: u64,          //8
    pub ticket_price_second: u64,         //8
    pub ticket_price_sol: u64,            //8
    pub claimed: u64,                     //8
    pub winner_index: u64,                //8
    pub winner: Pubkey,                   //32
    pub entrants: [Pubkey; MAX_ENTRANTS], //32*1000
}

impl Default for RafflePool {
    #[inline]
    fn default() -> RafflePool {
        RafflePool {
            creator: Pubkey::default(),
            count: 0,
            no_repeat: 0,
            max_entrants: 0,
            end_timestamp: 0,
            ticket_price_first: 0,
            ticket_price_second: 0,
            ticket_price_sol: 0,
            claimed: 0,
            winner_index: 0,
            nft_mint: Pubkey::default(),
            token_mint_first: Pubkey::default(),
            token_mint_second: Pubkey::default(),
            winner: Pubkey::default(),
            entrants: [Pubkey::default(); MAX_ENTRANTS],
        }
    }
}
impl RafflePool {
    pub fn append(&mut self, buyer: Pubkey) {
        self.entrants[self.count as usize] = buyer;
        self.count += 1;
    }
}
