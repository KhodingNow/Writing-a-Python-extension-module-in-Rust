/// Add together two i32 numbers and return the result of that addition !
///``` 
/// assert_eq!(testing::add(2, 2), 4);
///```
///
///```
/// use testing::add;
/// assert_eq!(add(2,2), 4);
///```
///
///```
/// use testing::add;
/// assert_eq!(add("hello", 2), 5);
///```


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
