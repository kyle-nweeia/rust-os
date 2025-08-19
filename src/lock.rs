use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

static LOCK: AtomicBool = AtomicBool::new(false);

pub struct Lock<T: ?Sized> {
    data: UnsafeCell<T>,
}

pub trait Mutex {
    type Type;

    fn lock<U>(&self, f: impl FnOnce(&mut Self::Type) -> U) -> U;
}

impl<T> Lock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> Mutex for Lock<T> {
    type Type = T;

    fn lock<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        loop {
            match LOCK.compare_exchange_weak(false, true, Ordering::AcqRel, Ordering::Relaxed) {
                Ok(false) => break,
                Err(true) => continue,
                _ => unreachable!(),
            }
        }

        let data = unsafe { &mut *self.data.get() };
        let u = f(data);

        LOCK.store(false, Ordering::Release);

        u
    }
}

unsafe impl<T> Sync for Lock<T> where T: ?Sized + Send {}
