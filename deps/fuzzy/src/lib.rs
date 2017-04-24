extern crate regex;

use std::vec::Vec;

use regex::{Regex, RegexBuilder};

// Order is important: used for priority
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FuzzyResult {
    ExactMatch,
    StartsWith,
    SmartCamel,
    UpperCamel,
    WordBreaks,
    Contains,
    Vague,
    None,
}

pub struct Finder {
    re: Regex,
    re_smart_camel: Option<Regex>,
    re_upper_camel: Regex,
    re_word_breaks: Regex,
    re_vague: Regex,
}

impl Finder {
    pub fn new(query: &str) -> Result<Self, regex::Error> {
        let re = try! {
            RegexBuilder::new(&query)
                .case_insensitive(true)
                .build()
        };

        let re_smart_camel = {
            // Break up the input query into sub-parts separated by UpperCamelCase
            // This only makes sense if there are at least two parts
            let re = Regex::new(r"[A-Z][^A-Z]*").unwrap();
            let matches: Vec<_> = re.captures_iter(&query).collect();

            if matches.len() >= 2 {
                let mut regex_str = String::new();

                for m in matches {
                    let term = m.get(0).unwrap().as_str();

                    // Build a new regex that incorporates all the sub-parts
                    // e.g. (Upper\w*)(Camel\w*)(Case\w*)
                    regex_str.push_str(term);
                    regex_str.push_str(r"\w+");
                }

                let re = try! { Regex::new(&regex_str) };
                Some(re)
            } else {
                None
            }
        };

        let re_upper_camel = {
            let mut regex_str = String::with_capacity(9 * query.len());

            for c in query.to_uppercase().chars() {
                let mut buf = [0; 4]; // A buffer of length four is large enough to encode any char
                let char_slice = c.encode_utf8(&mut buf);

                regex_str.push_str(r"(^|\s+)");
                regex_str.push_str(char_slice);
                regex_str.push_str(r".*(\s+|$).*");
            }

            try! {
                RegexBuilder::new(&regex_str)
                    .case_insensitive(false)
                    .build()
            }
        };

        let re_word_breaks = {
            let mut regex_str = String::with_capacity(9 * query.len());

            for c in query.chars() {
                let mut buf = [0; 4]; // A buffer of length four is large enough to encode any char
                let char_slice = c.encode_utf8(&mut buf);

                regex_str.push_str(r"(^|\s+)");
                regex_str.push_str(char_slice);
                regex_str.push_str(r".+(\s+|$)\**");
            }

            try! {
                RegexBuilder::new(&regex_str)
                    .case_insensitive(true)
                    .build()
            }
        };

        let re_vague = {
            let mut regex_str = String::with_capacity(9 * query.len());

            for c in query.chars() {
                let mut buf = [0; 4]; // A buffer of length four is large enough to encode any char
                let char_slice = c.encode_utf8(&mut buf);

                regex_str.push_str(char_slice);
                regex_str.push_str(r".*");
            }

            try! {
                RegexBuilder::new(&regex_str)
                    .case_insensitive(true)
                    .build()
            }
        };

        Ok(Finder {
            re: re,
            re_smart_camel: re_smart_camel,
            re_upper_camel: re_upper_camel,
            re_word_breaks: re_word_breaks,
            re_vague: re_vague,
        })
    }

    pub fn is_match(&self, s: &str) -> FuzzyResult {
        let m = self.re.find(s);

        // Priority: ExactMatch, StartsWith
        if let Some(m) = m {
            let strlen = s.len();

            match (m.start(), m.end()) {
                (0, end) if end == strlen => return FuzzyResult::ExactMatch,
                // (0, _) => return FuzzyResult::StartsWith,
                _ => {}
            }
        };

        // SmartCamel (if available)
        if let Some(ref re_smart_camel) = self.re_smart_camel {
            if re_smart_camel.is_match(s) {
                return FuzzyResult::SmartCamel;
            }
        }

        // UpperCamel
        if self.re_upper_camel.is_match(s) {
            return FuzzyResult::UpperCamel;
        }

        // Word breaks
        if self.re_word_breaks.is_match(s) {
            return FuzzyResult::WordBreaks;
        }

        // Basic regex
        if m.is_some() {
            return FuzzyResult::Contains;
        }

        // Clutching at straws
        if self.re_vague.is_match(s) {
            return FuzzyResult::Vague;
        }

        FuzzyResult::None
    }
}

pub fn fuzzy_query(terms: &[&str], input: &str) -> FuzzyResult {
    let mut matches: Vec<FuzzyResult> = terms.iter()
        .map(|t| fuzzy_match(t, input))
        .collect();

    matches.sort();

    matches
        .first()
        .map(|m| m.clone())
        .unwrap_or(FuzzyResult::None)
}

pub fn fuzzy_match(query: &str, input: &str) -> FuzzyResult {
    let query = regex::escape(&query);

    // Clean up input
    let input = {
        let re = Regex::new(r"[\s]+").unwrap();
        re.replace_all(input, "")
    };

    // Exact match
    {
        let regex_str = format!(r"^{}$", query);
        let re = RegexBuilder::new(&regex_str)
            .case_insensitive(true)
            .build();

        if let Ok(re) = re {
            if re.is_match(&input) {
                return FuzzyResult::ExactMatch;
            }
        }
    }

    // Starts with
    {
        let regex_str = format!(r"^{}", &query);
        let re = RegexBuilder::new(&regex_str)
            .case_insensitive(true)
            .build();

        if let Ok(re) = re {
            if re.is_match(&input) {
                return FuzzyResult::StartsWith;
            }
        }
    }

    // Smart camel
    {
        let re = Regex::new(r"[A-Z][^A-Z]*").unwrap();
        let captures: Vec<_> = re.captures_iter(&query).collect();

        if captures.len() > 0 {
            let mut regex_str = String::new();
            for capture in captures {
                let term = capture.get(0).unwrap().as_str();
                let part = format!(r"({}\w*)", term);
                regex_str.push_str(&part);
            }

            let re = Regex::new(&regex_str);

            if let Ok(re) = re {
                if re.is_match(&input) {
                    return FuzzyResult::SmartCamel;
                }
            }
        }
    }

    // Upper camel
    {
        let mut regex_str = String::new();
        for c in query.chars() {
            let part = format!(r"({}\w*)", c.to_uppercase());
            regex_str.push_str(&part);
        }

        let re = RegexBuilder::new(&regex_str)
            .case_insensitive(false)
            .build();

        if let Ok(re) = re {
            if re.is_match(&input) {
                return FuzzyResult::UpperCamel;
            }
        }
    }

    // Contains
    {
        let re = RegexBuilder::new(&query)
            .case_insensitive(true)
            .ignore_whitespace(true)
            .build();

        if let Ok(re) = re {
            if re.is_match(&input) {
                return FuzzyResult::Contains;
            }
        }
    }

    // Vague
    {
        let mut regex_str = String::new();
        for c in query.chars() {
            let part = format!(r"({}\w*)", c);
            regex_str.push_str(&part);
        }

        let re = RegexBuilder::new(&regex_str)
            .case_insensitive(true)
            .ignore_whitespace(true)
            .build();

        if let Ok(re) = re {
            if re.is_match(&input) {
                return FuzzyResult::Vague;
            }
        }
    }

    FuzzyResult::None
}

#[cfg(test)]
mod tests {
    use super::{fuzzy_match, fuzzy_query, FuzzyResult};

    #[test]
    fn it_works() {
        use regex::Regex;

        let re = Regex::new(r"([cC]\w*)([gG]\w*)([wW]\w*)").unwrap();

        assert_eq!(false, re.is_match(""));
        assert_eq!(false, re.is_match("hello"));

        assert!(re.is_match("cgw"));
        assert!(re.is_match("CGW"));
        assert!(re.is_match("CustomerGateway"));
        assert!(re.is_match("CustomerGateWay"));
        assert!(re.is_match("CustomerGateWay.cs"));
        assert!(re.is_match("OtherCustomerGateWay"));
        assert!(re.is_match("OCGateway"));
        assert!(re.is_match("customergateway"));
        assert!(re.is_match("customer_gateway"));
        assert!(re.is_match("_customer_gateway_"));

        assert_eq!(false, re.is_match("CG"));
        assert_eq!(false, re.is_match("GW"));
        assert_eq!(false, re.is_match("CW"));
        assert_eq!(false, re.is_match("Gateway"));
        assert_eq!(false, re.is_match("GateWay"));
    }

    #[test]
    fn exact_match() {
        assert_eq!(FuzzyResult::ExactMatch, fuzzy_match("", ""));
        assert_eq!(FuzzyResult::ExactMatch, fuzzy_match("hello", "hello"));
        assert_eq!(FuzzyResult::ExactMatch, fuzzy_match("hello", "Hello"));
    }

    #[test]
    fn starts_with() {
        assert_eq!(FuzzyResult::StartsWith, fuzzy_match("hello", "hello there"));
        assert_eq!(FuzzyResult::StartsWith, fuzzy_match("ins", "Insurance"));
    }

    #[test]
    fn contains() {
        assert_eq!(FuzzyResult::Contains, fuzzy_match("hello", "why hello there"));
        assert_eq!(FuzzyResult::Contains, fuzzy_match("other", "why hello there"));
    }

    #[test]
    fn smart_camel() {
        assert_eq!(FuzzyResult::SmartCamel, fuzzy_match("SuCl", "SuperClass"));
        assert_eq!(FuzzyResult::SmartCamel, fuzzy_match("SCl", "SuperClass"));
        assert_eq!(FuzzyResult::SmartCamel, fuzzy_match("Cl", "SuperClass"));
    }

    #[test]
    fn upper_camel() {
        assert_eq!(FuzzyResult::UpperCamel, fuzzy_match("mc", "MyClass"));
        assert_eq!(FuzzyResult::UpperCamel, fuzzy_match("mc", "MyOtherClass"));
        assert_eq!(FuzzyResult::UpperCamel, fuzzy_match("mc", "OtherMyClass"));
    }

    #[test]
    fn vague() {
        assert_eq!(FuzzyResult::Vague, fuzzy_match("ya", "MyClass"));
        assert_eq!(FuzzyResult::Vague, fuzzy_match("mass", "MyOtherClass"));
        assert_eq!(FuzzyResult::Vague, fuzzy_match("sucla", "SuperClass"));
    }

    #[test]
    fn none() {
        assert_eq!(FuzzyResult::None, fuzzy_match("abc", "cde"));
        assert_eq!(FuzzyResult::None, fuzzy_match("mc", "My"));
    }

    #[test]
    fn query_exact_match() {
        assert_eq!(FuzzyResult::ExactMatch, fuzzy_query(&["hello"], "hello"));
        assert_eq!(FuzzyResult::ExactMatch, fuzzy_query(&["hello", "world"], "hello"));
        assert_eq!(FuzzyResult::ExactMatch, fuzzy_query(&["world", "hello"], "hello"));
        assert_eq!(FuzzyResult::ExactMatch, fuzzy_query(&["MyClass", "MC"], "MyClass"));
        assert_eq!(FuzzyResult::ExactMatch, fuzzy_query(&["MC", "MyClass"], "MyClass"));
    }

    #[test]
    fn partialord() {
        assert!(FuzzyResult::ExactMatch < FuzzyResult::StartsWith);
        assert!(FuzzyResult::ExactMatch <= FuzzyResult::StartsWith);
        assert!(FuzzyResult::StartsWith > FuzzyResult::ExactMatch);
        assert!(FuzzyResult::StartsWith >= FuzzyResult::ExactMatch);
    }

}
