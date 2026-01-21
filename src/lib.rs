use regex::Regex;

pub enum TagStrategy {
    Preserve,
    Remove,
}

pub fn extract_asn1_blocks(content: &str) -> String {
    let re = Regex::new(r"(?m)^-- ASN1START([\s\S]*?)^-- ASN1STOP").unwrap();
    let mut result = Vec::new();
    for cap in re.captures_iter(content) {
        result.push(cap.get(1).map_or("", |m| m.as_str()));
    }

    if result.is_empty() {
        let fallback_re = Regex::new(r"(?m)^-- \*+[\s\S]*?^END").unwrap();
        for mat in fallback_re.find_iter(content) {
            result.push(mat.as_str());
        }
    }

    result.join("")
}

pub fn remove_trailing_comments(content: &str, tag_strategy: TagStrategy) -> String {
    let re = Regex::new(r"(?m)--.*?$").unwrap();
    re.replace_all(content, |caps: &regex::Captures| match tag_strategy {
        TagStrategy::Preserve => {
            if caps[0].to_lowercase().contains("need") || caps[0].to_lowercase().contains("cond") {
                caps[0].to_string()
            } else {
                "".to_string()
            }
        }
        TagStrategy::Remove => "".to_string(),
    })
    .to_string()
}

pub fn remove_delimited_comments(content: &str) -> String {
    let re = Regex::new(r"(--.*?--)(.*?\S+)").unwrap();
    re.replace_all(content, |caps: &regex::Captures| format!("{}", &caps[2]))
        .to_string()
}

pub fn remove_multiline_comments(content: &str) -> String {
    let re = Regex::new(r"/\*[\s\S]*?\*/").unwrap();
    re.replace_all(content, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_asn1_blocks() {
        let input = r#"
Beginning of the file

-- ASN1START

The first ASN.1 content

-- ASN1STOP

Middle of the file

-- ASN1START

The second ASN.1 content

-- ASN1STOP

End of the file
"#;
        let expected = r#"

The first ASN.1 content



The second ASN.1 content

"#;
        assert_eq!(extract_asn1_blocks(input), expected);
    }

    #[test]
    fn test_no_blocks() {
        let input = "No blocks here";
        assert_eq!(extract_asn1_blocks(input), "");
    }

    #[test]
    fn test_one_block() {
        let input = r#"
-- ASN1START
one block
-- ASN1STOP
"#;
        let expected = r#"
one block
"#;
        assert_eq!(extract_asn1_blocks(input), expected);
    }

    #[test]
    fn test_fallback_extraction() {
        let input = r#"
Some text
-- *****
My ASN.1 Content
END
More text
"#;
        let expected = r#"-- *****
My ASN.1 Content
END"#;
        assert_eq!(extract_asn1_blocks(input).trim(), expected);
    }

    #[test]
    fn test_remove_comments() {
        let input = r#"
/* This is
a multiline
comment */

qwer -- this is a delimited comment -- asdf

zxcv -- this is a trailing comment

wert -- this is a false positive need code

sdfg -- this is a false positive condition
"#;

        let multiline_removed = remove_multiline_comments(input);
        let multiline_expected = r#"


qwer -- this is a delimited comment -- asdf

zxcv -- this is a trailing comment

wert -- this is a false positive need code

sdfg -- this is a false positive condition
"#;
        assert_eq!(multiline_removed, multiline_expected);

        let delimited_removed = remove_delimited_comments(&multiline_removed);
        let trailing_removed = remove_trailing_comments(&delimited_removed, TagStrategy::Remove);
        let trailing_expected = r#"


qwer  asdf

zxcv 

wert 

sdfg 
"#;
        assert_eq!(trailing_removed, trailing_expected);

        let tag_preserved = remove_trailing_comments(&delimited_removed, TagStrategy::Preserve);
        let tag_expected = r#"


qwer  asdf

zxcv 

wert -- this is a false positive need code

sdfg -- this is a false positive condition
"#;
        assert_eq!(tag_preserved, tag_expected);
    }
}
