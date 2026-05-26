extern crate nifpga_dll_sys;

use fehler::throws;
use libc::c_void;

use crate::error::{NifpgaError, ToResult};
use crate::session::Session;

pub struct Context {
    context: *const c_void,
    session_handle: u32,
}

// We guarantee that the data pointed to by "Context.context" can be safely
// transferred across thread boundaries without data races.
unsafe impl Send for Context {}

impl Context {
    #[throws(NifpgaError)]
    pub fn wait_on_irqs(&self, irqs: u32, timeout: u32) -> (u32, bool) {
        let mut irqs_asserted: u32 = Default::default();
        let mut timed_out: bool = Default::default();
        unsafe {
            nifpga_dll_sys::wait_on_irqs(
                self.session_handle,
                self.context,
                irqs,
                timeout,
                &mut irqs_asserted,
                &mut timed_out,
            )
            .to_result()?
        }
        (irqs_asserted, timed_out)
    }

    #[throws(NifpgaError)]
    pub fn reserve(session: &Session) -> Context {
        let mut context = std::ptr::null();
        unsafe { nifpga_dll_sys::reserve_irq_context(session.handle, &mut context).to_result()? }
        Context {
            context,
            session_handle: session.handle,
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            nifpga_dll_sys::unreserve_irq_context(self.session_handle, self.context)
                .to_result()
                .unwrap()
        }
    }
}
