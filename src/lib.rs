pub fn parse_sexpr(source: &str, indent_step: &str) -> Vec<String> {
    let mut depth: i32 = 0;
    let mut start: usize = 0;
    let mut result: Vec<String> = Vec::new();
    let mut in_name: bool;
    let mut last_in_name = false;
    // state machine
    for (i, c) in source.chars().enumerate() {
        // change state
        if c == '(' { depth = depth + 1; in_name = false }
        else if c == ')' || c == ' ' || c == '\n' || c == '\t' || c == '\r'
            { in_name = false }
        else { in_name = true }
        // handle start/end of name
        if in_name && !last_in_name { start = i }
        else if !in_name && last_in_name {
            let num_indent: usize = if depth >= 1 { (depth - 1) as usize } else { 0 };
            let mut s: String = indent_step.repeat(num_indent);
            s.push_str(&source[start..i]);
            result.push(s);
        }
        if c == ')' { depth = depth - 1 }
        // save last state
        last_in_name = in_name;
    }
    if depth != 0 {
        // TODO throw exception
        println!("ERROR parse_sexpr: braces mismatch. found {} extra {}-braces",
            depth.abs(), if depth > 0 { "open" } else { "close" });
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::parse_sexpr;
    fn test_helper(sexpr_str: &str, indent_step: &str, expected_str: &[&str]) {
        println!("input string: {}", sexpr_str);

        let result: Vec<String>;
        result = parse_sexpr(sexpr_str, indent_step);

        let expected: Vec<String> = expected_str.iter().map(|s| s.to_string()).collect();

        println!("actual tree:");
        for l in result.iter() { println!("{}", l) }

        println!("expected tree:");
        for l in expected.iter() { println!("{}", l) }

        for (i, e) in expected.iter().enumerate() {
            assert_eq!(e, &result[i]);
        }
    }

    #[test]
    fn test_main() {
        test_helper(
            "(a (b (c (d))))",
            " ",
            &[
                "a",
                " b",
                "  c",
                "   d",
            ]
        );

        test_helper(
            "(a (b c d))",
            " ",
            &[
                "a",
                " b",
                " c",
                " d",
            ]
        );

        test_helper(
            "(a)",
            " ",
            &[
                "a",
            ]
        );

        test_helper(
            "(((((a)))))",
            " ",
            &[
                "    a",
            ]
        );
    }

}
