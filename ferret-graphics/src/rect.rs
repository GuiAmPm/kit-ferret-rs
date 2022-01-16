pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T
}

impl<T> Rect<T> {
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Self {
            left, top, right, bottom
        }
    }
}

impl<T> From<(T, T, T, T)> for Rect<T> {
    fn from(tuple: (T, T, T, T)) -> Self {
        Rect::new(tuple.0, tuple.1, tuple.2, tuple.3)
    }
}