mod base;

fn main() {
    let mut current_card: usize = 0;
    let mazo: [u8; 40] = base::new_deck(&mut current_card);

    //Variables jugadores
    let mut player1: base::Player = base::Player { //Tu
        cards: [0, 0, 0, 0],
        score: [0, 0, 0, 0],
    }; 
    let mut player2: base::Player = base::Player { //Oponente
        cards: [0, 0, 0, 0],
        score: [0, 0, 0, 0],
    };

    //just example code
    base::repartir(&mut player1, mazo, &mut current_card);
    base::repartir(&mut player2, mazo, &mut current_card);

    base::get_hand_scores(&mut player1);
    base::get_hand_scores(&mut player2);

    println!("player 1: {:?}", player1);
    println!("player 2: {:?}", player2);
}