use super::tokenizer::{Kind, Token};
use std::cell::RefCell;
use std::collections::HashMap;

// vector, symbol (symbol)

// type Symbol = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Functor {
	Nothing,
	// Symbol
	S(Vec<String>, String),
	// Vector
	V(Vec<String>, Vec<Functor>),
}

pub type NMap = HashMap<String, Network>;

#[derive(Debug, Clone)]
pub struct Network {
	// meta: Meta,
	// pub node: Node,
	pub copy: String,
	pub path: String,
	pub keys: Vec<String>,
	pub body: Functor,
}

impl Network {
	fn new() -> Network {
		Network {
			// meta: Meta { col: 0, row: 0 },
			// node,
			copy: String::new(),
			path: String::new(),
			keys: Vec::new(),
			body: Functor::Nothing,
		}
	}
}

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
	nmap: HashMap<String, Network>,
	path: Vec<String>,
	pipe: Vec<String>,
}

pub fn parser(path: &String, tokens: Vec<Token>) -> Result<NMap, String> {
	// tokens.reverse();
	let mut state = State {
		cursor: RefCell::new(0),
		tokens,
		path: Vec::new(),
		pipe: Vec::new(),
		nmap: HashMap::new(),
	};

	state.program(path)?;
	Ok(state.nmap)
}

// parser works backwards
impl State {
	fn program(&mut self, path: &String) -> Result<String, String> {
		// let path = path.replace("\\", "/");
		let path = "program".to_string();
		self.path.push(path);
		let mut program = Network::new();
		program.path = self.path.join("/");
		self.nmap.insert(program.path.clone(), program.clone());
		//
		self.network(&[])?;
		self.path.pop();
		Ok(program.path)
	}

	fn network(&mut self, stop: &[Kind]) -> Result<Vec<String>, String> {
		let mut graph = Vec::new();

		while self.until(0, stop) {
			let path = self.label()?;
			if path.len() > 0 {
				graph.push(path);
			}
			// graph.push(self.label()?)
		}
		graph.sort();
		graph.dedup();

		Ok(graph)
	}

	// label, monads, graphs, clones are not functors
	fn label(&mut self) -> Result<String, String> {
		if self.is(0, Kind::Label) {
			let mut network = Network::new();
			let label = self.eat(Kind::Label)?.text.clone();
			self.path.push(label);
			network.body = self.monad()?;
			// self.monad()?;
			// let path = self.path.join("/");
			network.path = self.path.join("/");
			match self.nmap.get(&network.path) {
				None => {
					self.nmap.insert(network.path.clone(), network.clone());
				}
				Some(v) => {
					network.copy = v.copy.clone();
					dbg!(&network);
					self.nmap.insert(network.path.clone(), network.clone());
					// dbg!(v);
				}
			}
			self.path.pop();
			Ok(network.path)
		// Ok(path)
		} else {
			self.monad()?;
			Ok(String::new())
		}
	}
	fn monad(&mut self) -> Result<Functor, String> {
		if self.is(0, Kind::Monad) {
			let monad = self.eat(Kind::Monad)?.text.clone();
			self.pipe.push(monad);
			let functor = self.monad()?;
			self.pipe.pop();
			Ok(functor)
		} else if self.is(0, Kind::Clone) {
			self.clone()
		// 	Ok(Functor::Nothing)
		} else if self.is(0, Kind::BraL) {
			self.graph()
		} else {
			self.functor()
		}
	}
	fn graph(&mut self) -> Result<Functor, String> {
		self.eat(Kind::BraL)?;
		let mut network = Network::new();
		network.path = self.path.join("/");
		network.keys = self.network(&[Kind::BraR])?;
		self.nmap.insert(network.path.clone(), network);
		self.eat(Kind::BraR)?;
		Ok(Functor::Nothing)
	}
	fn clone(&mut self) -> Result<Functor, String> {
		let mut network = Network::new();
		network.path = self.path.join("/");
		dbg!(&network.path);
		network.copy = self.eat(Kind::Clone)?.text.clone();
		network.keys = self.network(&[Kind::BraR])?;
		self.nmap.insert(network.path.clone(), network);
		// dbg!(&network);
		self.eat(Kind::BraR)?;
		Ok(Functor::Nothing)
	}
	fn functor(&mut self) -> Result<Functor, String> {
		// let mut network = Network::new();
		// network.path = self.path.join("/");
		// network.body =
		if self.is(0, Kind::ParL) {
			self.vector()
		} else {
			self.symbol()
		}
		// self.nmap.insert(network.path.clone(), network);
		// Ok(Functor::Nothing)
	}

	fn vector(&mut self) -> Result<Functor, String> {
		let pipe = self.pipe.clone();
		self.eat(Kind::ParL)?;
		let mut symbols = Vec::new();
		while self.until(0, &[Kind::ParR]) {
			symbols.push(self.functor()?)
		}
		self.eat(Kind::ParR)?;

		Ok(Functor::V(pipe, symbols))
	}
	fn symbol(&mut self) -> Result<Functor, String> {
		let pipe = self.pipe.clone();
		Ok(Functor::S(pipe, self.eat(Kind::Token)?.text.clone()))
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
