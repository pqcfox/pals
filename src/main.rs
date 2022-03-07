use crate::prompt::prompt_user;
use crate::prompt::parse_index;

mod freq;
mod prompt;
mod set_one;

fn get_set_fns() -> Vec<Vec<fn()>> {
    vec![
        set_one::get_problem_fns()
    ]
}

fn prompt_index(name: &str) -> usize {
    let prompt = format!("Enter {}: ", name);
    let index_str = prompt_user(&prompt);
    match parse_index(&index_str) {
        Some(index) => index,
        None => {
            println!("Invalid {} number!", name);
            std::process::exit(1)
        }
    }
}

fn run(set: usize, exercise: usize) {
    let set_fns = get_set_fns();
    if let Some(exercise_fns) = set_fns.get(set) {

        if let Some(exercise_fn) = exercise_fns.get(exercise) {
            exercise_fn()
        } else {
            println!("Not that many exercises in set {}!", set + 1);
        }
    } else {
        println!("Not that many sets!");
    }
}

fn main() {
    let set = prompt_index("set");
    let exercise = prompt_index("exercise");
    run(set, exercise)
}
