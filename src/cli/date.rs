use std::ops::Add;

pub fn parse(date_str: &str) -> Result<chrono::NaiveDate, anyhow::Error> {
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Ok(date);
    }

    let today = chrono::Local::now().date_naive();
    if date_str.eq("today") {
        return Ok(today);
    }

    match date_str {
        "today" => return Ok(today),
        "yesterday" => return Ok(today.add(chrono::Duration::days(-1))),
        "tomorrow" => return Ok(today.add(chrono::Duration::days(1))),
        _ => {}
    }

    return Err(anyhow::anyhow!("Invalid date: {}", date_str));
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn hardcoded_str() -> Result<(), anyhow::Error> {
        let today = chrono::Local::now().date_naive();
        assert_eq!(today, parse("today").unwrap());
        assert_eq!(
            today.add(chrono::Duration::days(-1)),
            parse("yesterday").unwrap()
        );
        assert_eq!(
            today.add(chrono::Duration::days(1)),
            parse("tomorrow").unwrap()
        );

        Ok(())
    }

    #[test]
    fn rfc_date() -> Result<(), anyhow::Error> {
        assert_eq!(
            chrono::NaiveDate::from_ymd_opt(2023, 10, 20).unwrap(),
            parse("2023-10-20").unwrap(),
        );

        Ok(())
    }

    #[test]
    fn invalid_date() {
        parse("abcbc").expect_err("invalid date");
    }
}
