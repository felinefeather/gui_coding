use std::ops::{Deref, DerefMut};

pub struct Type<T>(T);

impl<T> Deref for Type<T> {
    fn deref(&self) -> &Self::Target {
        &self.0
    }   
    type Target = T;
}

impl<T> DerefMut for Type<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Type<T> {
    fn from(value: T) -> Self {
        Type(value)
    }
}