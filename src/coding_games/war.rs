


//load deck 
//check value of card 
//cmp cards 
//stack control
//update deck 
//turn control
//declare winner or pat 

#[cfg(test)]
mod tests{

    struct game_input {
     
        cards_player_1 : String,
        cards_player_2 : String,
     
    }

    struct game_input {
        nb_cards_player_1 : String,
        cards_player_1 : String,
        nb_cards_player_2 : String,
        cards_player_2 : String,
    }

    struct game_expecter {
        pat : bool, 
        winner_nb : u8,
        nb_turns : u8,
    }

    fn load_game(test_name : &str) {
        if "3 cartes" {
            3 AD KC QC 3 KH QS JC
        }
    }


    #[test]
    fn deck_store() {
        
    }
}



use std::{io, cmp::Ordering, collections::VecDeque, iter::zip};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}



struct Game {
    decks : [VecDeque<u8>; 2],
    stacks : [Vec<u8>; 2],
    empty_deck_during_battle : bool,
    turn_nb: u8,
}

impl Game {
    fn new() -> Self {
        let mut decks : [VecDeque<u8>;2] = [VecDeque::new(), VecDeque::new()];
        for group in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let n = parse_input!(input_line, i32); // the number of cards for player 1
            for i in 0..n as usize {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let card = input_line.trim().to_string(); // the n cards of player 1
                let card_value = Self::card_to_value(card);
                decks[group].push_back(card_value);
            }
        }
        Game { decks , stacks : [vec![], vec![]], empty_deck_during_battle : false ,turn_nb : 0}
    }

    fn card_to_value(card : String) -> u8 {
        let card_vals = ["0","2","3","4","5","6","7","8","9","10","J","Q","K","A"];
        let rank = &card[..card.len() - 1];
        let idx = card_vals.into_iter().position(|r| *r == rank).unwrap() + 1;
        let x = idx as u8; //u8::try_from is better
        //eprintln!("|{} for {}|", x, card);
        x
    }

    fn stack_add_deck_bottom(&mut self, winner_nb : usize) {
        let drained_stacks1 : Vec<_> = self.stacks[0].drain(..).collect();
        let drained_stacks2 : Vec<_>= self.stacks[1].drain(..).collect();
        for card in [drained_stacks1, drained_stacks2].concat() {
            //self.decks[winner_nb].push_front(card);
            self.decks[winner_nb].push_back(card);
        }
    }

    fn battle(&mut self) {
        for _ in 0..3 {
            let top_card_present0 = self.take_top_deck_to_stack(0);
            let top_card_present1 = self.take_top_deck_to_stack(1);
            if top_card_present0.is_none() || top_card_present1.is_none() {
                self.empty_deck_during_battle = true;
            }
        }
    }

    fn turn(&self, val1 : Option<u8>, val2 : Option<u8>) -> String {

        /*if val1.is_none() || val2.is_none() {
            eprintln!("! ! ! something weird happened  TURN has no 2 cards to cmp ");
            return String::from("!")
        }*/
       /* if val1.is_none() || val2.is_none() {
            return String::from("PAT");
        }*/
        let v1 = val1.unwrap();
        let v2 = val2.unwrap();
        match val1.cmp(&val2) {
            Ordering::Less => String::from("2"),
            Ordering::Greater => String::from("1"),
            Ordering::Equal => String::from("battle"),
        }
    }
    
    fn take_top_deck_to_stack(&mut self, deck_nb : usize) -> Option<u8> {
        let top = self.decks[deck_nb].pop_front();
        //if let Some(top) = top {
        if let Some(top) = top {
            //self.stacks[deck_nb].push(top);
            self.stacks[deck_nb].push(top);
        }
        top
    }

    fn declare_winner(&self) {
        let mut winner =  if self.decks[0].is_empty() { "2" } else {"1"};
        if self.empty_deck_during_battle {
            
            println!("PAT");
        } else {
            println!("{} {}", winner, self.turn_nb);
        }
    }
    
}


fn main() {
    let mut the_game = Game::new();

    

    eprint!("========== \n turn nb {}", the_game.turn_nb);
    eprintln!(" \n \n deck_len1 {:?} \n decklen2 {:?} \n stack1 {:?} \n stack2 {:?} \n ===============", the_game.decks[0], the_game.decks[1], the_game.stacks[0], the_game.stacks[1]);
    

    let mut turn_type = String::from("");
    let mut battle_happened = false;
    //let mut last_pat = false;
    //get inputs convert
    
        //let nb_turns = nb_turns + 1;
        loop {

            the_game.turn_nb += 1;
            
            let top1 = the_game.take_top_deck_to_stack(0);
            let top2 = the_game.take_top_deck_to_stack(1);

            if top1.is_none() || top2.is_none() {
                the_game.turn_nb -= 1;
                break;
            }
            
            turn_type = the_game.turn(top1, top2);

            if turn_type == "one deck empty" {
                break;
            }
            
    
            if turn_type == "1" {
                
                the_game.stack_add_deck_bottom(0);
            }
            if turn_type == "2" {
                
                the_game.stack_add_deck_bottom(1);
            }

            if turn_type == "battle" {
                /*if !battle_happened { 
                    the_game.turn_nb += 1;
                    battle_happened = true;
                }*/
                the_game.turn_nb -= 1;
                
                the_game.battle();
            }

            if the_game.empty_deck_during_battle {
                eprintln!("> > > > > > > > DECK RAN OUT 1{:?} 2{:?}", the_game.decks[0], the_game.decks[1]);
                break;
            }
            
            if (the_game.turn_nb < 20) {
            eprint!("========== \n turn nb {} \n |{} vs {}|", the_game.turn_nb,  top1.unwrap_or(0), top2.unwrap_or(0));
            eprintln!(" \n winner {} \n deck_len1 {:?} \n decklen2 {:?} \n stack1 {:?} \n stack2 {:?} \n ===============", turn_type, the_game.decks[0], the_game.decks[1], the_game.stacks[0], the_game.stacks[1]);
            }
            

        }
        eprintln!(" \n winner {} \n deck_len1 {:?} \n decklen2 {:?} \n stack1 {:?} \n stack2 {:?} \n ===============", turn_type, the_game.decks[0], the_game.decks[1], the_game.stacks[0], the_game.stacks[1]);
        
        the_game.declare_winner();
}




