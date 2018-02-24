mod vm_prelude;

use Mode::*;
use Code::*;

pub enum Code {
    Builtin(fn(&mut Vm, usize)),
}

pub struct Word {
    name: String,
    code: Code,
}

pub enum Mode {
    Execute,
    BlockComment{ depth: usize },
}

pub struct Vm {
    pub dict: Vec<Word>,
    pub data_stack: Vec<u8>,
    pub mode: Mode,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            dict: Self::new_dict(),
            data_stack: Vec::new(),
            mode: Execute,
        }
    }

    fn new_dict() -> Vec<Word> {
        let mut ret = Vec::new();
        ret.push(Word{ name: "/*".to_string(), code: Builtin(Self::word_start_block_comment) });
        ret.push(Word{ name: "zero".to_string(), code: Builtin(Self::word_zero) });
        ret.push(Word{ name: "set-lsbit".to_string(), code: Builtin(Self::word_set_lsbit) });
        ret.push(Word{ name: "left-shift-1".to_string(), code: Builtin(Self::word_left_shift_1) });
        ret.push(Word{ name: "print-data-stack".to_string(), code: Builtin(Self::word_print_data_stack) });
        ret
    }

    fn word_start_block_comment(&mut self, _index: usize) {
        self.mode = BlockComment{ depth: 0 };
    }

    fn word_zero(&mut self, _index: usize) {
        self.data_stack.push(0);
    }

    fn word_set_lsbit(&mut self, index: usize) {
        if let Some(v) = self.data_stack.pop() {
            self.data_stack.push(v | 1);
        } else {
            panic!("{}: Runtime error: data stack is empty.", self.dict[index].name);
        }
    }

    fn word_left_shift_1(&mut self, index: usize) {
        if let Some(v) = self.data_stack.pop() {
            self.data_stack.push(v << 1);
        } else {
            panic!("{}: Runtime error: data stack is empty.", self.dict[index].name);
        }
    }

    fn word_print_data_stack(&mut self, _index: usize) {
        println!("{:?}", self.data_stack);
    }

    fn eval_block_comment(&mut self, tok: &str, depth: usize) {
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
        let mut index_and_builtin = None;

        for (i, w) in self.dict.iter().enumerate().rev() {
            if &*w.name == tok {
                match w.code {
                    Builtin(f) => index_and_builtin = Some((i, f)),
                }
            }
        }

        if let Some((i, f)) = index_and_builtin {
            f(self, i);
        } else {
            panic!("{}: Runtime error: invalid token.", tok);
        }
    }

    fn eval(&mut self, tok: &str) {
        match self.mode {
            Execute => self.eval_execute(tok),
            BlockComment{ depth: d } => self.eval_block_comment(tok, d),
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
