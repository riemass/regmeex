use crate::{State, StateMachine};

pub fn match_input<'a>(
    input: &'a [u8],
    state_machine: &StateMachine,
    current_state: State,
) -> (State, &'a [u8]) {
    if let Some((current, rest)) = input.split_first() {
        println!("Input: |{:?}|{:?}", current, rest);
        println!("Current state: {:?}", current_state);
        for t in &state_machine.transitions {
            if t.0 == current_state && t.2 == *current {
                println!("Transition: {:?}", t);
                let (state, rest) = match_input(rest, state_machine, t.1);
                if state == state_machine.final_state {
                    return (state, rest);
                }
            } else if t.0 == current_state && t.2 == 0 {
                println!("Transition: {:?}", t);
                let (state, rest) = match_input(input, state_machine, t.1);
                if state == state_machine.final_state {
                    return (state, rest);
                }
            }
        }
    } else {
        for t in &state_machine.transitions {
            if t.0 == current_state && t.2 == 0 {
                println!("Transition: {:?}", t);
                let (state, rest) = match_input(input, state_machine, t.1);
                if state == state_machine.final_state {
                    return (state, rest);
                }
            }
        }
    }
    return (current_state, input);
}
