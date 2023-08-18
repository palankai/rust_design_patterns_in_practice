use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub trait Specification<T: std::fmt::Debug>: std::fmt::Debug {
    fn is_satisfied_by(&self, candidate: &T) -> bool;

    fn and(self, other: impl Specification<T> + 'static) -> SpecificationCompositions<T> where Self: 'static + Sized {
        SpecificationCompositions::And(vec![SpecificationCompositions::Specification(Arc::new(self)), SpecificationCompositions::Specification(Arc::new(other))])
    }
    fn or(self, other: impl Specification<T> + 'static) -> SpecificationCompositions<T> where Self: 'static + Sized {
        SpecificationCompositions::Or(vec![SpecificationCompositions::Specification(Arc::new(self)), SpecificationCompositions::Specification(Arc::new(other))])
    }
    fn invert(self) -> SpecificationCompositions<T> where Self: 'static + Sized {
        SpecificationCompositions::Invert(Box::new(SpecificationCompositions::Specification(Arc::new(self))))
    }
    fn xor(self, other: impl Specification<T> + 'static) -> SpecificationCompositions<T> where Self: 'static + Sized {
        SpecificationCompositions::Xor(vec![SpecificationCompositions::Specification(Arc::new(self)), SpecificationCompositions::Specification(Arc::new(other))])
    }
    fn composite(self) -> SpecificationCompositions<T> where Self: 'static + Sized {
        SpecificationCompositions::Specification(Arc::new(self))
    }

}


#[derive(Debug, Clone)]
pub enum SpecificationCompositions<T: std::fmt::Debug> {
    Specification(Arc<dyn Specification<T>>),
    And(Vec<SpecificationCompositions<T>>),
    Or(Vec<SpecificationCompositions<T>>),
    Xor(Vec<SpecificationCompositions<T>>),
    Invert(Box<SpecificationCompositions<T>>),
    True,
    False,
}


impl <T: std::fmt::Debug>Specification<T> for SpecificationCompositions<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        match self {
            Self::Specification(f) => f.is_satisfied_by(candidate),
            Self::And(specifications) => specifications.iter().all(|specification| specification.is_satisfied_by(candidate)),
            Self::Or(specifications) => specifications.iter().any(|specification| specification.is_satisfied_by(candidate)),
            Self::Invert(specification) => !specification.is_satisfied_by(candidate),
            Self::Xor(specifications) => specifications.iter().filter(|specification| specification.is_satisfied_by(candidate)).count() == 1,
            Self::True => true,
            Self::False => false,
        }
    }
}

impl <T: std::fmt::Debug>SpecificationCompositions<T> {
    pub fn and(self, other: impl Specification<T> + 'static) -> Self {
        let other = other.composite();
        match self {
            Self::And(mut specifications) => {
                match other {
                    Self::And(other_specifications) => {
                        specifications.extend(other_specifications);
                    },
                    _ => specifications.push(other)
                }
                Self::And(specifications)
            },
            _ => Self::And(vec![self, other])
        }
    }
    pub fn or(self, other: impl Specification<T> + 'static) -> Self {
        let other = other.composite();
        match self {
            Self::Or(mut specifications) => {
                match other {
                    Self::Or(other_specifications) => {
                        specifications.extend(other_specifications);
                    },
                    _ => specifications.push(other)
                }
                Self::Or(specifications)
            },
            _ => Self::Or(vec![self, other])
        }
    }
    pub fn xor(self, other: impl Specification<T> + 'static) -> Self {
        let other = other.composite();
        match self {
            Self::Xor(mut specifications) => {
                match other {
                    Self::Xor(other_specifications) => {
                        specifications.extend(other_specifications);
                    },
                    _ => specifications.push(other)
                }
                Self::Xor(specifications)
            },
            _ => Self::Xor(vec![self, other])
        }
    }
    pub fn invert(self) -> Self {
        Self::Invert(Box::new(self))
    }

    pub const fn composite(self) -> Self {
        self
    }

    fn reminder_unsatisfied_by(&self, candidate: &T) -> Option<Self> {
        match self {
            Self::Specification(f) => {
                if f.is_satisfied_by(candidate) {
                    return None;
                }
                Some(Self::Specification(f.clone()))
            },
            Self::And(specifications) => {
                let mut unsatisfied = Vec::new();
                for specification in specifications {
                    if !specification.is_satisfied_by(candidate) {
                        if let Some(reminder) = specification.reminder_unsatisfied_by(candidate) {
                            unsatisfied.push(reminder);
                        }
                    }
                }
                if unsatisfied.is_empty() {
                    return None;
                }
                if unsatisfied.len() == 1 {
                    return Some(unsatisfied.remove(0));
                }
                Some(Self::And(unsatisfied))
            },
            Self::Or(specifications) => {
                let mut unsatisfied = Vec::new();
                for specification in specifications {
                    if !specification.is_satisfied_by(candidate) {
                        if let Some(reminder) = specification.reminder_unsatisfied_by(candidate) {
                            unsatisfied.push(reminder);
                        }
                    }
                }
                if unsatisfied.is_empty() {
                    return None;
                }
                if unsatisfied.len() == 1 {
                    return Some(unsatisfied.remove(0));
                }
                Some(Self::Or(unsatisfied))
            },
            Self::Invert(specification) => specification.reminder_unsatisfied_by(candidate),
            Self::Xor(specifications) => {
                let mut unsatisfied = Vec::new();
                for specification in specifications {
                    if !specification.is_satisfied_by(candidate) {
                        if let Some(reminder) = specification.reminder_unsatisfied_by(candidate) {
                            unsatisfied.push(reminder);
                        }
                    }
                }
                if unsatisfied.is_empty() {
                    return None;
                }
                if unsatisfied.len() == 1 {
                    return Some(unsatisfied.remove(0));
                }
                Some(Self::Xor(unsatisfied))
            },
            Self::True => None,
            Self::False => None,
        }
    }
}

impl <T: std::fmt::Debug>Display for SpecificationCompositions<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Specification(s) => write!(f, "{:?}", s),
            Self::And(specifications) => {
                write!(f, "(")?;
                for (i, specification) in specifications.iter().enumerate() {
                    if i != 0 {
                        write!(f, " and ")?;
                    }
                    write!(f, "{}", specification)?;
                }
                write!(f, ")")
            },
            Self::Or(specifications) => {
                write!(f, "(")?;
                for (i, specification) in specifications.iter().enumerate() {
                    if i != 0 {
                        write!(f, " or ")?;
                    }
                    write!(f, "{}", specification)?;
                }
                write!(f, ")")
            },
            Self::Invert(specification) => write!(f, "not {}", specification),
            Self::Xor(specifications) => {
                write!(f, "(")?;
                for (i, specification) in specifications.iter().enumerate() {
                    if i != 0 {
                        write!(f, " xor ")?;
                    }
                    write!(f, "{}", specification)?;
                }
                write!(f, ")")
            },
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Clone)]
    struct GreaterThan {
        value: i32,
    }
    impl Specification<i32> for GreaterThan {
        fn is_satisfied_by(&self, candidate: &i32) -> bool {
            candidate > &self.value
        }
    }

    #[derive(Debug, Clone)]
    struct LessThan {
        value: i32,
    }
    impl Specification<i32> for LessThan {
        fn is_satisfied_by(&self, candidate: &i32) -> bool {
            candidate < &self.value
        }
    }

    #[derive(Debug, Clone)]
    struct Zero {}

    impl Specification<i32> for Zero {
        fn is_satisfied_by(&self, candidate: &i32) -> bool {
            candidate == &0
        }
    }


    #[test]
    fn test_simple() {

        let greater_than_5 = GreaterThan { value: 5 };

        let res = greater_than_5.is_satisfied_by(&6);
        assert!(res);

        let res = greater_than_5.is_satisfied_by(&3);
        assert!(!res);
    }

    #[test]
    fn test_and() {
        let greater_than_5 = GreaterThan { value: 5 };
        let less_than_10 = LessThan { value: 10 };

        let res = greater_than_5.clone().and(less_than_10.clone()).is_satisfied_by(&6);
        assert!(res);

        let res = greater_than_5.clone().and(less_than_10.clone()).is_satisfied_by(&3);
        assert!(!res);

        let res = greater_than_5.and(less_than_10).is_satisfied_by(&33);
        assert!(!res);

    }


    #[test]
    fn test_and_or() {
        let greater_than_5 = GreaterThan { value: 5 };
        let less_than_10 = LessThan { value: 10 };
        let zero = Zero {};
        let specification = greater_than_5.and(less_than_10).or(zero);

        let res = specification.is_satisfied_by(&6);
        assert!(res);

        let res = specification.is_satisfied_by(&3);
        assert!(!res);

        let res = specification.is_satisfied_by(&33);
        assert!(!res);

        let res = specification.is_satisfied_by(&0);
        assert!(res);
    }

    #[test]
    fn test_reminder_unsatisfied_by() {
        let greater_than_5 = GreaterThan { value: 5 };
        let less_than_10 = LessThan { value: 10 };
        let specification = greater_than_5.and(less_than_10);

        let res = specification.reminder_unsatisfied_by(&6);
        assert!(res.is_none());

        let res = specification.reminder_unsatisfied_by(&3);
        assert!(matches!( res, Some(SpecificationCompositions::Specification(..)) ));
    }

}
