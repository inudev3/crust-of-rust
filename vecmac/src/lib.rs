#[macro_export]
macro_rules! avec{
    ($elem:expr;$count:expr)=>{{
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        vs.extend(std::iter::repeat($element).take(count));
        vs
    }};
    ($($elem:expr),+$(,)?)=>{{
        let count = [$($elem),*].len()
        let mut vs = Vec::with_capacity(count);
        $(vs.push($elem);)+
        vs
    }};

    (@COUNT, $($element:expr),*)=>{
        <[]>::len(&[$($crate::avec![@SUBST, $element]),*])
    }
    (@SUBST, $_element:expr)=>{()}
}
#[test]
fn empty_vec(){
    let my_vec:Vec<u32> = avec![];
    assert!(my_vec.is_empty());
}
#[test]
fn single(){

    let y = Some(42);
    let x  :Vec<u32> = avec![y.take().unwrap();2];
    assert!(!x.is_empty());
    assert_eq!(x[0],42);
}
#[test]
fn double(){
    let x  :Vec<u32> = avec![42;];
    assert!(!x.is_empty());
    assert_eq!(x[0],42);
    assert_eq!(x[1],43);
}
