use std::cell::UnsafeCell;
pub struct Cell<T>{
    value: UnsafeCell<T>,
}
//implied by UnsafeCell
//impl<T> !Sync for Cell<T>{}
impl<T> Cell<T>{
    pub fn new(value:T)->Cell<T>{
        Cell{value:UnsafeCell::new(value)}
    }
    pub fn set(&self, value:T){
        unsafe { *self.value.get() = value};
    }
    pub fn get(&self)->T where T:Copy{
        unsafe { *self.value.get()}
    }
}
#[cfg(test)]
mod test{
    use super::Cell;
    // #[test]
    // fn bad(){
    //     use std::sync::Arc;
    //     let x = Arc::new(Cell::new(42));
    //     let x1 = x.clone();
    //     std::thread::spawn(||{x1.set(43)});
    //     let x2 = x.clone();
    //     std::thread::spawn(||{x2.set(44)});
    // }
    // #[test]
    // fn bad2(){
    //     let x  = Cell::new(String::from("hi"));
    //     let first = x.get();
    //     x.set(String::from(""));
    //     x.set(String::from("hey"));
    //     eprintln!("{}",first)
    // }
}