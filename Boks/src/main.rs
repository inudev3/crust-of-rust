use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;


pub struct Boks<T>{
    p: NonNull<T>,
    _t : PhantomData<T>
}

impl<T> Boks<T>{
    pub fn ny(t:T)->Self{
        Boks{
            //SAFETY: Box never creates a null pointer.
            p:unsafe{NonNull::new_unchecked(Box::into_raw(Box::new(t)))},
            _t: PhantomData::default()
        }
    }
}
#[feature(dropck_eyepatch)]
impl < T> Drop for Boks<T>{
    fn drop(&mut self) {
        //SAFETY : p was constructed from a Box in the first place, and has not been used since.
        //otherwise drop could not be called.
        // let _ = unsafe {std::ptr::read(self.p as *const u8)};
        unsafe { Box::from_raw(self.p.as_mut())};
        //creates a new Box, and when it goes out of scope the Box and pointer is deallocated
    }
}

impl<T> Deref for Boks<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        //SAFETY is valid since it was constructed from a valid T(concrete type T), and turned into a pointer
        //through Box in the Boks::new function, which creates alligned pointer, and hasnt been freed since self is alive
        unsafe { &*self.p.as_ref()}
    }
}
impl<T> DerefMut for Boks<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        //SAFETY is valid since it was constructed from a valid T(concrete type T), and turned into a pointer
        //through Box in the Boks::new function, which creates alligned pointer, and hasnt been freed since self is alive
        //Also, since we have mut self, no other mutable reference has been given out to p.
        unsafe { &mut *self.p.as_mut()}
    }
}
fn main() {
    let x = 42;
    let b = Boks::ny(x);
    println!("{:?}", *b);
    let mut y = 42;

    let b = Boks::ny( &mut y); //b is now a raw pointer to a mut ref
    println!("{:?}", y);
    drop(b);
}
