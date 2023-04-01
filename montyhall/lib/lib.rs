use rand::Rng;
use std::collections::BTreeMap;

type StrategyFunction = fn(Step, usize, &mut rand::rngs::ThreadRng, Option<bool>, bool) -> usize;

pub fn print_monty_sim(_: u32) {
    let mut rng = rand::thread_rng();

    let num_simulations = 10_000;

    let mut success_rates = BTreeMap::new();

    const STRATEGIES: &[(&str, StrategyFunction)] = &[
        ("Alice", alice_strategy),
        ("Bob", bob_strategy),
        ("Carol", carol_strategy),
        ("Dave", dave_strategy),
        ("Erin", erin_strategy),
        ("Frank", frank_strategy),
        ("Gina", gina_strategy),
    ];

    for (name, strategy) in STRATEGIES.iter() {
        let mut wins = 0;
        let mut gina_previous_strategy = None;
        let mut gina_won_previous_game = false;

        for _ in 0..num_simulations {
            let prize_door = rng.gen_range(1..=3);
            let mut contestant_door = strategy(Step::Choose, 0, &mut rng, gina_previous_strategy, gina_won_previous_game);

            let monty_door = monty_opens_door(prize_door, contestant_door, &mut rng);

            contestant_door = strategy(Step::Switch, monty_door, &mut rng, gina_previous_strategy, gina_won_previous_game);

            if contestant_door == prize_door {
                wins += 1;
            }

            if *name == "Gina" {
                gina_previous_strategy = Some(gina_won_previous_game);
                gina_won_previous_game = contestant_door == prize_door;
            }
        }

        success_rates.insert(*name, wins as f64 / num_simulations as f64);
    }

    for (name, rate) in success_rates.iter() {
        println!("{}: {:.2}%", name, rate * 100.0);
    }
}

enum Step {
    Choose,
    Switch,
}

fn alice_strategy(step: Step, _: usize, _: &mut rand::rngs::ThreadRng, _: Option<bool>, _: bool) -> usize {
    match step {
        Step::Choose => 1,
        Step::Switch => 1,
    }
}

fn bob_strategy(step: Step, monty_door: usize, _: &mut rand::rngs::ThreadRng, _: Option<bool>, _: bool) -> usize {
    match step {
        Step::Choose => 1,
        Step::Switch => 6 - monty_door - 1,
    }
}

fn carol_strategy(_: Step, monty_door: usize, rng: &mut rand::rngs::ThreadRng, _: Option<bool>, _: bool) -> usize {
    let choices = [1, 2, 3].iter().cloned().filter(|&door| door != monty_door).collect::<Vec<usize>>();
    choices[rng.gen_range(0..choices.len())]
}

fn dave_strategy(step: Step, monty_door: usize, rng: &mut rand::rngs::ThreadRng, _: Option<bool>, _: bool) -> usize {
    match step {
        Step::Choose => rng.gen_range(1..=3),
        Step::Switch => {
            let contestant_initial_door = rng.gen_range(1..=3);
            6 - monty_door - contestant_initial_door
        },
    }
}

fn erin_strategy(step: Step, monty_door: usize, rng: &mut rand::rngs::ThreadRng, _: Option<bool>, _: bool) -> usize {
    static mut CONTESTANT_INITIAL_DOOR: usize = 0;

    match step {
        Step::Choose => {
            let chosen_door = rng.gen_range(1..=3);
            unsafe {
                CONTESTANT_INITIAL_DOOR = chosen_door;
            }
            chosen_door
        }
        Step::Switch => {
            unsafe {
                6 - monty_door - CONTESTANT_INITIAL_DOOR
            }
        }
    }
}
fn frank_strategy(step: Step, monty_door: usize, _: &mut rand::rngs::ThreadRng, _: Option<bool>, _: bool) -> usize {
    match step {
        Step::Choose => 1,
        Step::Switch => {
            if monty_door != 2 {
                2
            } else {
                1
            }
        }
    }
}

fn gina_strategy(step: Step, monty_door: usize, rng: &mut rand::rngs::ThreadRng, previous_strategy: Option<bool>, won_previous_game: bool) -> usize {
    let strategy = if previous_strategy.unwrap_or(true) ^ won_previous_game {
        bob_strategy
    } else {
        alice_strategy
    };

    strategy(step, monty_door, rng, None, false)
}

fn monty_opens_door(prize_door: usize, contestant_door: usize, rng: &mut rand::rngs::ThreadRng) -> usize {
    let remaining_doors = [1, 2, 3].iter().cloned().filter(|&door| door != prize_door && door != contestant_door).collect::<Vec<usize>>();
    remaining_doors[rng.gen_range(0..remaining_doors.len())]
}