#![no_std]

use core::{
    marker::PhantomData,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};
pub struct In<'a, T> {
    pub ptr: *mut T,
    _private: PhantomData<&'a ()>,
}
pub struct Out<'a, T> {
    pub ptr: *mut T,
    _private: PhantomData<&'a ()>,
}
impl<'a, T> In<'a, T> {
    pub fn fill(self, value: T) -> Out<'a, T> {
        unsafe {
            self.ptr.write(value);
        }
        Out {
            ptr: self.ptr,
            _private: self._private,
        }
    }
    pub unsafe fn raw<F>(raw: *mut T, f: F) -> Out<'a, T>
    where
        F: FnOnce(In<T>) -> Out<T>,
    {
        f(In {
            ptr: raw,
            _private: PhantomData,
        })
    }
}
impl<'a, T> Deref for Out<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}
impl<'a, T> DerefMut for Out<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}
impl<'a, T> Out<'a, T> {
    pub unsafe fn from_raw(a: *mut T) -> Self {
        Self {
            ptr: a,
            _private: PhantomData,
        }
    }
}
pub fn stack<T, E>(
    f: impl for<'a> FnOnce(In<'a, T>) -> Result<Out<'a, T>, (In<'a, T>, E)>,
) -> Result<T, E> {
    let mut x = MaybeUninit::uninit();
    match f(In {
        ptr: x.as_mut_ptr(),
        _private: PhantomData,
    }) {
        Err((_, e)) => return Err(e),
        Ok(o) => {
            return Ok(unsafe { o.ptr.read() });
        }
    }
}
#[macro_export]
macro_rules! init {
    ($p:ident.$field:ident @ $then:expr) => {
        $crate::In::raw(
            $crate::__::core::ptr::addr_of_mut!((*$p).$field),
            move |$p| $then,
        )
    };
    ($p:ident.$field:ident = $val:expr) => {
        $crate::In::raw($crate::__::core::ptr::addr_of_mut!((*$p).$field), |$p| {
            $p.fill($val)
        })
    };
}
#[doc(hidden)]
pub mod __ {
    pub use core;
}
