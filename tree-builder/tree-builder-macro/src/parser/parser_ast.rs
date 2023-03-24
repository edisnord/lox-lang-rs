#[derive(PartialEq, Eq, Debug)]
pub struct Terminal(pub String);

#[derive(PartialEq, Eq, Debug)]
pub struct Alternation {
    pub concatenation: Concatenation,
    pub identifier: Option<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Rule {
    pub lhs: String,
    pub rhs: Rhs
}

#[derive(PartialEq, Eq, Debug)]
pub struct Concatenation(pub Vec<Factor>);

#[derive(PartialEq, Eq, Debug)]
pub enum Factor {
    Optional(Term),
    ZeroOrMore(Term),
    OneOrMore(Term),
    Term(Term),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Metacharacter{
    AllChars,
    Digits,
    NonDigits,
    AlphaNumericUnderscore,
    NonAlphaNumericUnderscore,
    Whitespace,
    NonWhitespace,
    SquareBrackets(Vec<char>),
    ExcludingSquareBrackets(Vec<char>)
}

#[derive(PartialEq, Eq, Debug)]
pub enum Term {
    Metacharacter(Metacharacter),
    Terminal(Terminal),
    Grouping(Grouping),
    Ident(String),
    Include(Include)
}

#[derive(PartialEq, Eq, Debug)]
pub enum Include {
    Grouping(Grouping),
    Ident(String)
}

#[derive(PartialEq, Eq, Debug)]
pub struct Grouping(pub Box<Rhs>);

#[derive(PartialEq, Eq, Debug)]
pub struct Rhs(pub Vec<Alternation>);

#[derive(PartialEq, Eq, Debug)]
pub struct Specification(pub Vec<Rule>);