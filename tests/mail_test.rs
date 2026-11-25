use AsyncTTRPG96::mail::{generate_confirmation_code, roll_4d6_drop_lowest};

#[test]
fn test_generate_confirmation_code_format() {
    for _ in 0..50 {
        let code = generate_confirmation_code();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }
}

#[test]
fn test_roll_4d6_drop_lowest_range() {
    for _ in 0..200 {
        let value = roll_4d6_drop_lowest();
        assert!((3..=18).contains(&value));
    }
}
