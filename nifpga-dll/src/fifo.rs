use fehler::throws;
use std::marker::PhantomData;

use crate::datatype::Type;
use crate::elements::{ReadElements, WriteElements};
use crate::error::{NifpgaError, ToResult};
use crate::session::Session;

pub struct ReadFifo<T: Type> {
    pub(crate) handle: u32,
    pub(crate) session_handle: u32,
    phantom: PhantomData<T>,
}

impl<T: Type> ReadFifo<T> {
    #[throws(NifpgaError)]
    pub fn open(session: &Session, fifo: u32, depth: usize) -> (Self, usize) {
        let mut actual_depth = 0;
        unsafe {
            nifpga_dll_sys::configure_fifo2(session.handle, fifo, depth, &mut actual_depth)
                .to_result()?;
            nifpga_dll_sys::start_fifo(session.handle, fifo).to_result()?;
        };
        (
            ReadFifo {
                handle: fifo,
                session_handle: session.handle,
                phantom: PhantomData,
            },
            actual_depth,
        )
    }

    #[throws(NifpgaError)]
    pub fn read(&self, data: &mut [T], timeout: u32) -> usize {
        T::read_fifo(self.session_handle, self.handle, data, timeout)?
    }

    #[throws(NifpgaError)]
    pub unsafe fn acquire_elements<'a>(
        &'a self,
        num_elements: usize,
        timeout: u32,
    ) -> (ReadElements<'a, T>, usize, usize) {
        let (ptr, elements_acquired, elements_remaining) =
            T::acquire_fifo_read_elements(self.session_handle, self.handle, num_elements, timeout)?;
        (
            ReadElements::from_raw(ptr, elements_acquired, &self),
            elements_acquired,
            elements_remaining,
        )
    }
}

impl<T: Type> Drop for ReadFifo<T> {
    fn drop(&mut self) {
        unsafe {
            nifpga_dll_sys::stop_fifo(self.session_handle, self.handle)
                .to_result()
                .unwrap();
        };
    }
}

pub struct WriteFifo<T: Type> {
    pub(crate) handle: u32,
    pub(crate) session_handle: u32,
    phantom: PhantomData<T>,
}

impl<T: Type> WriteFifo<T> {
    #[throws(NifpgaError)]
    pub fn open(session: &Session, fifo: u32, depth: usize) -> (Self, usize) {
        let mut actual_depth = 0;
        unsafe {
            nifpga_dll_sys::configure_fifo2(session.handle, fifo, depth, &mut actual_depth)
                .to_result()?;
            nifpga_dll_sys::start_fifo(session.handle, fifo).to_result()?;
        };
        (
            WriteFifo {
                handle: fifo,
                session_handle: session.handle,
                phantom: PhantomData,
            },
            actual_depth,
        )
    }

    #[throws(NifpgaError)]
    pub fn write(&self, data: &[T], timeout: u32) -> usize {
        T::write_fifo(self.session_handle, self.handle, data, timeout)?
    }

    #[throws(NifpgaError)]
    pub unsafe fn acquire_elements<'a>(
        &'a self,
        num_elements: usize,
        timeout: u32,
    ) -> (WriteElements<'a, T>, usize, usize) {
        WriteElements::acquire(&self, num_elements, timeout)?
    }
}

impl<T: Type> Drop for WriteFifo<T> {
    fn drop(&mut self) {
        unsafe {
            nifpga_dll_sys::stop_fifo(self.session_handle, self.handle)
                .to_result()
                .unwrap();
        };
    }
}
