#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct Generic<T> {
    pub t: [T; 40usize],
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Generic<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl<T> ::std::fmt::Debug for Generic<T> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Generic {{ t: Array with length 40 }}")
    }
}
#[repr(C)]
pub struct NoDebug<T> {
    pub t: [T; 40usize],
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for NoDebug<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
