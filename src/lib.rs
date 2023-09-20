//! See [`AwaitableBool`] for all documentation.
use core::sync::atomic::{AtomicBool, Ordering};

use tokio::sync::Notify;

/// A bool whose value changes can be waited on.
///
/// Internally, it uses [`AtomicBool`]
/// and [`tokio::sync::Notify`].
///
/// Because of that, most methods only need a shared reference (`&` as opposed to `&mut`) to self,
/// so sharing between tasks or threads should be cheap and easy.
/// This struct doesn't implement [`Clone`], so place it in an [`Arc`]
/// (no lock protecting it necessary per the previous sentence) if cloning is needed.
///
/// [`Arc`]: std::sync::Arc
/// [`AtomicBool`]: std::sync::atomic::AtomicBool
#[derive(Debug, Default)]
pub struct AwaitableBool {
    bool: AtomicBool,
    notify: Notify,
}

impl<T: Into<AtomicBool>> From<T> for AwaitableBool {
    fn from(value: T) -> Self {
        Self {
            bool: value.into(),
            notify: Notify::new(),
        }
    }
}

impl AwaitableBool {
    /// Creates a new `AwaitableBool`.
    ///
    /// # Examples
    ///
    /// ```
    /// use awaitable_bool::AwaitableBool;
    ///
    /// let awaitable_true = AwaitableBool::new(true);
    /// let awaitable_false = AwaitableBool::new(false);
    /// ```
    pub fn new<IntoAtomicBool: Into<AtomicBool>>(value: IntoAtomicBool) -> Self {
        value.into().into()
    }

    /// Set the `AwaitableBool` to `true`
    /// (with [`Release`] ordering if not already `true`
    /// and [`Acquire`] ordering if it is).
    ///
    /// This wakes all tasks waiting for [`wait_true`].
    /// It also wakes those waiting for [`wait`] if the value wasn't already `true`.
    ///
    /// [`Acquire`]: core::sync::atomic::Ordering::Acquire
    /// [`Release`]: core::sync::atomic::Ordering::Release
    /// [`wait`]: AwaitableBool::wait
    /// [`wait_true`]: AwaitableBool::wait_true
    pub fn set_true(&self) {
        if self
            .bool
            .compare_exchange(false, true, Ordering::Release, Ordering::Acquire)
            .is_ok()
        {
            self.notify.notify_waiters();
        }
    }

    /// Set the `AwaitableBool` to `false`
    /// (with [`Release`] ordering if not already `false`
    /// and [`Acquire`] ordering if it is).
    ///
    /// This wakes all tasks waiting for [`wait_false`].
    /// It also wakes those waiting for [`wait`] if the value wasn't already `false`.
    ///
    /// [`Acquire`]: core::sync::atomic::Ordering::Acquire
    /// [`Release`]: core::sync::atomic::Ordering::Release
    /// [`wait`]: AwaitableBool::wait
    /// [`wait_false`]: AwaitableBool::wait_false
    pub fn set_false(&self) {
        if self
            .bool
            .compare_exchange(true, false, Ordering::Release, Ordering::Acquire)
            .is_ok()
        {
            self.notify.notify_waiters();
        }
    }

    /// Set the `AwaitableBool` to the inverse of its current value (i.e. `false` if `true` or `true` if `false`)
    /// (with [`AcqRel`] ordering).
    ///
    /// This wakes all tasks waiting for [`wait`].
    /// It also wakes those waiting for [`wait_true`] if the value was just changed from `false` to `true`,
    /// or those waiting for [`wait_false`] if the value was just changed from `true` to `false`.
    ///
    /// [`AcqRel`]: core::sync::atomic::Ordering::AcqRel
    /// [`wait`]: AwaitableBool::wait
    /// [`wait_false`]: AwaitableBool::wait_false
    /// [`wait_true`]: AwaitableBool::wait_true
    pub fn toggle(&self) {
        // Until AtomicBool::fetch_not is stable
        self.bool.fetch_xor(true, Ordering::AcqRel);

        self.notify.notify_waiters();
    }

    /// Get the current value of the `AwaitableBool`
    /// (with [`Acquire`] ordering).
    ///
    /// [`Acquire`]: core::sync::atomic::Ordering::Acquire
    #[inline]
    pub fn load(&self) -> bool {
        self.bool.load(Ordering::Acquire)
    }

    /// Check if the `AwaitableBool`'s value is currently `true`
    /// (with [`Acquire`] ordering).
    ///
    /// [`Acquire`]: core::sync::atomic::Ordering::Acquire
    #[inline]
    pub fn is_true(&self) -> bool {
        self.load()
    }
    /// Check if the `AwaitableBool`'s value is currently `false`
    /// (with [`Acquire`] ordering).
    ///
    /// [`Acquire`]: core::sync::atomic::Ordering::Acquire
    #[inline]
    pub fn is_false(&self) -> bool {
        !(self.load())
    }

    /// Wait for this [`AwaitableBool`]'s value to change.
    ///
    /// Use [`load`] after to know what it changed to.
    ///
    /// [`load`]: AwaitableBool::load
    pub async fn wait(&self) {
        self.notify.notified().await;
    }

    /// Wait for this [`AwaitableBool`]'s value to become `true`.
    /// This returns immediately if it's already `true`.
    pub async fn wait_true(&self) {
        if self.is_false() {
            self.wait().await;
        }
    }
    /// Wait for this [`AwaitableBool`]'s value to become `false`.
    /// This returns immediately if it's already `false`.
    pub async fn wait_false(&self) {
        if self.is_true() {
            self.wait().await;
        }
    }

    /// Consume this [`AwaitableBool`] to get the contained [`AtomicBool`].
    ///
    /// [`AtomicBool`] also has an [`into_inner`] method to get its contained [`bool`].
    ///
    /// [`AtomicBool`]: std::sync::atomic::AtomicBool
    /// [`into_inner`]: std::sync::atomic::AtomicBool::into_inner
    #[inline]
    pub const fn into_inner(self) -> AtomicBool {
        self.bool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializing_as_true() {
        let awaitable = AwaitableBool::new(true);

        assert!(awaitable.is_true());
        assert!(!awaitable.is_false());
    }

    #[test]
    fn initializing_as_false() {
        let awaitable = AwaitableBool::new(false);

        assert!(awaitable.is_false());
        assert!(!awaitable.is_true());
    }

    #[tokio::test]
    async fn waiting_for_true_when_true_is_immediate() {
        let awaitable = AwaitableBool::new(true);

        awaitable.wait_true().await;
    }

    #[tokio::test]
    async fn waiting_for_false_when_false_is_immediate() {
        let awaitable = AwaitableBool::new(false);

        awaitable.wait_false().await;
    }
}
