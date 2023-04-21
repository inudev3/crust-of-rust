use std::collections::VecDeque;
use std::io::BufRead;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T>{
    shared: Arc<Shared<T>>
}
pub struct Receiver<T>{
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>
}
struct Shared<T>{
    inner: Mutex<Inner<T>>,
    available:Condvar
}

struct Inner<T>{
    queue: VecDeque<T>,
    senders: usize,
}
impl<T> Clone for Sender<T>{
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders+=1;
        drop(inner);
        Sender{
            shared: Arc::clone(&self.shared)
        }
    }
}
impl<T> Drop for Sender<T>{
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders-=1;
        let was_last = inner.senders==0;
        //wake up if the receiver is still blocking, when there is no sender
        drop(inner);
        if was_last{
            self.shared.available.notify_one();
        }
    }
}
impl<T> Sender<T>{
    pub fn send(&mut self, t:T){
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner); //drop the lock - MutexGuard를 drop
        self.shared.available.notify_one()
    }
}
impl <T> Receiver<T>{
    pub fn receive(&mut self)-> Option<T>{
        if let Some(t) = self.buffer.pop_front(){
            return Some(t)
        } //buffer is not empty, just return
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    std::mem::swap(&mut self.buffer, &mut inner.queue); //buffer is empty here, so queue is being emptied
                    return Some(t)
                }
                None if inner.senders==0 =>return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }

        }
    }
}
pub fn channel<T>()->(Sender<T>, Receiver<T>){
    let inner = Inner{
        queue: VecDeque::new(),
        senders:1,
    };
    let shared = Shared{
        inner:Mutex::new(inner),
        available: Condvar::new()
    };
    let shared  = Arc::new(shared);
    (Sender{shared:shared.clone() }, Receiver{shared:shared.clone()})
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn ping_pong(){
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.receive(),Some(42));
    }

    #[test]
    fn closed(){
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.receive(), None);
    }
}