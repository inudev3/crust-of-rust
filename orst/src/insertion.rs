use super::*;
impl Sorter for InsertionSort{
    fn sort<T>(slice: &mut [T]) where T: ord {
        for unsorted in 1..slice.len(){
            let mut i = unsorted;
            while i>0 && slice[i-1]>slice[i]{
                slice.swap(i-1,i);
                i -=1;
            }
        }
    }
}