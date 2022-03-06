use std::io;
use std::io::Write;
mod set_one;

fn prompt_index(prompt: &str) -> Option<usize> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut result = String::new();
    io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line");

    let maybe_read_num = result.trim().parse();

    if let Ok(read_num) = maybe_read_num {
        match read_num {
            0 => None,
            _ => Some(read_num - 1)
        }
    } else {
        None
    }
}

fn get_set_fns() -> Vec<Vec<fn()>> {
    vec![
        set_one::get_problem_fns()
    ]
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
    let maybe_set = prompt_index("Enter set: ");
    if let Some(set) = maybe_set {
        let maybe_exercise = prompt_index("Enter exercise: ");
        if let Some(exercise) = maybe_exercise {
            run(set, exercise);
        } else{
            println!("Invalid exercise number!");
        }
    } else {
        println!("Invalid set number!");
    }
}
