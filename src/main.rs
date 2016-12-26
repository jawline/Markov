extern crate rand;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::vec::Vec;
use std::collections::VecDeque;
use rand::distributions::{IndependentSample, Range};

type Prefix = VecDeque<String>;
type State = HashMap<Prefix, Vec<String>>;

pub struct Markov {
	state: State,
	prefix: Prefix,
	leading: usize
}

impl Markov {
	pub fn new(leading: usize) -> Markov {

		let mut n = Markov {
			state: State::new(),
			prefix: Prefix::new(),
			leading: leading
		};

		//Pad the initial prefix with rubbish
		n.pad();

		n
	}

	/**
	 * Adds a word from the source text (Should be added in order)
	 *
	 * For some state, prefix and word
	 * let LEADING_WORDS = self.leading
	 *
	 * If the prefix is larger than LEADING_WORDS then
	 * add a transition entry from prefix -> word then
	 * remove the first element of prefix
	 * 
	 * Finally add word to the end of the prefix
	 *
	 * This creates a State of Prefix -> Words which has
	 * each prefix be exactly LEADING_WORDS long.
	 * 
	 * Increading LEADING_WORDS will reduce variation but make
	 * text seem more normal
	 */
	pub fn add(&mut self, word: &str) {
		let p_size = self.prefix.len();
		if p_size == self.leading {
			self.state.entry(self.prefix.clone()).or_insert(Vec::new()).push(word.to_string());
			self.prefix.pop_front();
		}
		self.prefix.push_back(word.to_string());
	}

	pub fn finalize(&mut self) {
		self.pad();
	}

	fn select(follows: &Vec<String>) -> String {
	    let between = Range::new(0, follows.len());
	    let mut rng = rand::thread_rng();
		follows[between.ind_sample(&mut rng)].to_string()
	}

	/**
	 * Generates the next word randomly
	 * 
	 * Given some prefix and state
	 * 
	 * let follow = all the words which follow the current prefix in state
	 * let the next word equal a random word in follow
	 * Finally remove the first element of the prefix and add the new word
	 * to the end of the prefix. This acts to advance the prefix so that a word
	 * which follows the selected will follow next time this method is called
	 */
	pub fn next(&mut self) -> String {
		let nword = Markov::select(self.state.entry(self.prefix.clone()).or_insert(Vec::new()));

		//Advance the prefix by one word
	    self.prefix.pop_front();

	    //Add the selected word to the prefix end
	    self.prefix.push_back(nword.to_string());

	    nword
	}

	fn pad(&mut self) {
		for _ in 0..self.leading {
			self.add("\n");
		}
	}
}

pub fn from_file(path: &str) -> Markov {
	let mut m = Markov::new(1);

	let mut data = "".to_string();

	File::open(path).expect("Unable to open file").read_to_string(&mut data).expect("Unable to read string");

	for d in data.split(" ") {
		m.add(d);
	}

	m.finalize();
	m
}

fn main() {
	let mut m = from_file("./sample");

	for _ in 0..100000000 {
		print!("{} ", m.next());
	}
}
