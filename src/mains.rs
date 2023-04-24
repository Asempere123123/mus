use csv::Writer;

mod base;

const ITERACIONES: u32 = 1000000;

fn main() {
    //Variables del programa
    let mut tabla_victorias: [[f32; 50]; 5] = [[0.0; 50]; 5];
    let mut tabla_partidas_jugadas: [[f32; 50]; 5] = [[0.0; 50]; 5];
    let mut tabla_winrate: [[f64; 50]; 5] = [[0.0; 50]; 5];
    let mut wrt = Writer::from_path("output.csv").unwrap();

    //Benchmark + Debug
    let mut iteraciones_reales: u32 = 0;
    let time = std::time::Instant::now();
    let mut victorias: f32 = 0.0;
    let mut partidas_jugadas: f32 = 0.0;

    //Variables mazo
    let mut mazo;

    //Variables jugadores
    let mut player1: base::Player = base::new_player(); //Tu
    let mut player2: base::Player = base::new_player(); //Oponente
    let mut player3: base::Player = base::new_player(); //Jugador tercero random(seria como pareja)

    //Variables simulación(jaque mate)
    let mut cuenta_real: i16;

    //Simulacion
    for _ in 0..ITERACIONES {
        mazo = base::new_deck();

        base::repartir(&mut player1, &mut mazo);
        base::repartir(&mut player2, &mut mazo);
        base::repartir(&mut player3, &mut mazo);

        base::get_hand_scores(&mut player1);
        base::get_hand_scores(&mut player2);

        cuenta_real = base::calculate_count(&player3, &player1) + 25; //el 25 es para que entre en el array

        for i in 0..4 {
            // Par
            if i == 2 && (player1.score[2] == 0 || player2.score[2] == 0) { // si no hay par, no hacer nada

                // Punto
            } else if i == 3 && player1.score[3] < 31 && player2.score[3] < 31 {
                if player1.score[3] > player2.score[3] {
                    tabla_victorias[4][cuenta_real as usize] += 1.0;
                    victorias += 1.0;
                } else if player1.score[3] == player2.score[3] {
                    tabla_victorias[4][cuenta_real as usize] += 0.5;
                    victorias += 0.5;
                }
                tabla_partidas_jugadas[4][cuenta_real as usize] += 1.0;
                partidas_jugadas += 1.0;
                
            } else if i == 3 && (player1.score[3] < 31 || player2.score[3] < 31) {
            } else {
                if player1.score[i] > player2.score[i] {
                    tabla_victorias[i][cuenta_real as usize] += 1.0;
                    victorias += 1.0;
                } else if player1.score[i] == player2.score[i] {
                    tabla_victorias[i][cuenta_real as usize] += 0.5;
                    victorias += 0.5;
                }
                tabla_partidas_jugadas[i][cuenta_real as usize] += 1.0;
                partidas_jugadas += 1.0;
            }

            iteraciones_reales += 1;
        }

        //Resetear todos los valores
        base::reset_player(&mut player1);
        base::reset_player(&mut player2);
        base::reset_player(&mut player3);
    }

    //calcular proporciones
    for i in 0..tabla_victorias.len() {
        for j in 0..tabla_victorias[i].len() {
            if tabla_partidas_jugadas[i][j] != 0.0 {
                tabla_winrate[i][j] =
                    tabla_victorias[i][j] as f64 / tabla_partidas_jugadas[i][j] as f64;
            } else {
                tabla_winrate[i][j] = -1.0;
            }
        }
    }

    //Format results into csv
    let first_layer: Vec<String> = (-(25 as i32)..((tabla_victorias[0].len()-25) as i32))
    .map(|n| n.to_string()).collect();
    
    wrt.write_record(&first_layer)
        .unwrap();
    for i in 0..tabla_winrate.len() {
        let content = tabla_winrate[i]
            .iter()
            .map(|&f| f.to_string())
            .collect::<Vec<_>>();
        wrt.write_record(&content)
            .unwrap();
    }

    let winrate = victorias/partidas_jugadas;
    println!(
        "Overall winrate: {}, {}, {}",
        winrate,
        victorias,
        partidas_jugadas
    );

    println!(
        "Iteraciones realizadas: {}, en {}s",
        iteraciones_reales,
        time.elapsed().as_secs()
    );
}
