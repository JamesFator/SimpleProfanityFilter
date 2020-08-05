extern crate simple_profanity_filter;
use simple_profanity_filter::ProfanityFilter;

#[test]
fn test_single_words() {
    let filter = ProfanityFilter::new();
    assert_eq!(filter.filter("Hello"), "Hello");
    assert_eq!(filter.filter("world!"), "world!");
    assert_eq!(filter.filter("Cunk"), "****");
    assert_eq!(filter.filter("cunk"), "****");
}

#[test]
fn test_sentences() {
    let filter = ProfanityFilter::new();
    assert_eq!(filter.filter("Hello world!"), "Hello world!");
    assert_eq!(filter.filter("You're such a cunk"), "You're such a ****");
}

#[test]
fn test_known_issues() {
    // In theory, we should test failure cases, amirite?
    let filter = ProfanityFilter::new();

    // Appending punctuation
    assert_eq!(filter.filter("cunk!"), "cunk!");

    // Merging two words which aren't in the blocklist
    assert_eq!(filter.filter("cunkcunk"), "cunkcunk");
}
