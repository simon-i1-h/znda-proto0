use Mode::*;

pub enum Mode {
    Execute,
    BlockComment{ depth: usize },
}

pub struct Vm {
    pub data_stack: Vec<u8>,
    pub mode: Mode,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            data_stack: Vec::new(),
            mode: Execute,
        }
    }

    fn eval_comment(&mut self, tok: &str, depth: usize) {
        if tok == "*/" {
            if depth == 0 {
                self.mode = Execute;
            } else {
                debug_assert!(depth - 1 < depth);
                self.mode = BlockComment{ depth: depth - 1 };
            }
        } else if tok == "/*" {
                debug_assert!(depth < depth + 1);
            self.mode = BlockComment{ depth: depth + 1 };
        }
    }

    fn eval_execute(&mut self, tok: &str) {
        if tok == "/*" {
            self.mode = BlockComment{ depth: 0 };
        } else if tok == "zero" {
            self.data_stack.push(0);
        } else if tok == "set-lsbit" {
            if let Some(v) = self.data_stack.pop() {
                self.data_stack.push(v | 1);
            } else {
                panic!("{}: Runtime error: data stack is empty.", tok);
            }
        } else if tok == "left-shift-1" {
            if let Some(v) = self.data_stack.pop() {
                self.data_stack.push(v << 1);
            } else {
                panic!("{}: Runtime error: data stack is empty.", tok);
            }
        } else if tok == "print-data-stack" {
            println!("{:?}", self.data_stack);
        } else {
            panic!("{}: Runtime error: invalid token.", tok);
        }
    }

    fn eval(&mut self, s: &str) {
        match self.mode {
            Execute => self.eval_execute(s),
            BlockComment{ depth: d } => self.eval_comment(s, d),
        }
    }

    pub fn eval_string(&mut self, mut s: &str) {
        s = s.trim_left();

        while let Some(len) = s.find(char::is_whitespace) {
            let (tok, rest) = s.split_at(len);
            s = rest.trim_left();
            self.eval(tok);
        }
        s = s.trim();
        if !s.is_empty() {
            self.eval(s);
        }
    }
}
