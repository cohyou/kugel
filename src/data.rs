use std::borrow::Cow;
pub enum Data<'a> {
    One(Cow<'a, str>),
    Labels(Vec<Cow<'a, str>>),
}

impl<'a> Data<'a> {
    pub fn one(label_name: &'a str) -> Self {
        Self::One(label_name.into())
    }
}
use std::ops::Add;

impl<'a> Add for Data<'a> {
    type Output = Data<'a>;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::One(s1) => {
                match rhs {
                    Self::One(s2) => {
                        Self::Labels(vec![s1, s2])
                    },
                    _ => unimplemented!(),
                }
            },
            _ => unimplemented!(),
        }
    }
}

#[test]
fn test_add() {
    let _ = Data::one("foo") + Data::one("bar");
}