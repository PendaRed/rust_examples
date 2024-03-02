use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct JpgString {
    payload:String
}

impl JpgString {
    pub fn new(payload:String)->Self {
        JpgString{payload}
    }

    fn matches(p:&Vec<char>, idx:usize, o:&Vec<char>)->bool {
        if p.len()<idx+o.len() {return false};
        for i in 0..o.len() {
            if p[idx+i]!=o[i] {return false};
        }
        true
    }

    pub fn replace(&self, old_chars: &str, new_chars: &str) -> Cow<Self> {
        if self.payload.contains(old_chars) {
            let p: Vec<char> = self.payload.chars().collect();
            let o: Vec<char> = old_chars.chars().collect();

            let mut output = String::new();
            let mut fwd = 0;
            for i in 0..p.len() {
                if i + fwd >= p.len() {
                    break;
                }
                let c = p[i + fwd];
                if Self::matches(&p, i + fwd, &o) {
                    output.push_str(new_chars);
                    fwd += old_chars.len() - 1;
                } else {
                    output.push(c);
                }
            }
            Cow::Owned(JpgString::new( output))
        } else {
            Cow::Borrowed(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_test() {
        assert_eq!(true,JpgString::matches(&{"aaaJPGvvv".chars().collect()},
                           3, &{"JPG".chars().collect()}));
        assert_eq!(false,JpgString::matches(&{"aaaJPGvvv".chars().collect()},
                                           2, &{"JPG".chars().collect()}));
        assert_eq!(false,JpgString::matches(&{"".chars().collect()},
                                            2, &{"JPG".chars().collect()}));
    }

    #[test]
    fn replace_test() {
        assert_eq!(String::from("aaa___vvv"),
                   JpgString::new(String::from("aaaJPGvvv"))
                       .replace(&"JPG",&"___")
                       .payload);
        assert_eq!(String::from("I\nAm\nA\nCrappy\nMsoft.\n"),
                   JpgString::new(String::from("I\r\nAm\r\nA\r\nCrappy\r\nMsoft.\r\n"))
                       .replace(&"\r\n",&"\n")
                       .payload);
    }

}