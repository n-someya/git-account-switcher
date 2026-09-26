use git_account_switcher::ui::Reporter;

#[test]
fn test_reporter_formatting_in_memory() {
    let mut reporter = Reporter::buffer();

    reporter.success("Account configured").unwrap();
    reporter.warn("Key missing").unwrap();
    reporter.error("Syntax error").unwrap();
    reporter.section("Summary").unwrap();
    reporter.item("Includes file created").unwrap();

    let output = reporter.into_string();
    assert!(output.contains("Account configured"));
    assert!(output.contains("Key missing"));
    assert!(output.contains("Syntax error"));
    assert!(output.contains("Summary"));
    assert!(output.contains("Includes file created"));
}

#[test]
fn test_reporter_color_suppression_with_buffer() {
    // A buffer is not a tty, anstream/anstyle writes standard text cleanly
    let mut reporter = Reporter::buffer();
    reporter.success("Clean message").unwrap();
    let output = reporter.into_string();
    assert!(output.contains("Clean message"));
}
