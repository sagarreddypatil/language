pub fn preprocess(source: String) -> String {
    // remove comments
    let mut ret = String::new();

    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '/' {
            if let Some('/') = chars.peek() {
                chars.next();
                while let Some(c) = chars.next() {
                    if c == '\n' {
                        ret.push('\n');
                        break;
                    }
                }
            } else {
                ret.push(c);
            }
        } else {
            ret.push(c);
        }
    }

    ret
}