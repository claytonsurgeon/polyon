use super::tokenizer::{Kind, Token};
use std::cell::RefCell;

// active: network, functor, element
// s-expr: vector, symbol, bottom

#[derive(Debug, Clone, PartialEq)]
pub enum Program {
	Nothing,
	Element(String),
	Functor(Vec<Program>, Vec<Program>),
	// Include(Vec<Program>),
	Network(String, Vec<Program>, Vec<Program>),
}

// Program: the composition of
// 	network: a graph of functors and elements
// 	element: a base/bottom value
// 	functor: a vector of functors and elements
// 	nothing:

struct State {
	cursor: RefCell<usize>,
	tokens: Vec<Token>,
}

pub fn parser(tokens: Vec<Token>) -> Result<Program, String> {
	// tokens.reverse();
	let mut state = State {
		cursor: RefCell::new(0),
		tokens,
	};

	state.program()
}

// parser works backwards
impl State {
	fn program(&mut self) -> Result<Program, String> {
		let (programs, networks) = self.collect(&[])?;
		Ok(Program::Network("program".to_string(), programs, networks))
	}

	fn network(&mut self) -> Result<Program, String> {
		self.eat(Kind::ParL)?;
		let label = self.eat(Kind::Semicolon)?.text.clone();
		let (programs, networks) = self.collect(&[Kind::ParR])?;
		self.eat(Kind::ParR)?;
		Ok(Program::Network(label, programs, networks))
	}
	fn functor(&mut self) -> Result<Program, String> {
		self.eat(Kind::ParL)?;
		let (programs, networks) = self.collect(&[Kind::ParR])?;
		self.eat(Kind::ParR)?;
		Ok(Program::Functor(programs, networks))
	}
	fn element(&mut self) -> Result<Program, String> {
		Ok(Program::Element(self.eat(Kind::Token)?.text.clone()))
	}
	fn nothing(&mut self) -> Result<Program, String> {
		self.eat(Kind::ParL)?;
		self.eat(Kind::ParR)?;
		Ok(Program::Nothing)
	}
	fn collect(&mut self, stop: &[Kind]) -> Result<(Vec<Program>, Vec<Program>), String> {
		let mut body = Vec::new();
		let mut kids = Vec::new();
		while self.until(0, stop) {
			if self.is(0, Kind::ParL) {
				if self.is(1, Kind::Semicolon) {
					kids.push(self.network()?)
				} else if self.is(1, Kind::ParR) {
					body.push(self.nothing()?)
				} else {
					body.push(self.functor()?)
				}
			} else {
				body.push(self.element()?)
			};
		}
		Ok((body, kids))
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

	fn until(&self, offset: usize, stop: &[Kind]) -> bool {
		match self.get(offset) {
			Some(t) => {
				for s in stop {
					if t.kind == *s {
						return false;
					}
				}
				true
			}
			None => false,
		}
	}
}
