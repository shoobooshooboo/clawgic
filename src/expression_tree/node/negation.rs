///Struct representing the number of tildes attached to something.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Negation{
    count: u32,
}

impl Negation{
    pub fn new(count: u32) -> Self{
        Self{count}
    }

    ///If count > 0, decrement. otherwise, increment.
    pub fn deny(&mut self){
        if self.count > 0{
            self.count -= 1;
        }else{
            self.count += 1;
        }
    }

    ///If count > 1, decrement. otherwise, increment.
    pub fn double_deny(&mut self){
        if self.count > 1{
            self.count -= 2;
        }else{
            self.count += 2;
        }
    }

    ///Increments count.
    pub fn negate(&mut self){
        self.count += 1;
    }

    ///Adds 2 to count.
    pub fn double_negate(&mut self){
        self.count += 2;
    }

    ///Reduces count to either 0 or 1 while retaining tval
    pub fn reduce(&mut self){
        self.count &= 1
    }

    ///If count is even, return false. otherwise true.
    pub fn is_denied(&self) -> bool{
        self.count & 1 == 1
    }

    ///Truth value. If count is even, return true. otherwise false.
    pub fn tval(&self) -> bool{
        self.count & 1 != 1
    }

    ///Returns the count.
    pub fn count(&self) -> u32{
        self.count
    }
}

impl Default for Negation{
    fn default() -> Self {
        Self { count: 0 }
    }
}