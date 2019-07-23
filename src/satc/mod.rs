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
use std::hint::unreachable_unchecked;

enum Satc<'bump, A: BumpClone<'bump>> {
    Unique(&'bump mut A),
    Borrowed(&'bump A),
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
                unsafe { self.as_mut_unchecked() } // safe: we just set it to Unique
            }
        }
    }

    #[inline(always)]
    unsafe fn as_mut_unchecked(&mut self) -> &mut A {
        match self {
            Satc::Unique(r) => *r,
            _ => unreachable_unchecked()
        }
    }

    #[inline(always)]
    pub fn as_immut(&mut self) -> &A {
        match self {
            Satc::Unique(r) => r,
            Satc::Borrowed(r) => r,
        }
    }

    #[inline(always)]
    fn safe_split(self) -> (Satc<'bump, A>, Satc<'bump, A>) {
        match self {
            Satc::Unique(r) => { (Satc::Borrowed(r), Satc::Borrowed(r)) }
            Satc::Borrowed(r) => { (Satc::Borrowed(r), Satc::Borrowed(r)) }
        }
    }

    #[inline(always)]
    pub fn split(&mut self) -> Satc<'bump, A> {
        // safety: you can't get s1 or s2, which would duplicate the mut ref,
        // unless this function returns, after which `self` has been overwritten
        unsafe {
            let old_self = std::ptr::read(self);
            let (s1, s2) = old_self.safe_split();
            std::ptr::write(self, s1);
            s2
        }
    }
}

