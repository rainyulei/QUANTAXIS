
use evalexpr::*;
use crate::qadatastruct::stockday::QADataStruct_StockDay;


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut testds = QADataStruct_StockDay::new_from_vec(
            vec!["2021-01-01".to_string(), "2021-01-02".to_string()],
            vec!["000001.XSHE".to_string(), "000001.XSHE".to_string()],
            vec![20.1, 20.2],
            vec![22.1, 21.1],
            vec![19.2, 19.8],
            vec![21.0, 20.4],
            vec![22.0, 23.0],
            vec![19.0, 19.5],
            vec![99.2, 99.2],
            vec![880.2, 990.2],
            vec![8880.2, 8890.2],
        );

        let close = testds["close"];

        use evalexpr::*;

        let context = context_map! {
            "close": close
        "five" => 5,
        "twelve" => 12,
        "f" => Function::new(|argument| {
            if let Ok(int) = argument.as_int() {
                Ok(Value::Int(int / 2))
            } else if let Ok(float) = argument.as_float() {
                Ok(Value::Float(float / 2.0))
            } else {
                Err(EvalexprError::expected_number(argument.clone()))
            }
        }),
        "avg" => Function::new(|argument| {
            let arguments = argument.as_tuple()?;

            if let (Value::Int(a), Value::Int(b)) = (&arguments[0], &arguments[1]) {
                Ok(Value::Int((a + b) / 2))
            } else {
                Ok(Value::Float((arguments[0].as_number()? + arguments[1].as_number()?) / 2.0))
            }
        })
        }
        .unwrap(); // Do proper error handling here

        assert_eq!(
            eval_with_context("five + 8 > f(twelve)", &context),
            Ok(Value::from(true))
        );
        // `eval_with_context` returns a variant of the `Value` enum,
        // while `eval_[type]_with_context` returns the respective type directly.
        // Both can be used interchangeably.
        assert_eq!(
            eval_boolean_with_context("five + 8 > f(twelve)", &context),
            Ok(true)
        );
        assert_eq!(
            eval_with_context("avg(2, 4) == 3", &context),
            Ok(Value::from(true))
        );
    }
}
