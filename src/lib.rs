pub struct Vm {
    pub data_stack: Vec<u8>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            data_stack: Vec::new(),
        }
    }

    fn eval(&mut self, s: &str) {
        if s == "zero" {
            self.data_stack.push(0);
        } else if s == "set-lsbit" {
            if let Some(v) = self.data_stack.pop() {
                self.data_stack.push(v | 1);
            } else {
                panic!("set-lsbit: Runtime error: data stack is empty.");
            }
        } else if s == "left-shift-1" {
            if let Some(v) = self.data_stack.pop() {
                self.data_stack.push(v << 1);
            } else {
                panic!("left-shift-1: Runtime error: data stack is empty.");
            }
        } else if s == "print_data_stack" {
            println!("{:?}", self.data_stack);
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
