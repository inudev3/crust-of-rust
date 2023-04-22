pub fn strtok<'a, 'b>(s: &'a mut &'b str, delim:char) ->  &'b str {
    if let Some(i) = s.find(delim){
        let prefix = &s[..i];
        let suffix  = &s[(i+1)..];
        *s = suffix;
        return prefix
    }else{
        let prefix = *s;
        *s = "";
        prefix
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn check_is_static(_:&'static str){}
        let mut x = "hello world";
        //strtok <'a,'b> (&'a mut &'b str)-> &'b str
        let z = &mut x;
        //여기서 z의 라이프타임이 끝남. 라이프타임은 공변적이기 때문.
        let hello = strtok(&mut x, ' ');
        assert_eq!(hello, "hello");
        assert_eq!(x, "world");
    }
}
