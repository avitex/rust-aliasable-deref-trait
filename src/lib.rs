//! This module defines a marker trait implemented for pointer types that
//! are allowed to have their pointee aliased.
#![no_std]
#![deny(missing_debug_implementations, missing_docs)]

use core::ops::Deref;

/// Marker trait for a pointer type that is allowed to have its
/// pointee aliased (except when dropped).
pub unsafe trait AliasableDeref: Deref {}

#[cfg(any(feature = "alloc", feature = "std"))]
unsafe impl<T: ?Sized> AliasableDeref for self::alloc::Rc<T> {}

#[cfg(any(feature = "alloc", feature = "std"))]
unsafe impl<T: ?Sized> AliasableDeref for self::alloc::Arc<T> {}

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
    T: AliasableDeref
{
    type Target = T;

    fn into_aliasable(self) -> Self::Target {
        self
    }
}

#[cfg(any(feature = "std", test))]
mod alloc {
    extern crate std;
    pub use std::{rc::Rc, sync::Arc};
}

#[cfg(all(feature = "alloc", not(feature = "std"), not(test)))]
mod alloc {
    extern crate alloc;
    pub use alloc::{rc::Rc, sync::Arc};
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::alloc::*;

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
    fn test_into_aliasable_owner() {
        let aliasable_ptr = Arc::new(()).into_aliasable();
        assert_eq!(aliasable_ptr.deref(), &());
    } 
}
