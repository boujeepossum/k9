use crate::string_diff::colored_diff;
use colored::*;

pub trait FormattableForComparison {
    fn format(&self) -> String;
}

impl<T> FormattableForComparison for T
where
    T: std::fmt::Debug,
{
    fn format(&self) -> String {
        format!("{self:#?}")
    }
}

pub fn assert_equal<
    T1: FormattableForComparison + PartialEq,
    T2: FormattableForComparison + PartialEq,
>(
    left: T1,
    right: T2,
    fail: bool,
) -> Option<String> {
    if fail {
        let diff_string = colored_diff(&left.format(), &right.format())
            .unwrap_or_else(|| "no visual difference between values".to_string());

        let message = format!(
            "
Expected `{left_desc}` to equal `{right_desc}`:
{diff_string}",
            left_desc = "Left".red(),
            right_desc = "Right".green(),
            diff_string = &diff_string
        );

        Some(message)
    } else {
        None
    }
}
