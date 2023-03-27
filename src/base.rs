use rand::seq::SliceRandom;
use rand::thread_rng;

pub const BARAJA_ESPAÑOLA: [u8; 40] = [
    1, 2, 3, 4, 5, 6, 7, 10, 11, 12, 1, 2, 3, 4, 5, 6, 7, 10, 11, 12, 1, 2, 3, 4, 5, 6, 7, 10, 11,
    12, 1, 2, 3, 4, 5, 6, 7, 10, 11, 12,
];

/*
fn main() {
    //Variables del mazo
    let mut current_card: usize = 0;
    let mazo: [u8; 40] = new_deck(&mut current_card);

    //Variables jugadores
    let mut player1: Player = Player { //Tu
        cards: [0, 0, 0, 0],
        score: [0, 0, 0, 0],
    };
    let mut player2: Player = Player { //Oponente
        cards: [0, 0, 0, 0],
        score: [0, 0, 0, 0],
    };

    repartir(&mut player1, mazo, &mut current_card);
    repartir(&mut player2, mazo, &mut current_card);

    get_hand_scores(&mut player1);
    get_hand_scores(&mut player2);

    println!("Player 1 has: {:?}", player1);
    println!("Player 2 has: {:?}", player2);

    println!("mazo: {:?}, wich ended in position: {}", mazo, current_card);
}
*/

//Player
#[derive(Debug)]
pub struct Player {
    pub(crate) cards: [u8; 4],
    pub(crate) score: [u16; 4],
}

//Utility Funcitons
pub fn new_deck(current_card: &mut usize) -> [u8; 40] {
    *current_card = 0;

    let mut mazo = BARAJA_ESPAÑOLA;
    mazo.shuffle(&mut thread_rng());
    mazo
}

pub fn get_card(mazo: [u8; 40], current_card: &mut usize) -> u8 {
    *current_card += 1;
    mazo[*current_card - 1]
}

pub fn get_hand_scores(player: &mut Player) {
    //Tabla a la grande y chica(no se hacer hashmaps)
    let grande: [u8; 13] = [0, 2, 2, 9, 3, 4, 5, 6, 0, 0, 7, 8, 9];
    let chica: [u8; 13] = [0, 9, 9, 2, 8, 7, 6, 5, 0, 0, 4, 3, 2];

    let mut grande_scores: [u8; 4] = [0, 0, 0, 0];
    let mut chica_scores: [u8; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        grande_scores[i] = grande[player.cards[i] as usize];
        chica_scores[i] = chica[player.cards[i] as usize];
    }
    grande_scores.sort();
    chica_scores.sort();

    let mut grande_score: u16 = 0;
    let mut chica_score: u16 = 0;
    for i in 0..4 {
        grande_score += grande_scores[i] as u16 * u16::pow(10, i.try_into().unwrap());
        chica_score += chica_scores[i] as u16 * u16::pow(10, i.try_into().unwrap());
    }

    //Pares
    let mut pair_tier: u8 = 0; // 0 --> no pair | 1--> Dobles | 2--> triple |3 --> duples
    let mut pair_score: u8 = 0;
    let mut pair_already: u8 = 0;

    if grande_scores[0] == grande_scores[1] && grande_scores[2] == grande_scores[3] {
        pair_tier = 3;
        if grande_scores[0] > grande_scores[2] {
            pair_score = grande_scores[0];
        } else {
            pair_score = grande_scores[2];
        }
    } else {
        for i in 0..grande_scores.len() - 1 {
            if grande_scores[i] == grande_scores[i + 1] {
                if pair_already == 1 {
                    if pair_tier < 2 {
                        pair_tier = 2;
                        pair_score = grande_scores[i];
                    }
                }

                if pair_tier < 1 {
                    pair_tier = 1;
                    pair_score = grande_scores[i];
                }
                pair_already += 1;
            }
        }
    }

    let pair_score = u16::pow(10, pair_tier as u32) * pair_score as u16;

    //puntijuego
    let mut suma_punto: u8 = 0;
    for i in 0..4 {
        if [3, 10, 11, 12].contains(&player.cards[i]) {
            suma_punto += 10;
        } else if [1, 2].contains(&player.cards[i]) {
            suma_punto += 1;
        } else {
            suma_punto += player.cards[i];
        }
    }

    let mut juego_score: u16;
    if suma_punto == 31 {
        juego_score = 99;
    } else if suma_punto == 32 {
        juego_score = 98;
    } else {
        juego_score = suma_punto as u16;
    }
    if suma_punto >= 31 {
        juego_score *= 100;
    }

    player.score = [grande_score, chica_score, pair_score, juego_score];
}

pub fn repartir(player: &mut Player, mazo: [u8; 40], current_card: &mut usize) {
    for i in 0..4 {
        if player.cards[i] == 0 {
            player.cards[i] = get_card(mazo, current_card);
        }
    }
}

pub fn reset_player(player: &mut Player) {
    player.cards = [0, 0, 0, 0];
    player.score = [0, 0, 0, 0];
}
