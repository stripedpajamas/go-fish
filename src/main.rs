use rand::prelude::*;

fn main() {
    let deck = get_shuffled_deck();

    println!("{:?}", deck);
}

fn get_shuffled_deck() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut deck = (0..4).flat_map(|_| (0..13)).collect::<Vec<i32>>();
    deck.shuffle(&mut rng);
    deck
}