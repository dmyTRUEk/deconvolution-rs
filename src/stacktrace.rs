//! Error stacktrace



#[derive(Debug, Clone)]
pub struct Stacktrace {
    stack: Vec<&'static str>,
}

impl Stacktrace {
    pub const fn empty() -> Self {
        Self { stack: vec![] }
    }

    pub fn new(str: &'static str) -> Self {
        Self { stack: vec![str] }
    }

    pub fn last(&self) -> &str {
        self.stack.last().unwrap()
    }

    pub fn push(&mut self, str: &'static str) {
        self.stack.push(str);
    }

    pub fn pushed(&self, str: &'static str) -> Self {
        let mut self_ = self.clone();
        self_.push(str);
        self_
    }

    // pub fn pop(&mut self) -> Option<&str> {
    //     unreachable!();
    //     self.stack.pop()
    // }

    pub fn panic(&self, final_msg: &str) -> ! {
        let stack_str = self.stack
            .iter()
            .map(|s| format!("`{s}`"))
            .collect::<Vec<String>>()
            .join(" -> ");
        panic!("{stack_str}: {final_msg}")
    }

    pub fn panic_not_found(&self) -> ! {
        self.panic("not found")
    }

    pub fn panic_cant_parse_as(&self, type_: &str) -> ! {
        self.panic(&format!("can't parse as {type_}"))
    }
}

