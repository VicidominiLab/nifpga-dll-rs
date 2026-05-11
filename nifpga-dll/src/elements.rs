use crate::datatype::Type;
use crate::error::{NifpgaError, ToResult};
use crate::fifo::{ReadFifo, WriteFifo};

use fehler::throws;

pub struct WriteElements<'a, T: Type> {
    pub slice: &'a mut [T],
    handle_session: u32,
    handle_fifo: u32,
}

impl<'a, T: Type> WriteElements<'a, T> {
    pub unsafe fn from_raw(data: *mut T, len: usize, fifo: &WriteFifo<T>) -> Self {
        unsafe {
            WriteElements {
                slice: std::slice::from_raw_parts_mut(data, len),
                handle_session: fifo.session_handle,
                handle_fifo: fifo.handle,
            }
        }
    }

    #[throws(NifpgaError)]
    pub unsafe fn acquire(
        fifo: &WriteFifo<T>,
        num_elements: usize,
        timeout: u32,
    ) -> (WriteElements<'a, T>, usize, usize) {
        let (ptr, elements_acquired, elements_remaining) = T::acquire_fifo_write_elements(
            fifo.session_handle,
            fifo.handle,
            num_elements,
            timeout,
        )?;
        (
            WriteElements {
                slice: std::slice::from_raw_parts_mut(ptr, elements_acquired),
                handle_session: fifo.session_handle,
                handle_fifo: fifo.handle,
            },
            elements_acquired,
            elements_remaining,
        )
    }
}

impl<T: Type> Drop for WriteElements<'_, T> {
    fn drop(&mut self) {
        unsafe {
            nifpga_dll_sys::release_fifo_elements(
                self.handle_session,
                self.handle_fifo,
                self.slice.len(),
            )
            .to_result()
            .unwrap()
        }
    }
}

pub struct ReadElements<'a, T: Type> {
    pub slice: &'a [T],
    handle_session: u32,
    handle_fifo: u32,
}

impl<'a, T: Type> ReadElements<'a, T> {
    pub unsafe fn from_raw(data: *const T, len: usize, fifo: &ReadFifo<T>) -> Self {
        unsafe {
            ReadElements {
                slice: std::slice::from_raw_parts(data, len),
                handle_session: fifo.session_handle,
                handle_fifo: fifo.handle,
            }
        }
    }

    #[throws(NifpgaError)]
    pub unsafe fn acquire(
        fifo: &ReadFifo<T>,
        num_elements: usize,
        timeout: u32,
    ) -> (ReadElements<'a, T>, usize, usize) {
        let (ptr, elements_acquired, elements_remaining) =
            T::acquire_fifo_read_elements(fifo.session_handle, fifo.handle, num_elements, timeout)?;
        (
            ReadElements {
                slice: std::slice::from_raw_parts(ptr, elements_acquired),
                handle_session: fifo.session_handle,
                handle_fifo: fifo.handle,
            },
            elements_acquired,
            elements_remaining,
        )
    }
}

impl<T: Type> Drop for ReadElements<'_, T> {
    fn drop(&mut self) {
        unsafe {
            nifpga_dll_sys::release_fifo_elements(
                self.handle_session,
                self.handle_fifo,
                self.slice.len(),
            )
            .to_result()
            .unwrap()
        }
    }
}
