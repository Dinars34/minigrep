pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines() // make an new iterator that generates new content without spaces
        .filter(|line| line.contains(query)) // filtering basedon line that containing query
        .collect() // turn into collection
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines() // make an new iterator that generates new content without spaces
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase())) // filtering basedon line that containing query
        .collect() // turn into collection
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        // asserting both vec for left and right
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_sensitive(query, contents)
        );
    }
}
