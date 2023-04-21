use std::ops::Deref;
use crate::cell::Cell;
use std::ptr::NonNull;
struct RcInner<T> {
    value: T,
    refCount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
}

impl<T> Rc<T> {
    fn new(v: T) -> Self {
        let value = RcInner { value: v, refCount: Cell::new(1) };
        let inner = Box::new(value); //allocate to heap, attain ownership
        Self {
            //SAFETY :  box does not give us a null pointer
            inner: unsafe{NonNull::new_unchecked(Box::into_raw(inner))}
        }
    }
}

// impl<T> Clone for Rc<T> {
//     fn clone(&self) -> Self {
//         let inner = unsafe { &*self.inner };
//         let c = inner.refCount.get();
//         inner.refCount.set(c+1);
//         Rc { inner: self.inner }
//     }
// }

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        //SAFETY:
        //self.inner is a Box that is only deallocated when the last rc goes away.
        //we have a rc, therefore the box is not deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
     fn drop(&mut self) {
        let inner = unsafe{self.inner.as_ref()};
        let c = inner.refCount.get();
        if c==1{
            //SAFETY: we are the only rc, and we are being dropped.
            //therefore after us, there will be no Rcs, and no references to T
            drop(inner);
            let _ = unsafe{Box::from_raw(self.inner.as_ptr())};
        }else{
            //there are other Rcs. dont drop the box!
            inner.refCount.set(c-1);
        }
    }
}