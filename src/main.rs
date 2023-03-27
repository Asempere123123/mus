mod base;

const ITERACIONES: u32 = 1000;

fn main() {
    //Variables mazo
    let mut current_card: usize = 0;
    let mut mazo: [u8; 40];

    //Variables jugadores
    let mut player1: base::Player = base::Player {
        //Tu
        cards: [0, 0, 0, 0],
        score: [0, 0, 0, 0],
    };
    let mut player2: base::Player = base::Player {
        //Oponente
        cards: [0, 0, 0, 0],
        score: [0, 0, 0, 0],
    };

    //Variables del programa
    let mut tabla_victorias: [[u32; 10000]; 4] = [[0; 10000]; 4];
    let mut tabla_partidas_jugadas: [[u32; 10000]; 4] = [[0; 10000]; 4];
    let mut tabla_winrate: [[i64; 10000]; 4] = [[0; 10000]; 4];

    //Simulacion
    for _ in 0..ITERACIONES {
        mazo = base::new_deck(&mut current_card);

        base::repartir(&mut player1, mazo, &mut current_card);
        base::repartir(&mut player2, mazo, &mut current_card);

        base::get_hand_scores(&mut player1);
        base::get_hand_scores(&mut player2);

        for i in 0..4 {
            if player1.score[i] >= player2.score[i] {
                tabla_victorias[i][player1.score[i] as usize] += 1;
            }
            tabla_partidas_jugadas[i][player1.score[i] as usize] += 1;
        }
    }

    //calcular proporciones
    for i in 0..tabla_victorias.len() {
        for j in 0..tabla_victorias[i].len() {
            if tabla_partidas_jugadas[i][j] != 0 {
                tabla_winrate[i][j] = (tabla_victorias[i][j] / tabla_partidas_jugadas[i][j]) as i64;
            } else {
                tabla_winrate[i][j] = 0;
            }
        }
    }
    println!("Winrate: {:?}", tabla_partidas_jugadas[0]);
}
