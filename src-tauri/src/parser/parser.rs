use std::{collections::HashMap, u32};

//Note: we implement ids as a Vector of ints representing both its depth and which child it is

#[derive(Debug)]
pub struct Document {
    pub block: Block,
}

impl Document {
    fn parse(source_path: String) {
        println!("hello");
    }
    fn write(parser_output_path: String, definitions_output_path: String) {
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub id: Vec<u32>,
    pub nodes: Vec<Node>,
}

impl Block {
    fn new() -> Self {
        Self {
            id: Vec::new(),
            nodes: Vec::new(),
        }
    }

    fn to_json(&self) -> String {
        let nodes_json: Vec<String> = self.nodes.iter()
            .map(|node| node.to_json())
            .collect();

        format!(r#"{{"nodes":[{}]}}"#, nodes_json.join(","))
    }
}


// helper to convert an id to a String for to_json()
fn id_to_str(id: &Vec<u32>) -> String { 
    let mut conv = String::new();

    for (i, e) in id.iter().enumerate() {
        if i > 0 {
            conv.push('/');
        }

        conv.push_str(&e.to_string());
    }

    conv
}


#[derive(Debug, Clone)]
pub enum Node {
    Text {
        id: Vec<u32>,
        text: String,
    },
    
    // Command Types 
    Def {
        id: Vec<u32>,
        ident: String,
        body: Block,
    },
    Local {
        id: Vec<u32>,
        ident: String,
        body: Block,
    },
    Section {
        id: Vec<u32>,
        ident: String,
        body: Block,
    },
    Img {
        id: Vec<u32>,
        path: String,
    },
    Latex {
        id: Vec<u32>,
        expr: String,
    },
    Custom {
        id: Vec<u32>,
        name: String,
        args: Vec<Block>,
    },
}

impl Node {
    fn to_json(&self) -> String {
        match self {
            Node::Text { id, text } => format!(r#"{{"type":"text", "id":"{}", "text":"{}"}}"#, id_to_str(id), text),

            Node::Def { id, ident, body } => {
                format!(r#"{{"type":"def","id":"{}", "ident":"{}","body":{}}}"#, id_to_str(id), ident, body.to_json())
            }

            Node::Local { id, ident, body } => {
                format!(r#"{{"type":"local", "id":"{}", "ident":"{}","body":{}}}"#, id_to_str(id), ident, body.to_json())
            }

            Node::Section { id, ident, body } => {
                format!(r#"{{"type":"section", "id":"{}", "ident":"{}","body":{}}}"#, id_to_str(id), ident, body.to_json())
            }

            Node::Img { id, path } => {
                format!(r#"{{"type":"img", "id":"{}", "path":"{}"}}"#, id_to_str(id), path)
            }

            Node::Latex { id, expr } => {
                format!(r#"{{"type":"latex", "id":"{}", "expr":"{}"}}"#, id_to_str(id), expr)
            }

            Node::Custom { id, name, args } => {
                let args_json: Vec<String> = args.iter().map(|b| b.to_json()).collect();
                format!(
                    r#"{{"type":"custom", "id":"{}", "name":"{}","args":[{}]}}"#,
                    id_to_str(id),
                    name,
                    args_json.join(",")
                )
            }
        }
    }
}

pub struct Parser {
    input: Vec<char>,
    pos: usize,
    defs: HashMap<String, Vec<Node>>,
    id_stack: Vec<u32>

}

impl Parser {

    pub fn new(input: String) -> Self {
        let normalized = input.replace("\r\n", "\n").replace('\r', "\n");

        Self {
            input: normalized.chars().collect(),
            pos: 0,
            defs: HashMap::new(),
            id_stack: Vec::new()
        }
    }

    // ----- ID Functions ---- //

    fn current_id(&self) -> Vec<u32> {
        self.id_stack.clone()
    }

    fn with_child_id<T>(&mut self, child_index: u32, f: impl FnOnce(&mut Self) -> T) -> T {
        self.id_stack.push(child_index);
        let result = f(self);
        self.id_stack.pop();
        result
    }
    
    // ----- Utils ----- //

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    fn advance_word(&mut self) -> Option<String> {
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                break;
            }
            s.push(c);
            self.advance();
        }

        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
    
    // advances until provided token break_token is found, and return consumed tokens
    fn advance_until(&mut self, break_token: char) -> Option<String> {

        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c == '\\' {
                self.advance();
                let next = match self.advance() {
                    Some(ch) => match ch {
                        'n' => break,
                        _ => ch
                    }
                    None => panic!("finished parsing before encountering expected token {}", break_token)
                };
                s.push(next);
            }
            else if c == break_token {
                break;
            }
            s.push(c);
            self.advance();
        }

        if s.is_empty() {
            None
        } else {
            Some(s)
        } 
    }

    // consumes characters until provided token break_token is found
    fn consume_until(&mut self, break_token: char) {
        while let Some(c) = self.peek() {
            if c == '\\' {
                self.advance();
                self.advance();
            }
            else if c == break_token {
                break;
            }
            self.advance();
        }
    }

    // ----- Command Parsers ----- //
    
    // would be nice to have a generic command parser that lets you auto parse (...) {...} without
    // having to manually repeat every time

    // Generic function to parse all text of form ("text here") { "block stuff here" }
    // Want to extend to make Block part optional to just parse form ("text here")
    fn parse_command_structure(&mut self) -> (String, Block) {

        self.consume_until('(');
        self.advance(); // consume '('

        let ident = self
            .advance_until(')')
            .expect("Must define on some term (cannot be empty)");

        self.advance(); // consume ')'

        self.consume_until('{');
        self.advance(); // consume '{'

        let body = self.parse_until(Some('}'));

        self.advance(); // consume '}'

        (ident, body)
    }

    fn parse_def(&mut self) -> Node {

        let (ident, body) = self.parse_command_structure();
        let node = Node::Def { id: self.current_id(), ident: ident.clone(), body };

        self.defs.entry(ident)
            .or_insert_with(Vec::new)
            .push(node.clone());

        node
    }

    fn parse_local(&mut self) -> Node {

        let (ident, body) = self.parse_command_structure();
        let node = Node::Local { id: self.current_id(), ident: ident.clone(), body };

        self.defs.entry(ident)
            .or_insert_with(Vec::new)
            .push(node.clone());

        node
    }

    fn parse_section(&mut self) -> Node {
        let (ident, body) = self.parse_command_structure();

        Node::Section { id: self.current_id(), ident, body }
    }

    fn parse_img(&mut self) -> Node {

        self.consume_until('(');
        self.advance(); // consume '('

        let path = self
            .advance_until(')')
            .expect("Must define on some term (cannot be empty)");
        // do some extra validation for valid path structure here

        self.advance(); // consume ')'

        Node::Img { id: self.current_id(), path }

    }

    fn parse_latex(&mut self) -> Node {

        self.consume_until('(');
        self.advance(); // consume '('

        let expr = self
            .advance_until(')')
            .expect("Must define on some term (cannot be empty)");
        // need to implement avoid breaking on '@' here, also may want to fully parse latex here?
        // (Check to see if there are any rust latex parser crates?)

        self.advance(); // consume ')'

        Node::Latex { id: self.current_id(), expr }
    }


    fn parse_command(&mut self) -> Node {
        let s = self.advance_word();

        match s {
            Some(command) => match command.as_str() {
                "def" => self.parse_def(),
                "local" => self.parse_local(),
                "section" => self.parse_section(),
                "img" => self.parse_img(),
                "latex" => self.parse_latex(),

                _ => panic!("Unrecognized function name {}", command)
            
            }

            None => panic!("command cannot have value 'None'")
        }
    }
    
    // NOTE: Only used for Node::Text blocks (in order to allow for interruption with special chars), NOT used for parsing text making up idents (currently using consume_until and advance_until for that)
    fn parse_text(&mut self, break_token: Option<char>) -> Node {
        let mut text = String::new();

        while let Some(c) = self.peek() {
            match c {
                
                '\\' => {
                    self.advance();
                    if let Some(next) = self.advance() {
                        text.push(next);
                    }
                }

                c if break_token.is_some_and(|bt| c == bt) => break,
                '@' => break,
                
                _ => {
                    text.push(c);
                    self.advance();
                }
            }
        }
        Node::Text { id: self.current_id(), text }
    }

    pub fn parse_until(&mut self, break_token: Option<char>) -> Block {
        let mut block = Block {
            id: self.current_id(),
            nodes: Vec::new()
        };

        while let Some(c) = self.peek() {
            if break_token.is_some_and(|bt| c == bt) {
                break;
            }

            match c {
                '@' => {
                    self.advance();
                    
                    let child_index = block.nodes.len() as u32;
                    let node = self.with_child_id(child_index, |parser| {
                        parser.parse_command()
                    });

                    block.nodes.push(node);
                }

                _ => {
                    block.nodes.push(self.parse_text(None)); // if no special character encountered, proceed to parse as normal text
                }
            }
        }

        block
    }
}
