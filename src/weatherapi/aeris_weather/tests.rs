#[cfg(test)]
#[allow(unused)]
mod test {
    // TODO: use templates instead of repeating format!()

    use crate::weatherapi::{
        aeris_weather::parser::AWParser,
        api::Forecast,
        parser::{Parser, ParserError},
        visual_crossing::parser::VCParser,
    };
    use chrono::{NaiveDateTime, NaiveTime};
    use serde_json::Value;

    fn convert_test_case(v: &Vec<Forecast>) -> String {
        let data = v
            .iter()
            .map(|f| {
                format!(
                    r#"{{
                            "timestamp": {},
                            "maxTempC": {},
                            "minTempC": {}
                        }}"#,
                    chrono::NaiveDateTime::new(
                        f.date,
                        NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap()
                    )
                    .timestamp(),
                    f.temperature,
                    f.temperature,
                )
            })
            .collect::<Vec<String>>()
            .join(",");
        let data = format!(
            r#"{{
                "success":true,
                "error": null,
                "response": [{{
                    "periods": [{}]
                }}]
            }}"#,
            data
        );
        return data;
    }
    #[test]
    fn test_no_temp_c() {
        let ts = 1625313600i64;
        let min_t = 24;
        let max_t = 34;
        let temp = (min_t + max_t) as f64 / 2.0;
        let data = format!(
            r#"{{
            "success": true,
            "error": null,
            "response": [
                {{
                    "periods": [
                        {{
                            "timestamp": {},
                            "tempC": null,
                            "maxTempC": {},
                            "minTempC": {}
                        }}
                    ]
                }}
            ]
        }}
        "#,
            ts, max_t, min_t
        );
        let v: serde_json::Value = serde_json::from_str(&data).unwrap();
        let parser = AWParser::new();
        let r = parser.parse(v);
        assert!(r.is_ok());
        assert_eq!(
            *r.unwrap().first().expect("empty vec"),
            Forecast::new(temp, chrono::NaiveDate::from_ymd(2021, 07, 03))
        );
    }

    #[test]
    fn test_error() {
        let msg = r#"{"something":"wrong"}"#.to_string();
        let data = format!(
            r#"{{
            "success": false,
            "error": {}
        }}"#,
            msg
        );
        let v: serde_json::Value = serde_json::from_str(&data).unwrap();
        let parser = AWParser::new();
        let r = parser.parse(v);
        assert!(r.is_err());
        assert_eq!(r.err().unwrap(), ParserError::Other(msg));
    }

    #[test]
    fn test_ok() {
        let case = vec![
            Forecast::new(20.1, chrono::NaiveDate::from_ymd(2021, 07, 01)),
            Forecast::new(21.2, chrono::NaiveDate::from_ymd(2021, 07, 02)),
        ];
        let data = convert_test_case(&case);
        let v: serde_json::Value = serde_json::from_str(&data).unwrap();
        let parser = AWParser::new();
        let r = parser.parse(v);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), case);
    }
}
