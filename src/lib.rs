//! Provides an interface over RwLock providing a type which can only
//! perform read and a type which can only perform write on the lock.
//!
//! There is currently no performance optimization, however it helps
//! preventing some logic errors.

extern crate parking_lot;

use std::sync::Arc;
pub use parking_lot::{RwLockReadGuard, RwLockWriteGuard};

struct RwLockInner<T> {
    lock: parking_lot::RwLock<T>,
}

#[derive(Clone)]
pub struct RwLockReader<T> {
    inner: Arc<RwLockInner<T>>,
}

#[derive(Clone)]
pub struct RwLockWriter<T> {
    inner: Arc<RwLockInner<T>>,
}

pub fn new<T>(value: T) -> (RwLockReader<T>, RwLockWriter<T>) {
    let inner = Arc::new(RwLockInner {
        lock: parking_lot::RwLock::new(value),
    });
    let reader = RwLockReader {
        inner: Arc::clone(&inner),
    };
    let writer = RwLockWriter { inner: inner };
    (reader, writer)
}

impl<T> RwLockReader<T> {
    #[inline]
    pub fn read(&self) -> RwLockReadGuard<T> {
        self.inner.lock.read()
    }

    #[inline]
    pub fn try_read(&self) -> Option<RwLockReadGuard<T>> {
        self.inner.lock.try_read()
    }
}

impl<T> RwLockWriter<T> {
    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.inner.lock.write()
    }

    #[inline]
    pub fn try_write(&self) -> Option<RwLockWriteGuard<T>> {
        self.inner.lock.try_write()
    }

    #[inline]
    pub fn read(&self) -> RwLockReadGuard<T> {
        self.inner.lock.read()
    }

    #[inline]
    pub fn try_read(&self) -> Option<RwLockReadGuard<T>> {
        self.inner.lock.try_read()
    }

    /// Convert this writer into a reader.
    ///
    /// The conversion is only allowed in this direction.
    pub fn to_reader(&self) -> RwLockReader<T> {
        RwLockReader {
            inner: Arc::clone(&self.inner),
        }
    }
}
