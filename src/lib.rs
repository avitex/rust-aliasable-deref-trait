//! This module defines a marker trait implemented for pointer types that
//! indicate their pointee's can be safely aliased.
#![no_std]
#![deny(missing_debug_implementations, missing_docs)]

use core::ops::Deref;

/// Marker trait for a pointer type that is allowed to have its
/// pointee aliased (except when dropped).
pub unsafe trait AliasableDeref: Deref {}

/// Helper trait for converting non-aliasable types
/// into their aliasable counterparts.
pub trait IntoAliasable {
    /// The aliasable type to convert to.
    type Target: AliasableDeref;

    /// Convert into an aliasable pointer type.
    fn into_aliasable(self) -> Self::Target;
}

impl<T> IntoAliasable for T
where
    T: AliasableDeref,
{
    type Target = T;

    fn into_aliasable(self) -> Self::Target {
        self
    }
}

unsafe impl<'a, T: ?Sized> AliasableDeref for core::cell::Ref<'a, T> {}

unsafe impl<'a, T: ?Sized> AliasableDeref for core::cell::RefMut<'a, T> {}

#[cfg(any(feature = "std", feature = "alloc", test))]
unsafe impl<T: ?Sized> AliasableDeref for self::std::rc::Rc<T> {}

#[cfg(any(feature = "std", feature = "alloc", test))]
unsafe impl<T: ?Sized> AliasableDeref for self::std::sync::Arc<T> {}

#[cfg(any(feature = "std", test))]
unsafe impl<'a, T: ?Sized> AliasableDeref for self::std::sync::MutexGuard<'a, T> {}

#[cfg(any(feature = "std", test))]
unsafe impl<'a, T: ?Sized> AliasableDeref for self::std::sync::RwLockReadGuard<'a, T> {}

#[cfg(any(feature = "std", test))]
unsafe impl<'a, T: ?Sized> AliasableDeref for self::std::sync::RwLockWriteGuard<'a, T> {}

#[cfg(any(feature = "std", test))]
mod std {
    extern crate std;
    pub use std::{rc, sync};
}

#[cfg(all(feature = "alloc", not(feature = "std"), not(test)))]
mod std {
    extern crate alloc;
    pub use alloc::{rc, sync};
}

#[cfg(test)]
mod tests {
    use core::{cell::RefCell, ops::Deref};

    use super::{
        std::{
            rc::Rc,
            sync::{Arc, Mutex, RwLock},
        },
        AliasableDeref, IntoAliasable,
    };

    #[test]
    fn test_rc() {
        let ptr = &Rc::new(()) as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

    #[test]
    fn test_arc() {
        let ptr = &Arc::new(()) as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

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
    fn test_mutex_guard() {
        let mutex = Mutex::new(());
        let ptr = &mutex.lock().unwrap() as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

    #[test]
    fn test_rw_lock_read_guard() {
        let rw_lock = RwLock::new(());
        let ptr = &rw_lock.read().unwrap() as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

    #[test]
    fn test_rw_lock_write_guard() {
        let rw_lock = RwLock::new(());
        let ptr = &rw_lock.write().unwrap() as &dyn AliasableDeref<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

    #[test]
    fn test_into_aliasable_owner() {
        let aliasable_ptr = Arc::new(()).into_aliasable();
        assert_eq!(aliasable_ptr.deref(), &());
    }
}
