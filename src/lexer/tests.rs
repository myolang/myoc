use super::*;

use pretty_assertions::assert_eq;

#[test]
fn empty() {
    let actual = lex("");
    let expected = Ok(vec![]);
    assert_eq!(expected, actual);
}

#[test]
fn just_whitespace() {
    let actual = lex("   \n  \t\t \n ");
    let expected = Ok(vec![]);
    assert_eq!(expected, actual);
}

#[test]
fn ind_nat() {
    let src = r#"enum Nat zero succ(pred: Nat) end"#;
    let actual = lex(src);
    let expected = Ok(vec![
        Token::EnumKw(EnumKw {
            level: 0,
            start: ByteIndex(src.find("enum").unwrap()),
            erasable: false,
        }),
        Token::Ident(Ident {
            value: "Nat".to_owned(),
            start: ByteIndex(src.find("Nat").unwrap()),
        }),
        Token::Ident(Ident {
            value: "zero".to_owned(),
            start: ByteIndex(src.find("zero").unwrap()),
        }),
        Token::Ident(Ident {
            value: "succ".to_owned(),
            start: ByteIndex(src.find("succ").unwrap()),
        }),
        Token::LParen(ByteIndex(src.find("(").unwrap())),
        Token::Ident(Ident {
            value: "pred".to_owned(),
            start: ByteIndex(src.find("pred").unwrap()),
        }),
        Token::Colon(ByteIndex(src.find(":").unwrap())),
        Token::Ident(Ident {
            value: "Nat".to_owned(),
            start: ByteIndex(src.find_nth("Nat", 1).unwrap()),
        }),
        Token::RParen(ByteIndex(src.find(")").unwrap())),
        Token::EndKw(ByteIndex(src.find("end").unwrap())),
    ]);
    assert_eq!(expected, actual);
}

#[test]
fn ind_eq() {
    let src = r#"enum* Eq(T: Type, left: T) ^(right: T) refl ^(left) end"#;
    let actual = lex(src);
    let expected = Ok(vec![
        Token::EnumKw(EnumKw {
            level: 0,
            start: ByteIndex(src.find("enum*").unwrap()),
            erasable: true,
        }),
        Token::Ident(Ident {
            value: "Eq".to_owned(),
            start: ByteIndex(src.find("Eq").unwrap()),
        }),
        //
        Token::LParen(ByteIndex(src.find("(").unwrap())),
        Token::Ident(Ident {
            value: "T".to_owned(),
            start: ByteIndex(src.find("T").unwrap()),
        }),
        Token::Colon(ByteIndex(src.find(":").unwrap())),
        Token::UniverseLiteral(UniverseLiteral {
            level: 0,
            start: ByteIndex(src.find("Type").unwrap()),
            erasable: false,
        }),
        Token::Comma(ByteIndex(src.find(",").unwrap())),
        Token::Ident(Ident {
            value: "left".to_owned(),
            start: ByteIndex(src.find("left").unwrap()),
        }),
        Token::Colon(ByteIndex(src.find_nth(":", 1).unwrap())),
        Token::Ident(Ident {
            value: "T".to_owned(),
            start: ByteIndex(src.find("T) ^").unwrap()),
        }),
        Token::RParen(ByteIndex(src.find(")").unwrap())),
        //
        Token::Caret(ByteIndex(src.find("^").unwrap())),
        Token::LParen(ByteIndex(src.find_nth("(", 1).unwrap())),
        Token::Ident(Ident {
            value: "right".to_owned(),
            start: ByteIndex(src.find("right").unwrap()),
        }),
        Token::Colon(ByteIndex(src.find_nth(":", 2).unwrap())),
        Token::Ident(Ident {
            value: "T".to_owned(),
            start: ByteIndex(src.find("T) refl").unwrap()),
        }),
        Token::RParen(ByteIndex(src.find_nth(")", 1).unwrap())),
        //
        Token::Ident(Ident {
            value: "refl".to_owned(),
            start: ByteIndex(src.find("refl").unwrap()),
        }),
        Token::Caret(ByteIndex(src.find_nth("^", 1).unwrap())),
        Token::LParen(ByteIndex(src.find_nth("(", 2).unwrap())),
        Token::Ident(Ident {
            value: "left".to_owned(),
            start: ByteIndex(src.find_nth("left", 1).unwrap()),
        }),
        Token::RParen(ByteIndex(src.find_nth(")", 2).unwrap())),
        Token::EndKw(ByteIndex(src.find("end").unwrap())),
    ]);
    assert_eq!(expected, actual);
}

#[test]
fn dashes_and_thin_arrows() {
    let src = r#"-a-->->-->-"#;
    let actual = lex(src);
    let expected = Ok(vec![
        Token::Dash(ByteIndex(0)),
        Token::Ident(Ident {
            value: "a".to_owned(),
            start: ByteIndex(1),
        }),
        Token::Dash(ByteIndex(2)),
        Token::ThinArrow(ByteIndex(3)),
        Token::ThinArrow(ByteIndex(5)),
        Token::Dash(ByteIndex(7)),
        Token::ThinArrow(ByteIndex(8)),
        Token::Dash(ByteIndex(10)),
    ]);
    assert_eq!(expected, actual);
}

#[test]
fn keywords() {
    let src = r#"_ enum enum1 enum33 enum* enum1* enum33* def match For case use end dec Type Type1 Type33 Type* Type1* Type33*"#;
    let actual = lex(src);
    let expected = Ok(vec![
        Token::Underscore(ByteIndex(src.find("_").unwrap())),
        Token::EnumKw(EnumKw {
            level: 0,
            start: ByteIndex(src.find("enum").unwrap()),
            erasable: false,
        }),
        Token::EnumKw(EnumKw {
            level: 1,
            start: ByteIndex(src.find("enum1").unwrap()),
            erasable: false,
        }),
        Token::EnumKw(EnumKw {
            level: 33,
            start: ByteIndex(src.find("enum33").unwrap()),
            erasable: false,
        }),
        Token::EnumKw(EnumKw {
            level: 0,
            start: ByteIndex(src.find("enum*").unwrap()),
            erasable: true,
        }),
        Token::EnumKw(EnumKw {
            level: 1,
            start: ByteIndex(src.find("enum1*").unwrap()),
            erasable: true,
        }),
        Token::EnumKw(EnumKw {
            level: 33,
            start: ByteIndex(src.find("enum33*").unwrap()),
            erasable: true,
        }),
        Token::DefKw(ByteIndex(src.find("def").unwrap())),
        Token::MatchKw(ByteIndex(src.find("match").unwrap())),
        Token::ForKw(ByteIndex(src.find("For").unwrap())),
        Token::CaseKw(ByteIndex(src.find("case").unwrap())),
        Token::UseKw(ByteIndex(src.find("use").unwrap())),
        Token::EndKw(ByteIndex(src.find("end").unwrap())),
        Token::DecKw(ByteIndex(src.find("dec").unwrap())),
        Token::UniverseLiteral(UniverseLiteral {
            level: 0,
            start: ByteIndex(src.find("Type").unwrap()),
            erasable: false,
        }),
        Token::UniverseLiteral(UniverseLiteral {
            level: 1,
            start: ByteIndex(src.find("Type1").unwrap()),
            erasable: false,
        }),
        Token::UniverseLiteral(UniverseLiteral {
            level: 33,
            start: ByteIndex(src.find("Type33").unwrap()),
            erasable: false,
        }),
        Token::UniverseLiteral(UniverseLiteral {
            level: 0,
            start: ByteIndex(src.find("Type*").unwrap()),
            erasable: true,
        }),
        Token::UniverseLiteral(UniverseLiteral {
            level: 1,
            start: ByteIndex(src.find("Type1*").unwrap()),
            erasable: true,
        }),
        Token::UniverseLiteral(UniverseLiteral {
            level: 33,
            start: ByteIndex(src.find("Type33*").unwrap()),
            erasable: true,
        }),
    ]);
    assert_eq!(expected, actual);
}

#[test]
fn no_whitespace() {
    let src = r#"(def)"#;
    let actual = lex(src);
    let expected = Ok(vec![
        Token::LParen(ByteIndex(src.find("(").unwrap())),
        Token::DefKw(ByteIndex(src.find("def").unwrap())),
        Token::RParen(ByteIndex(src.find(")").unwrap())),
    ]);
    assert_eq!(expected, actual);
}

#[test]
fn comments() {
    let src = r#"(// Hello world!
// You can write comments on their own line.
def // You can also write them at the end of a line 
use)"#;
    let actual = lex(src);
    let expected = Ok(vec![
        Token::LParen(ByteIndex(src.find("(").unwrap())),
        Token::DefKw(ByteIndex(src.find("def").unwrap())),
        Token::UseKw(ByteIndex(src.find("use").unwrap())),
        Token::RParen(ByteIndex(src.find(")").unwrap())),
    ]);
    assert_eq!(expected, actual);
}

#[test]
fn enum0() {
    let src = r#"enum0"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}

#[test]
fn enum0_star() {
    let src = r#"enum0*"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}
#[test]
fn type0() {
    let src = r#"Type0"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}
#[test]
fn type0_star() {
    let src = r#"Type0*"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}

#[test]
fn enum00() {
    let src = r#"enum00"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}
#[test]
fn enum00_star() {
    let src = r#"enum00*"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}
#[test]
fn type00() {
    let src = r#"Type00"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}
#[test]
fn type00_star() {
    let src = r#"Type00*"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}

#[test]
fn enum01() {
    let src = r#"enum01"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}
#[test]
fn enum01_star() {
    let src = r#"enum01*"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}
#[test]
fn type01() {
    let src = r#"Type01"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}
#[test]
fn type01_star() {
    let src = r#"Type01*"#;
    let actual = lex(src);
    let expected = Err(LexError(ByteIndex(0), ByteIndex(src.len())));
    assert_eq!(expected, actual);
}

trait FindNth {
    /// `n` is zero-indexed.
    ///
    /// ```rust
    /// assert_eq!("abcxx".find_nth("x", 0), Some(3));
    /// assert_eq!("abcxx".find_nth("x", 1), Some(4));
    /// assert_eq!("abcxx".find_nth("x", 2), None);
    /// ```
    fn find_nth(&self, needle: &str, n: usize) -> Option<usize>;
}

impl FindNth for str {
    fn find_nth(&self, needle: &str, n: usize) -> Option<usize> {
        let mut last_needle_pos = self.find(needle)?;
        for _ in 0..n {
            let start = last_needle_pos + needle.len();
            let local_needle_pos = self[start..].find(needle)?;
            last_needle_pos = start + local_needle_pos;
        }
        Some(last_needle_pos)
    }
}
