use super::tokenizer::{Kind, Token};
use std::cell::RefCell;
use std::collections::HashMap;

// vector, symbol (symbol)

// type Symbol = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Functor {
	Nothing,
	// Symbol
	S(String),
	// Vector
	V(Vec<Functor>),
}

pub type NMap = HashMap<String, Network>;

#[derive(Debug, Clone)]
pub struct Network {
	// meta: Meta,
	pub node: Node,
	pub copy: String,
	pub path: String,
	pub keys: Vec<String>,
	pub body: Functor,
}

impl Network {
	fn new(node: Node) -> Network {
		Network {
			// meta: Meta { col: 0, row: 0 },
			node,
			copy: String::new(),
			path: String::new(),
			keys: Vec::new(),
			body: Functor::Nothing,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
	Nothing,
	//
	Graph,
	Clone,
	Point,
}

struct State {
	cursor: RefCell<usize>,
	tokens: Vec<Token>,
	points: HashMap<String, Network>,
}

pub fn parser(tokens: Vec<Token>) -> Result<Network, String> {
	// tokens.reverse();
	let mut state = State {
		cursor: RefCell::new(0),
		tokens,
		points: HashMap::new(),
	};

	state.program()
}

// parser works backwards
impl State {
	fn program(&mut self) -> Result<Network, String> {
		Ok(Network::new(Node::Nothing))
	}
}

impl State {
	fn eat(&mut self, kind: Kind) -> Result<&Token, String> {
		match self.get(0) {
			Some(t) => {
				*self.cursor.borrow_mut() += 1;
				if t.kind == kind {
					Ok(t)
				} else {
					Err(format!(
						"UnexpectedToken: {:?} of {:?} on line {}\nExpected token of name: {:?}",
						t.text, t.kind, t.meta.row, kind
					))
				}
			}
			None => Err("UnexpectedEndOfInput".to_string()),
		}
	}

	fn get(&self, offset: usize) -> Option<&Token> {
		if *self.cursor.borrow() + offset < self.tokens.len() {
			Some(&self.tokens[*self.cursor.borrow() + offset])
		} else {
			None
		}
	}

	fn is(&self, offset: usize, stop: Kind) -> bool {
		match self.get(offset) {
			Some(t) => t.kind == stop,
			None => false,
		}
	}

	fn until(&self, offset: usize, stops: &[Kind]) -> bool {
		match self.get(offset) {
			Some(t) => {
				for stop in stops {
					if t.kind == *stop {
						return false;
					}
				}
				true
			}
			None => false,
		}
	}
}
