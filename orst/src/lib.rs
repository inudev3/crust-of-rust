trait Sorter{
    fn sort<T>(slice: &mut [T])
    where
        T:ord;
}
fn sort<T,S>(slice:&mut [T])
where
    T:Ord,
    S:Sorter
{
    S::sort(slice)
}
mod bubblesort;
mod insertion;

#[cfg(test)]
mod tests{
    use crate::*;

    struct StdSorter;
    impl Sorter for StdSorter{
        fn sort<T>(slice: &mut [T]) where T: ord {
            slice.sort()
        }
    }
}
