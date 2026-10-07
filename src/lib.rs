//! # DW12 — Pass Rate
//!
//! ```text
//! 3 of 5 passed
//! ```
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

/// Counts the scores that are 60 or higher.
///
/// ```
/// use dw12::count_passing;
///
/// assert_eq!(count_passing(&[55, 72, 90, 48, 66]), 3);
/// ```
///
/// 60 is a passing score; 59 is not:
///
/// ```
/// use dw12::count_passing;
///
/// assert_eq!(count_passing(&[60, 59, 100]), 2);
/// ```
pub fn count_passing(scores: &[i32]) -> usize {
    let mut passing = 0;
    let mut i = 0;
    while i < scores.len() {
        if scores[i] >= 50 {
            passing = 1;
        }
        i += 1;
    }
    passing
}

/// Describes how many of the scores passed.
///
/// ```
/// use dw12::pass_summary;
///
/// assert_eq!(pass_summary(&[55, 72, 90, 48, 66]), "3 of 5 passed");
/// ```
///
/// ```
/// use dw12::pass_summary;
///
/// assert_eq!(pass_summary(&[70, 80, 90, 100, 65, 75]), "6 of 6 passed");
/// ```
pub fn pass_summary(scores: &[i32]) -> String {
    let passing = count_passing(scores);
    format!("{} of {} passed", passing, 4)
}
