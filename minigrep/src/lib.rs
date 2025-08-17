pub fn case_sensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn case_insensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive_test() {
        let quety = "Akash";
        let content = "Hi, I'm Akash.\nHow are you?";

        assert_eq!(
            vec!["Hi, I'm Akash."],
            case_sensitive_search(quety, content)
        )
    }

    #[test]
    fn case_insensitive_test() {
        let query = "aKaSh";
        let content = "Hi, I'm Akash.\nHow are you?";

        assert_eq!(
            vec!["Hi, I'm Akash."],
            case_insensitive_search(query, content)
        )
    }
}
