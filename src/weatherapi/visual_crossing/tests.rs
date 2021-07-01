#[cfg(test)]
#[allow(unused)]
mod test {
    use crate::weatherapi::{api::Forecast, parser::Parser, visual_crossing::parser::VCParser};
    use chrono::NaiveDateTime;
    use serde_json::Value;

    fn convert_test_case(v: &Vec<Forecast>) -> String {
        let data = v
            .iter()
            .map(|f| {
                format!(
                    r#"{{
                        "datetime":"{}",
                        "temp": {}
                    }}"#,
                    f.date.to_string(),
                    f.temperature
                )
            })
            .collect::<Vec<String>>()
            .join(",");
        let data = format!(r#"{{"days":[{}]}}"#, data);
        return data;
    }

    fn make_test(case: Vec<Forecast>) {
        let data = convert_test_case(&case);
        let v: serde_json::Value = serde_json::from_str(&data).unwrap();
        let parser = VCParser::new();
        let r = parser.parse(v);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), case);
    }

    #[test]
    fn test_zero() {
        make_test(vec![]);
    }

    #[test]
    fn test_one() {
        make_test(vec![Forecast::new(
            21.5,
            chrono::NaiveDate::from_ymd(2021, 07, 01),
        )]);
    }

    #[test]
    fn test_many() {
        make_test(vec![
            Forecast::new(20.02, chrono::NaiveDate::from_ymd(2021, 07, 01)),
            Forecast::new(21.5, chrono::NaiveDate::from_ymd(2021, 07, 02)),
            Forecast::new(30.1, chrono::NaiveDate::from_ymd(2021, 07, 03)),
        ]);
    }
}
