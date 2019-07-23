use crate::bump::Bump;
use crate::bump::collections::Vec as BVec;
use crate::bump::traits::*;
use crate::interns::Intern;
use crate::primitive::Value;
use crate::satc::Satc;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BValue<'bump> {
    Integer(i64),
    Bool(bool),

    Compound(Intern, BVec<'bump, BValue<'bump>>),
    Vector(BVec<'bump, BValue<'bump>>),
    Set(BVec<'bump, BValue<'bump>>),
}

pub type SBV<'bump> = Satc<'bump, BValue<'bump>>;

impl<'bump> BValue<'bump> {
    pub fn lower(bump: &'bump Bump, v: &Value) -> BValue<'bump> {
        match v {
            Value::Integer(i) => BValue::Integer(*i),
            Value::Bool(b) => BValue::Bool(*b),

            Value::Compound(i, vs) => {
                let mut vs2 = BVec::with_capacity_in(bump, vs.len());
                for v in vs {
                    vs2.push(bump, BValue::lower(bump, v))
                }

                BValue::Compound(*i, vs2)
            },
            Value::Vector(vs) => {
                let mut vs2 = BVec::with_capacity_in(bump, vs.len());
                for v in vs {
                    vs2.push(bump, BValue::lower(bump, v))
                }
                BValue::Vector(vs2)
            },
            Value::Set(s) => panic!("sets not implemented yet"),
        }
    }

    pub fn raise(v: &BValue<'bump>) -> Value {
        match v {
            BValue::Integer(i) => Value::Integer(*i),
            BValue::Bool(b) => Value::Bool(*b),

            BValue::Compound(i, vs) => {
                let mut vs2 = Vec::new();
                for v in vs {
                    vs2.push(BValue::raise(v))
                }

                Value::Compound(*i, vs2)
            }

            BValue::Vector(vs) => {
                let mut vs2 = Vec::new();
                for v in vs {
                    vs2.push(BValue::raise(v))
                }

                Value::Vector(vs2)
            }

            BValue::Set(s) => panic!("sets not implemented yet"),
        }
    }

    pub fn clonesume(&mut self, bump: &'bump Bump) -> Self {
        match self {
            BValue::Integer(i) => BValue::Integer(*i),
            BValue::Bool(b) => BValue::Bool(*b),

            BValue::Compound(i, vs) => {
                let mut tmp = BVec::new();
                std::mem::swap(vs, &mut tmp);
                BValue::Compound(*i, tmp)
            }
            BValue::Vector(vs) => {
                let mut tmp = BVec::new();
                std::mem::swap(vs, &mut tmp);
                BValue::Vector(tmp)
            }
            BValue::Set(s) => panic!("sets not implemented yet"),
        }
    }
}

impl<'bump> BumpClone<'bump> for BValue<'bump> {
    fn bclone(&self, bump: &'bump Bump) -> Self {
        match self {
            BValue::Integer(i) => BValue::Integer(*i),
            BValue::Bool(b) => BValue::Bool(*b),

            BValue::Compound(i, vs) => {
                BValue::Compound(*i, vs.bclone(bump))
            },
            BValue::Vector(vs) => {
                BValue::Vector(vs.bclone(bump))
            },
            BValue::Set(vs) => panic!("sets not implemented yet"),
        }
    }
}

/*
impl<'bump> BumpSplit<'bump> for BValue<'bump> {
    fn bsplit(&mut self, bump: &'bump Bump) -> Self {
        match self {
            BValue::Integer(i) => BValue::Integer(*i),
            BValue::Bool(b) => BValue::Bool(*b),

            BValue::Compound(i, vs) => {
                BValue::Compound(*i, vs.bsplit(bump))
            },
            BValue::Vector(vs) => {
                BValue::Vector(vs.bsplit(bump))
            },
            BValue::Set(vs) => panic!("sets not implemented yet"),
        }
    }
}
*/
