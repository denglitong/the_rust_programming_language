#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        /// string literal, hardcoded directly into the final executable
        /// string literal type is &str, a slice pointing to that specific point of the binary,
        /// and this is why string literals are immutable
        let s = "hello";
        println!("{}", s);

        let mut s = "hello";
        s = "world";
        println!("{}", s);

        /// String allocate an amount of memory on the heap, unknown at compile time:
        /// The memory must be requested from the operating system at runtime(when we call String::from),
        /// We need a way of returning this memory to the operating system when we're done with our String
        let mut s = String::from("hello");
        s.push_str(", world");
        println!("{}", s);

        {
            let s = String::from("hello");
        }

        let s1 = gives_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        println!("{} {}", s1, s3);

        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("{}", len);
    }

    /// The Rules of References:
    /// + At any given time, you can have either one mutable reference or any number of immutable references
    /// + References must always be valid
    /// the references takes no ownership, and it's called borrowing, another data type that does not
    /// take ownership is the slice, which allow you reference a contiguous sequence of elements in
    /// a collection rather than the whole collection.
    /// String slice range indies must occur at valid UTF-8 character boundaries.
    /// If you attempt to create a string slice in the middle of a multibyte character,
    /// your program will exit with an error, such as raw byte array slice &[u8] to string
    fn first_word_slice(s: &str) -> &str {
        for (i, &item) in s.as_bytes().iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        some_string
    }
    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }
}
