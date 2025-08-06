use password_gen::cli::Cli;
use password_gen::generator::generate;

#[test]
fn test_exclude_chars() {
    let config = Cli {
        length: 20,
        digits: true,
        symbols: true,
        uppercase: true,
        no_similar: false,
        exclude: Some("abc123".to_string()),
    };

    let password = generate(&config);
    assert!(!password.contains('a'));
    assert!(!password.contains('1'));
}

#[test]
fn test_no_similar() {
    let config = Cli {
        length: 20,
        digits: true,
        symbols: false,
        uppercase: true,
        no_similar: true,
        exclude: None,
    };

    let password = generate(&config);
    for ch in password.chars() {
        assert!(!"0Oo1Il".contains(ch));
    }
}
