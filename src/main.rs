use crate::prompt::prompt_user;

mod prompt;
mod set_one;

fn parse_index(num_str: &str) -> Option<usize> {
    let maybe_read_num = num_str.trim().parse();

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
    let set_str = prompt_user("Enter set: ");
    if let Some(set) = parse_index(&set_str) {
        let exercise_str = prompt_user("Enter exercise: ");
        if let Some(exercise) = parse_index(&exercise_str) {
            run(set, exercise);
        } else{
            println!("Invalid exercise number!");
        }
    } else {
        println!("Invalid set number!");
    }
}
