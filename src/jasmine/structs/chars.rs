#[derive(Clone, Debug, PartialEq)]
pub enum Escape {
    Newline,
    Tab,
    CarriageReturn,
    Backslash,
    SingleQuote,
    DoubleQuote,
    NullByte,
    Unicode(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Character {
    Raw(char),
    Escape(Escape),
}
