use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum By {
    Values,
    Keys,
}


impl Display for By {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            By::Values => write!(f, "values"),
            By::Keys => write!(f, "keys"),
        }
    }
}
