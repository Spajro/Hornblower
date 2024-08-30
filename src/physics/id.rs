pub type ID=u32;

pub struct IdFactory{
    next:ID
}

impl IdFactory{

    pub fn new()->Self{
        IdFactory{
            next: 0,
        }
    }
    pub fn next(&mut self)->ID{
        self.next+=1;
        self.next
    }
}