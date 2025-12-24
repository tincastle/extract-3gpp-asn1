use regex::Regex;

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
}