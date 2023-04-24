mod base;

use csv::Writer;

const ITERACIONES: u32 = 1000000;

fn main() {
    let mut partidas_jugadas: f32 = 0.0;
    let mut partidas_cuenta: [f32; 50] = [0.0; 50];

    for _ in 0..ITERACIONES {
        let mut mazo = base::new_deck();
        let mut player1 = base::new_player();
        let mut player2 = base::new_player();

        base::repartir(&mut player1, &mut mazo);
        base::repartir(&mut player2, &mut mazo);

        let cuenta = base::calculate_count(&player1, &player2);

        partidas_cuenta[cuenta as usize + 25] += 1.0;
        partidas_jugadas += 1.0;
    }

    let proporcion_partidas_cuenta: Vec<String> = partidas_cuenta
        .iter()
        .map(|&f| (f/partidas_jugadas).to_string())
        .collect();

    //csv
    let mut wrt = Writer::from_path("output.csv").unwrap();
    let first_layer: Vec<String> = (-(25 as i32)..((25) as i32))
    .map(|n| n.to_string()).collect();
    
    wrt.write_record(first_layer).unwrap();
    wrt.write_record(proporcion_partidas_cuenta).unwrap();
}
