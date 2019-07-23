use crate::bump::Bump;

pub trait BumpClone<'bump> {
    fn bclone(&self, bump: &'bump Bump) -> Self;
}

/*
pub trait BumpSplit<'bump> {
    fn bsplit(&mut self, bump: &'bump Bump) -> Self;
}
*/

pub trait BumpExtend<'bump, A> {
    fn extend<T: IntoIterator<Item=A>>(&mut self, bump: &'bump Bump, iter: T);
}


/*
impl <'bump, T: BumpClone<'bump>> BumpSplit<'bump> for T {
    fn bsplit(&mut self, bump: &'bump Bump) -> Self {
        self.bclone(bump)
    }
}
*/