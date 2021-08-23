pub fn without_braces(source: &str)->&str{
    // удалить сначала кавычку и потом так же 
    // удалить кавычку 
    let mut chars = source.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn split_to_lines(content: &str, something: &str)->Vec<String>{
    return content.split(something)
        .map(|s| s.to_string())
        .collect();
}