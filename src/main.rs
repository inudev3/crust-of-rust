mod flatten;

pub struct StrSplit<'a,'b> {
    remainder: Option<&'a str>,
    delimiter: &'b str,
}


impl<'a,'b> StrSplit<'a,'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str) -> Self {
        Self {
            remainder: Option::from(haystack),
            delimiter,
        }
    }
}

impl<'a,'b> Iterator for StrSplit<'a,'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delim = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delim)
        } else {
            self.remainder.take()
        };
        None
    }
}

fn until_char(s:&str, c:char)-> & str{
    let delim = format!("{}",c);
    StrSplit::new(s,&delim).next().expect("expect at least one result")
}
// #[test]
fn until_char_test(){
    assert_eq!(until_char("hello_world", 'o'), "hell");

}

fn main() {

}