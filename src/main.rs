extern crate rand;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::vec::Vec;
use std::collections::VecDeque;
use rand::distributions::{IndependentSample, Range};

type Prefix = VecDeque<String>;
type State = HashMap<Prefix, Vec<String>>;

const NPREF: usize = 1;

pub struct Markov {
	state: State,
	prefix: Prefix
}

impl Markov {
	pub fn new() -> Markov {
		let mut n = Markov {
			state: State::new(),
			prefix: Prefix::new()
		};
		n.pad();
		n
	}

	pub fn add(&mut self, word: &str) {
		let p_size = self.prefix.len();
		if p_size == NPREF {
			self.state.entry(self.prefix.clone()).or_insert(Vec::new()).push(word.to_string());
			self.prefix.pop_front();
		}
		self.prefix.push_back(word.to_string());
	}

	pub fn finalize(&mut self) {
		self.pad();
	}

	fn next(&mut self) -> String {
		let follows = self.state.entry(self.prefix.clone()).or_insert(Vec::new());
	    let between = Range::new(0, follows.len());
	    let mut rng = rand::thread_rng();
		let w = &follows[between.ind_sample(&mut rng)];
	    self.prefix.pop_front(); // advance
	    self.prefix.push_back(w.to_string());
	    w.to_string()
	}

	fn pad(&mut self) {
		for _ in 0..NPREF {
			self.add("\n");
		}
	}
}

fn main() {
	
	let mut m = Markov::new();

	let mut data = "".to_string();
	File::open("./sample").expect("Unable to open file").read_to_string(&mut data).expect("Unable to read string");

	for d in data.split(" ") {
		m.add(d);
	}

	m.finalize();

	for _ in 0..100 {
		print!("{} ", m.next());
	}
}
