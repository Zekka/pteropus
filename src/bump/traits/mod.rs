use crate::bump::Bump;

pub trait BumpClone<'bump> {
    fn clone(&self, bump: &'bump Bump) -> Self;
}

pub trait BumpExtend<'bump, A> {
    fn extend<T: IntoIterator<Item=A>>(&mut self, bump: &'bump Bump, iter: T);
}
