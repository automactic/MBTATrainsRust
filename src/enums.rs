pub enum Line {
    Orange,
    Red,
    Blue,
    Green,
    Silver,
}

impl Line {
    pub fn as_str(&self) -> &str {
        match self {
            Line::Orange => "Orange",
            Line::Red => "Red",
            Line::Blue => "Blue",
            Line::Green => "Green",
            Line::Silver => "Silver",
        }
    }
}