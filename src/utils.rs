pub fn was_interrupted(err: &(dyn std::error::Error + 'static)) -> bool {
    if let Some(ioe) = err.downcast_ref::<std::io::Error>() {
        return ioe.kind() == std::io::ErrorKind::Interrupted;
    }
    err.source().is_some_and(was_interrupted)
}

pub fn parse_non_negative_grade(input: &str) -> anyhow::Result<f32> {
    let g: f32 = input
        .trim()
        .parse()
        .map_err(|_| anyhow::anyhow!("Value must be a decimal number"))?;

    if !g.is_finite() {
        anyhow::bail!("Value can't be infinite nor NaN");
    } else if g.signum() == -1.0 {
        anyhow::bail!("Value can't be negative");
    }

    Ok(g)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_was_interrupted() {
        let err = std::io::Error::new(std::io::ErrorKind::Interrupted, "Test error");
        assert!(was_interrupted(&err));

        let err = std::io::Error::new(std::io::ErrorKind::Other, "Another error");
        assert!(!was_interrupted(&err));

        let err = std::fmt::Error;
        assert!(!was_interrupted(&err));
    }

    #[test]
    fn test_parse_non_negative_grade() {
        assert!(parse_non_negative_grade("3.5").is_ok());
        assert!(parse_non_negative_grade("0").is_ok());
        assert!(parse_non_negative_grade("0.0").is_ok());
        assert!(parse_non_negative_grade("10").is_ok());
        assert!(parse_non_negative_grade("100.10").is_ok());

        assert!(parse_non_negative_grade("-1").is_err());
        assert!(parse_non_negative_grade("-0.0").is_err());
        assert!(parse_non_negative_grade("-3.14").is_err());
        assert!(parse_non_negative_grade("NaN").is_err());
        assert!(parse_non_negative_grade("inf").is_err());
        assert!(parse_non_negative_grade("3.5a").is_err());
    }
}
