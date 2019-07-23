// Saturated reference counting
//
// It's ref-counting with the interesting property that we only care about whether
// the count is ever greater than one.
//
// That means the count can be stored in the refs instead of on the object.
//
// This is equivalent to copy-on-write with mut refs instead of immut refs.
// (Thank rpjohnst for identifying the name of this pattern)

use crate::bump::Bump;

use crate::bump::traits::BumpClone;

enum Satc<'bump, A: BumpClone<'bump>> {
    Unique(&'bump mut A),
    Borrowed(&'bump A),
    InProcess,
}

impl<'bump, A: BumpClone<'bump>> Satc<'bump, A> {
    pub fn new(ptr: &'bump mut A) -> Self { Satc::Unique(ptr) }
    pub fn new_borrowed(ptr: &'bump A) -> Self { Satc::Borrowed(ptr) }

    pub fn as_mut<'borrow>(&'borrow mut self, bump: &'bump Bump) -> &'borrow mut A {
        match self {
            Satc::Unique(r) => *r,
            Satc::Borrowed(r) => {
                let owned = r.clone(bump);
                let mut_ref = bump.alloc(owned);
                *self = Satc::Unique(mut_ref);
                return self.as_mut(bump);
            }
            Satc::InProcess => unreachable!()
        }
    }

    pub fn split(&'bump mut self) -> Satc<'bump, A> {
        let mut swapzone = Satc::InProcess;
        std::mem::swap(self, &mut swapzone);

        let (s1, s2) = match swapzone {
            Satc::Unique(r) => {
                (Satc::Borrowed(r), Satc::Borrowed(r))
            }
            Satc::Borrowed(r) => {
                (Satc::Borrowed(r), Satc::Borrowed(r))
            }
            Satc::InProcess => unreachable!()
        };
        *self = s1;
        return s2;
    }
}

