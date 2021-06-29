//! This crate defines an unsafe marker trait for types that deref to an address that is aliasable
//! when coerced to a raw pointer.

#![deny(
    warnings,
    rustdoc::all,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::ops::Deref;

/// An unsafe marker trait for types that deref to an address that is aliasable
/// when coerced to a raw pointer.
///
/// This means types of which their deref is backed by a `core::ptr::UniquePtr`
/// are not-applicable due to `noalias`.
pub unsafe trait AliasableDeref: Deref {}

unsafe impl<'a, T: ?Sized> AliasableDeref for core::cell::Ref<'a, T> {}

unsafe impl<'a, T: ?Sized> AliasableDeref for core::cell::RefMut<'a, T> {}

#[cfg(feature = "alloc")]
unsafe impl<T: ?Sized> AliasableDeref for alloc::rc::Rc<T> {}

#[cfg(feature = "alloc")]
unsafe impl<T: ?Sized> AliasableDeref for alloc::sync::Arc<T> {}

#[cfg(feature = "std")]
unsafe impl<'a, T: ?Sized> AliasableDeref for std::sync::MutexGuard<'a, T> {}

#[cfg(feature = "std")]
unsafe impl<'a, T: ?Sized> AliasableDeref for std::sync::RwLockReadGuard<'a, T> {}

#[cfg(feature = "std")]
unsafe impl<'a, T: ?Sized> AliasableDeref for std::sync::RwLockWriteGuard<'a, T> {}

#[cfg(test)]
mod tests {
    use core::cell::RefCell;

    #[cfg(feature = "alloc")]
    use alloc::{rc::Rc, sync::Arc};

    #[cfg(feature = "std")]
    use std::sync::{Mutex, RwLock};

    use super::AliasableDeref;

    #[test]
    fn test_cell_ref() {
        let ref_cell = RefCell::new(());
        let ptr = &ref_cell.borrow() as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

    #[test]
    fn test_cell_ref_mut() {
        let ref_cell = RefCell::new(());
        let ptr = &ref_cell.borrow_mut() as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_rc() {
        let ptr = &Rc::new(()) as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_arc() {
        let ptr = &Arc::new(()) as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_mutex_guard() {
        let mutex = Mutex::new(());
        let ptr = &mutex.lock().unwrap() as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_rw_lock_read_guard() {
        let rw_lock = RwLock::new(());
        let ptr = &rw_lock.read().unwrap() as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_rw_lock_write_guard() {
        let rw_lock = RwLock::new(());
        let ptr = &rw_lock.write().unwrap() as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }
}
