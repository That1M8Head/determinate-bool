use std::fmt;

#[derive(Debug)]
pub enum Determinate<T = ()> {
    True(T),
    False,
    Indeterminate,
}

impl<T: fmt::Display> fmt::Display for Determinate<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Determinate::True(value) => write!(f, "True({})", value),
            Determinate::False => write!(f, "False"),
            Determinate::Indeterminate => write!(f, "Indeterminate"),
        }
    }
}

impl Determinate {
    pub fn new() -> Self {
        Determinate::Indeterminate
    }
}

impl Determinate<u32> {
    pub fn from_bool(from_bool: bool) -> Self {
        if from_bool {
            Determinate::True(0)
        } 
        else {
            Determinate::False
        }
    }
}

impl<T> Determinate<T> {
    pub fn unwrap(self) -> T {
        match self {
            Determinate::True(value) => value,
            Determinate::False => panic!("Can't unwrap a false value"),
            Determinate::Indeterminate => panic!("Can't unwrap an indeterminate value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bool_true() {
        let result_true = Determinate::from_bool(true);
        assert_eq!(format!("{}", result_true), "True(0)");
    }

    #[test]
    fn test_from_bool_false() {
        let result_false = Determinate::from_bool(false);
        assert_eq!(format!("{}", result_false), "False(Condition is false)");
    }

    #[test]
    fn test_unwrap_true() {
        let result_true = Determinate::True("Test");
        assert_eq!(result_true.unwrap(), "Test");
    }

    #[test]
    #[should_panic(expected = "Can't unwrap a false value. The original reason was Condition is false")]
    fn test_unwrap_false() {
        let result_false = Determinate::<u32>::False;
        result_false.unwrap();
    }

    #[test]
    #[should_panic(expected = "Can't unwrap an indeterminate value")]
    fn test_unwrap_indeterminate() {
        let result_indeterminate = Determinate::<u32>::Indeterminate;
        result_indeterminate.unwrap();
    }
}
