use crate::parser::*;

pub type State = u32;

pub struct StateGenerator {
    state: State,
}

impl StateGenerator {
    pub fn new() -> Self {
        Self { state: 0 }
    }

    pub fn next_state(&mut self) -> State {
        let ret = self.state;
        self.state += 1;
        ret
    }
}

type Transition = (State, State, u8);

#[derive(Debug)]
pub struct StateMachine {
    pub entry_state: State,
    pub final_state: State,
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
}

impl StateMachine {
    pub fn new(sg: &mut StateGenerator) -> Self {
        let entry_state = sg.next_state();
        let final_state = sg.next_state();

        Self {
            entry_state,
            final_state,
            states: vec![entry_state, final_state],
            transitions: vec![],
        }
    }
}

fn alternate(mut outer: StateMachine, mut inner: StateMachine) -> StateMachine {
    outer.states.append(&mut inner.states);
    outer.transitions.append(&mut inner.transitions);

    outer
        .transitions
        .push((outer.entry_state, inner.entry_state, 0));
    outer
        .transitions
        .push((inner.final_state, outer.final_state, 0));

    outer
}

fn combine(sm1: StateMachine, mut sm2: StateMachine) -> StateMachine {
    let entry_state = sm1.entry_state;
    let final_state = sm2.final_state;
    let mut states = sm1.states;
    states.append(&mut sm2.states);
    let mut transitions = sm1.transitions;
    // TODO
    transitions.push((sm1.final_state, sm2.entry_state, 0));
    transitions.append(&mut sm2.transitions);

    StateMachine {
        entry_state,
        final_state,
        states,
        transitions,
    }
}

pub fn create_fsm(ast: RegexAST) -> Result<StateMachine, String> {
    let mut state_generator = StateGenerator::new();
    create_fsm_inner(ast, &mut state_generator)
}

fn create_fsm_inner(
    ast: RegexAST,
    state_generator: &mut StateGenerator,
) -> Result<StateMachine, String> {
    let mut states: Vec<State> = Vec::new();
    let mut transitions = Vec::<Transition>::new();

    match ast {
        RegexAST::Match(charset) => {
            let entry_state = state_generator.next_state();
            let final_state = state_generator.next_state();
            for c in charset.chars().into_iter() {
                transitions.push((entry_state, final_state, c));
            }
            states.push(entry_state);
            states.push(final_state);
            return Ok(StateMachine {
                entry_state,
                final_state,
                states,
                transitions,
            });
        }
        RegexAST::Exprs(exprs) => {
            let sms: Result<Vec<StateMachine>, _> = exprs
                .into_iter()
                .map(|expr| create_fsm_inner(expr, state_generator))
                .collect();
            let fin = sms?
                .into_iter()
                .reduce(|sm1, sm2| combine(sm1, sm2))
                .unwrap();
            Ok(fin)
        }
        RegexAST::Alterations(exprs) => {
            let sms: Result<Vec<StateMachine>, _> = exprs
                .into_iter()
                .map(|expr| create_fsm_inner(expr, state_generator))
                .collect();
            let fin = sms?
                .into_iter()
                .fold(StateMachine::new(state_generator), |acc, sm| {
                    alternate(acc, sm)
                });
            Ok(fin)
        }
        RegexAST::Quantifier(quantifier_type, expr) => { 
            let mut expr = create_fsm_inner(*expr, state_generator)?;
            match quantifier_type {
                QuantifierType::ZeroOrOne => {
                    expr.transitions.push((expr.entry_state, expr.final_state, 0));
                    return Ok(expr);
                }, 
                QuantifierType::ZeroOrMore => {
                    let new_entry = state_generator.next_state();
                    let new_final = state_generator.next_state();
                    expr.states.push(new_entry);
                    expr.states.push(new_final);
                    expr.transitions.push((expr.final_state, expr.entry_state, 0));
                    expr.transitions.push((new_entry, expr.entry_state, 0));
                    expr.entry_state = new_entry;
                    expr.transitions.push((expr.final_state, new_final, 0));
                    expr.final_state = new_final;
                    expr.transitions.push((expr.entry_state, expr.final_state, 0));
                }
                QuantifierType::OneOrMore => {
                    let new_entry = state_generator.next_state();
                    let new_final = state_generator.next_state();
                    expr.states.push(new_entry);
                    expr.states.push(new_final);
                    expr.transitions.push((expr.final_state, expr.entry_state, 0));
                    expr.transitions.push((new_entry, expr.entry_state, 0));
                    expr.entry_state = new_entry;
                    expr.transitions.push((expr.final_state, new_final, 0));
                    expr.final_state = new_final;
                }
            }
            Ok(expr)
        }
    }
}

