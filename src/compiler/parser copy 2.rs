use super::tokenizer::{Kind, Token};
use std::cell::RefCell;
// use std::collections::HashMap;

// vector, symbol (symbol)

// type Symbol = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
	Nothing,
	// Symbol
	S(String),
	// Vector
	V(Vec<Ast>),
}

// pub type NMap = HashMap<String, Network>;

// #[derive(Debug, Clone)]
// pub struct Network {
// 	// meta: Meta,
// 	// pub node: Node,
// 	pub copy: String,
// 	pub path: String,
// 	pub keys: Vec<String>,
// 	pub body: Functor,
// }

// impl Network {
// 	fn new() -> Network {
// 		Network {
// 			// meta: Meta { col: 0, row: 0 },
// 			// node,
// 			copy: String::new(),
// 			path: String::new(),
// 			keys: Vec::new(),
// 			body: Functor::Nothing,
// 		}
// 	}
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum Node {
// 	Nothing,
// 	//
// 	Graph,
// 	Clone,
// 	Point,
// }

struct State {
	cursor: RefCell<usize>,
	tokens: Vec<Token>,
	// nmap: HashMap<String, Network>,
	// path: Vec<String>,
	// pipe: Vec<String>,
}

pub fn parser(path: &String, tokens: Vec<Token>) -> Result<Ast, String> {
	// tokens.reverse();
	let mut state = State {
		cursor: RefCell::new(0),
		tokens,
		// path: Vec::new(),
		// pipe: Vec::new(),
		// nmap: HashMap::new(),
	};

	state.program()
	// Ok(state.nmap)
}

// parser works backwards
impl State {
	fn program(&mut self) -> Result<Ast, String> {
		// Ok(Ast::V(
		// 	Ast::S("graph".to_string())

		// ))
		self.network(&[])
	}

	fn network(&mut self, stop: &[Kind]) -> Result<Ast, String> {
		let mut vector = Vec::new();
		while self.until(0, stop) {
			vector.push(self.element()?)
		}
		Ok(Ast::V(vector))
	}

	fn element(&mut self) -> Result<Ast, String> {
		if self.is(0, Kind::Label) {
			self.label()
		} else {
			Ok(Ast::Nothing)
		}
	}

	fn label(&mut self) -> Result<Ast, String> {
		let label = self.eat(Kind::Label)?.text.clone();
		let mut vector = Vec::new();
		vector.push(Ast::S("label".to_string()));
		vector.push(Ast::S(label));
		vector.push(self.element()?);
		Ok(Ast::V(vector))
	}

	fn monad(&mut self) -> Result<Ast, String> {
		let label = self.eat(Kind::Label)?.text.clone();
		let mut vector = Vec::new();
		vector.push(Ast::S("label".to_string()));
		vector.push(Ast::S(label));
		vector.push(self.element()?);
		Ok(Ast::V(vector))
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
