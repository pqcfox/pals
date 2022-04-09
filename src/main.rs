use crate::prompt::prompt_user;
use crate::prompt::parse_index;

mod set_one;
mod prompt;


fn get_set_fns() -> Vec<Vec<fn()>> {
    vec![
        set_one::get_problem_fns()
    ]
}

fn main() {
    let set_fns = get_set_fns();

    let set: usize = prompt_user("Enter set: ")
        .parse()
        .expect("Invalid index!");

    let exercise: usize = prompt_user("Enter exercise: ")
        .parse()
        .expect("Invalid index!");

    set_fns[set - 1][exercise - 1]();
}
