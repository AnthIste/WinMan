extern crate regex;

use std::vec::Vec;

use regex::{Regex, RegexBuilder};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum FuzzyResult {
    ExactMatch,
    StartsWith,
    SmartCamel,
    UpperCamel,
    Contains,
    Vague,
    None,
}

fn fuzzy_match(query: &str, input: &str) -> FuzzyResult {
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
    use super::{fuzzy_match, FuzzyResult};

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
    fn partialord() {
        assert!(FuzzyResult::ExactMatch < FuzzyResult::StartsWith);
        assert!(FuzzyResult::ExactMatch <= FuzzyResult::StartsWith);
        assert!(FuzzyResult::StartsWith > FuzzyResult::ExactMatch);
        assert!(FuzzyResult::StartsWith >= FuzzyResult::ExactMatch);
    }

}
