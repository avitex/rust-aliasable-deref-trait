//! This module defines a marker trait implemented for pointer types that
//! are allowed to have their pointee aliased.
#![no_std]
#![deny(missing_debug_implementations, missing_docs)]

#[cfg(any(feature = "std", test))]
mod core {
    extern crate std;

    pub use std::{ops::Deref, ptr::NonNull, rc::Rc, sync::Arc, boxed::Box};
}

#[cfg(all(not(feature = "std"), not(test)))]
mod core {
    extern crate core;
    #[cfg(feature = "alloc")]
    extern crate alloc;

    pub use core::ops::{ops::Deref, ptr::NonNull};

    #[cfg(feature = "alloc")]
    pub use alloc::{rc::Rc, sync::Arc, boxed::Box};
}

use self::core::*;

/// Marker trait for a pointer type that is allowed to have its
/// pointee aliased (except when dropped).
pub unsafe trait AliasableOwner: Deref {}

#[cfg(any(feature = "alloc", feature = "std"))]
unsafe impl<T: ?Sized> AliasableOwner for Rc<T> {}

#[cfg(any(feature = "alloc", feature = "std"))]
unsafe impl<T: ?Sized> AliasableOwner for Arc<T> {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc() {
        let ptr = &Rc::new(()) as &dyn AliasableOwner<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }

    #[test]
    fn test_arc() {
        let ptr = &Arc::new(()) as &dyn AliasableOwner<Target = ()>;
        assert_eq!(ptr.deref(), &());
    }
}
