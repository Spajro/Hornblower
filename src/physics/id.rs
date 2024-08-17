pub struct IdFactory{
    next:u32
}

impl IdFactory{

    pub fn new()->Self{
        IdFactory{
            next: 0,
        }
    }
    pub fn next(&mut self)->u32{
        self.next+=1;
        self.next
    }
}