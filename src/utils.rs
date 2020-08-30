use std::cmp::Ordering;

pub(crate) struct Rev<T, F>
where
    T: Eq + PartialEq,
    F: Fn(&T, &T) -> Ordering,
{
    pub elem: T,
    pub cmp: F,
}

impl<T, F> PartialEq for Rev<T, F>
where
    T: Eq + PartialEq,
    F: Fn(&T, &T) -> Ordering,
{
    fn eq(&self, other: &Self) -> bool {
        self.elem == other.elem
    }
}

impl<T, F> Eq for Rev<T, F>
where
    T: Eq + PartialEq,
    F: Fn(&T, &T) -> Ordering,
{
}

impl<T, F> Ord for Rev<T, F>
where
    T: Eq + PartialEq,
    F: Fn(&T, &T) -> Ordering,
{
    fn cmp(&self, other: &Rev<T, F>) -> Ordering {
        (self.cmp)(&other.elem, &self.elem)
    }
}

impl<T, F> PartialOrd for Rev<T, F>
where
    T: Eq + PartialEq,
    F: Fn(&T, &T) -> Ordering,
{
    fn partial_cmp(&self, other: &Rev<T, F>) -> Option<Ordering> {
        Some((self.cmp)(&other.elem, &self.elem))
    }
}
