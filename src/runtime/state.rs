use super::scope::Scope;
use super::{Value, Ref, Signal, ExprRes, expr_err};

pub enum HeapVal {
    List(Vec<Box<Value>>),
}


pub struct State {
    stack: Vec<Scope>,

    //
    
}

impl State {
    pub fn new() -> Self {
        State {
            stack: Vec::new(),
            //heap_tracker: Vec::new(),
            //heap: Vec::new(),
        }
    }

    fn clean_heap(&mut self) {
        /*let s = self.heap_tracker.pop();
        let size = self.heap.len() - s;
        self.heap.truncate(size);*/
        
    }

    pub fn scope_push(&mut self) {
        self.stack.push(Scope::new());
    }

    pub fn scope_pop(&mut self) {
        self.stack.pop();
        self.clean_heap();
    }

    /*pub fn get_scope(&mut self) -> &mut Scope {
        self.stack.last_mut().unwrap()
    }*/

    pub fn scope_extend(&mut self) {
        self.stack.last_mut().unwrap().extend();
    }

    pub fn scope_reduce(&mut self) {
        self.stack.last_mut().unwrap().reduce();
    }

    pub fn new_var(&mut self, name: &str, val: Value) -> Signal {
        // does space need to be allocated on heap?
        match val {
            RawList(l) => {
                // store list on heap and generate ref
                let r = 
                self.stack.last_mut().unwrap().new_var(name, Value::List(r))
            },
            v => self.stack.last_mut().unwrap().new_var(name, v)
        }
    }

    pub fn set_var(&mut self, name: &str, val: Value) -> Signal {

    }

    pub fn get_var(&self, name: &str) -> ExprRes {
        self.stack.last_mut().unwrap().get_var(name)
    }

    pub fn set_index_var(&mut self, r: Ref, i: u64, val: Value) -> Signal {
        // use ref to access heap var
        // index with i and set
    }

    pub fn get_index_var(&mut self, r: Ref, i: u64) -> ExprRes {
        // use ref to access heap var
        // index with i and get
    }
}
