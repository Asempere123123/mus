use csv::WriterBuilder;

mod base;

const ITERACIONES: u32 = 100000000;

fn main() {
    //Variables del programa
    let mut tabla_victorias: [[u32; 10000]; 4] = [[0; 10000]; 4];
    let mut tabla_partidas_jugadas: [[u32; 10000]; 4] = [[0; 10000]; 4];
    let mut tabla_winrate: [[f64; 10000]; 4] = [[0.0; 10000]; 4];
    let mut wrt = WriterBuilder::new().from_path("output.csv").expect("No se como, pero el programa ha petado.");

    //Benchmark + Debug
    let mut iteraciones_reales: u32 = 0;
    let time = std::time::Instant::now();

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

        base::reset_player(&mut player1);
        base::reset_player(&mut player2);
        iteraciones_reales += 1;
    }

    //calcular proporciones
    for i in 0..tabla_victorias.len() {
        for j in 0..tabla_victorias[i].len() {
            if tabla_partidas_jugadas[i][j] != 0 {
                tabla_winrate[i][j] =
                    tabla_victorias[i][j] as f64 / tabla_partidas_jugadas[i][j] as f64;
            } else {
                tabla_winrate[i][j] = -1.0;
            }
        }
    }

    //Format results into csv
    let first_layer: Vec<String> = (0..10000).map(|n| n.to_string()).collect();
    wrt.write_record(&first_layer).expect("No se como, pero el programa ha petado.");
    for i in 0..tabla_winrate.len() {
        let content = tabla_winrate[i].iter().map(|&f| f.to_string()).collect::<Vec<_>>();
        wrt.write_record(&content).expect("No se como, pero el programa ha petado.");
    }

    println!("Iteraciones realizadas: {}, en {}s", iteraciones_reales, time.elapsed().as_secs());
}
