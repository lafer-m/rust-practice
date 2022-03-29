

pub fn search<'a>(query: &str,contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}