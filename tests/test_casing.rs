#![feature(plugin)]
#![plugin(casing_macros)]

#[test]
pub fn test_lowercase() {
    assert_eq!("ident", to_lower!(stringify!(Ident)));
}

#[test]
pub fn test_uppercase() {
    assert_eq!("IDENT", to_upper!(stringify!(Ident)));
}
