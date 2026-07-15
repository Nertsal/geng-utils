use std::future::Future;

use geng::prelude::{Geng, future::FutureExt};

/// Represents a wrapped future.
pub struct Task<T> {
    inner: async_executor::Task<T>,
}

impl<T: 'static> Task<T> {
    /// Wrap a future in the engine runtime. Has to be polled to make progress.
    pub fn new(geng: &Geng, future: impl Future<Output = T> + 'static) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let future = async_compat::Compat::new(future);

        Self {
            inner: geng.window().spawn(future),
        }
    }

    /// Tasks have to be manually polled (e.g. in the update function) to make progress.
    /// If the task is completed, returns `Ok(value)`, otherwise returns itself as `Err(task)`.
    pub fn poll(self) -> Result<T, Self> {
        if !self.inner.is_finished() {
            return Err(self);
        }
        Ok(self.inner.now_or_never().unwrap())
    }
}
