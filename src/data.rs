use std::borrow::Cow;
pub enum Inst<'a> {
    One(Cow<'a, str>),
    Labels(Vec<Cow<'a, str>>),
}

impl<'a> Inst<'a> {
    pub fn one(label_name: &'a str) -> Self {
        Self::One(label_name.into())
    }
}
use std::ops::Add;

impl<'a> Add for Inst<'a> {
    type Output = Inst<'a>;
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
    let _ = Inst::one("foo") + Inst::one("bar");
}