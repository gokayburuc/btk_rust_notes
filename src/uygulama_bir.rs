// use rnd::Rng; // random range func
use std::io::stdin; // standart input from user

enum State {
    Locked,
    Unlocked,
    Failed,
}

pub fn sample_func() {
    let code = String::from("1234"); // pass is 1234
    let mut state = State::Locked; // current state is locked
    let mut entry = String::new(); // get entry

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue,
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&input) {
                    state = State::Failed;
                }
            }

            State::Failed => {
                println!("FAILED!");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED!");
                return;
            }
        }
    }
}
