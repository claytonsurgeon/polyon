use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
   Invalid,
   Skip,
   Newline,
   //
   //
   ParL,
   ParR,
   SquL,
   SquR,
   BraL,
   BraR,
   //
   // Value,
   Number,
   String,
   //
   Token,
   Point, // abc_XYZ
   Monad, // i32,
   Label, // i32:
   Clone, // point-{
}

#[derive(Debug, Clone, PartialEq)]
pub struct Meta {
   pub row: usize,
   pub col: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
   pub kind: Kind,
   pub text: String,
   pub meta: Meta,
}

pub fn tokenizer(input: &String) -> Result<Vec<Token>, String> {
   lazy_static! {
      static ref SPEC: Vec<(Kind, Regex)> =
         vec![
            (Kind::Newline, Regex::new(r"^\n").unwrap()),
            // insignificant whitespace
            (Kind::Skip, Regex::new(r"^[\t\v\f\r ]+").unwrap()),

            // Comments
            (Kind::Skip, Regex::new(r"^;.*").unwrap()),
            (Kind::Skip, Regex::new(r"^---.*").unwrap()),

            // (Kind::Number, Regex::new(r"^[[:digit:]]+\.[[:digit:]]+").unwrap()),
            // (Kind::Number, Regex::new(r"^[[:digit:]]+").unwrap()),

            // (Kind::Value, Regex::new(r#"^[^{}()\[\]:"']*-[{]"#).unwrap()),

            (Kind::Clone, Regex::new(r#"^[^[:space:],{}()\[\]:"']+[[:space:]]*[{]"#).unwrap()),
            (Kind::Monad, Regex::new(r#"^[^[:space:],{}()\[\]:"']+[,]"#).unwrap()),
            (Kind::Label, Regex::new(r#"^[^[:space:],{}()\[\]:"']+[:]"#).unwrap()),
            (Kind::Token, Regex::new(r#"^[^[:space:],{}()\[\]:"']+"#).unwrap()),


            // parens
            (Kind::ParL, Regex::new(r"^\(").unwrap()),
            (Kind::ParR, Regex::new(r"^\)").unwrap()),

            (Kind::SquL, Regex::new(r"^\[").unwrap()),
            (Kind::SquR, Regex::new(r"^\]").unwrap()),

            (Kind::BraL, Regex::new(r"^\{").unwrap()),
            (Kind::BraR, Regex::new(r"^\}").unwrap()),


            // (Kind::Value, Regex::new(r#"^"[^"]*("|$)"#).unwrap()),
            (Kind::Token, Regex::new(r#"^"[^"]*(")"#).unwrap()),
            // (Kind::Token, Regex::new(r##"^#"[^"]*"#"##).unwrap()),
            // (Kind::Token, Regex::new(r###"^##"[^"]*"##"###).unwrap()),
            // (Kind::Token, Regex::new(r####"^###"[^"]*"###"####).unwrap()),
            // (Kind::Token, Regex::new(r#####"^####"[^"]*"####"#####).unwrap()),



            (Kind::Invalid, Regex::new(r"^.").unwrap()),
         ];
   }

   let mut tokens: Vec<Token> = Vec::new();
   let mut cursor = 0;
   let mut row = 1;
   let mut col = 1;
   let length = input.len();

   'outer: while cursor < length {
      for (kind, re) in &SPEC[..] {
         match re.find(&input[cursor..]) {
            Some(mat) => {
               let token_text = &input[cursor..cursor + mat.end()];
               let text = token_text.to_string();
               let mut t = Token {
                  kind: *kind,
                  text,
                  meta: Meta { col, row },
               };
               col += mat.end();

               match kind {
                  Kind::Newline => {
                     row += 1;
                     col = 1;
                  }
                  Kind::Skip => {}
                  Kind::Monad => {
                     t.text = t.text[..t.text.len() - 1].to_string();
                     tokens.push(t);
                  }
                  Kind::Label => {
                     t.text = t.text[..t.text.len() - 1].to_string();
                     tokens.push(t);
                  }
                  Kind::Clone => {
                     // t.text = t.text[..t.text.len() - 2].to_string();
                     tokens.push(t);
                  }
                  _ => {
                     tokens.push(t);
                  }
               }

               cursor += mat.end();
               continue 'outer;
            }
            None => {}
         }
      }
   }
   // tokens.reverse();
   Ok(tokens)
}
