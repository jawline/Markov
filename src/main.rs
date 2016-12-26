extern crate rand;

use std::collections::HashMap;
use std::vec::Vec;
use std::collections::VecDeque;
use rand::distributions::{IndependentSample, Range};

type Prefix = VecDeque<String>;
type State = HashMap<Prefix, Vec<String>>;

const NPREF: usize = 2;

fn add(prefix: &mut Prefix, state: &mut State, word: &str) {
	let p_size = prefix.len();
	if p_size == NPREF {
		state.entry(prefix.clone()).or_insert(Vec::new()).push(word.to_string());
		prefix.pop_front();
	}
	prefix.push_back(word.to_string());
}

fn next(prefix: &mut Prefix, state: &mut State) -> String {
	let follows = state.entry(prefix.clone()).or_insert(Vec::new());
    let between = Range::new(0, follows.len());
    let mut rng = rand::thread_rng();
    println!("All of between {:?}", follows);
	let w = &follows[between.ind_sample(&mut rng)];
    prefix.pop_front(); // advance
    prefix.push_back(w.to_string());
    w.to_string()
}

fn pad_prefix(prefix: &mut Prefix, state: &mut State) {
	for _ in 0..NPREF {
		add(prefix, state, "\n");
	}
}

fn main() {
	let mut state = State::new();
	let mut prefix = Prefix::new();
	
	pad_prefix(&mut prefix, &mut state);

	add(&mut prefix, &mut state, "a");
	add(&mut prefix, &mut state, "brown");
	add(&mut prefix, &mut state, "dog");
	add(&mut prefix, &mut state, "a");
	add(&mut prefix, &mut state, "brown");
	add(&mut prefix, &mut state, "cat");
	add(&mut prefix, &mut state, "a");
	add(&mut prefix, &mut state, "brown");
	add(&mut prefix, &mut state, "green");
	add(&mut prefix, &mut state, "a");
	add(&mut prefix, &mut state, "brown");
	add(&mut prefix, &mut state, "poo");

	pad_prefix(&mut prefix, &mut state);

	println!("{} {} {}", next(&mut prefix, &mut state),next(&mut prefix, &mut state),next(&mut prefix, &mut state));
}
