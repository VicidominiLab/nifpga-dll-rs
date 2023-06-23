extern crate libc;
use libc::{c_char, c_void, size_t};

//#[cfg_attr(feature = "static", link(name = "NiFpga", kind = "static"))]
//#[cfg_attr(not(feature = "static"), link(name = "NiFpga", kind = "dylib"))]
#[cfg_attr(not(feature = "static"), link(name = "NiFpga", kind = "raw-dylib"))]

extern "C" {
    // #[link_name = "NiFpga_Initialize"]
    // pub fn initialize() -> i32;

    #[link_name = "NiFpgaDll_Open"]
    pub fn open(
        bitfile: *const c_char,
        signature: *const c_char,
        resource: *const c_char,
        attribute: u32,
        session: *mut u32,
    ) -> i32;

    #[link_name = "NiFpgaDll_Close"]
    pub fn close(session: u32, attribute: u32) -> i32;

    // #[link_name = "NiFpga_Finalize"]
    // pub fn finalize() -> i32;

    #[link_name = "NiFpgaDll_Run"]
    pub fn run(session: u32) -> i32;

    #[link_name = "NiFpgaDll_Abort"]
    pub fn abort(session: u32) -> i32;

    #[link_name = "NiFpgaDll_Reset"]
    pub fn reset(session: u32) -> i32;

    #[link_name = "NiFpgaDll_Download"]
    pub fn download(session: u32) -> i32;

    #[link_name = "NiFpgaDll_ReadI8"]
    pub fn read_i8(session: u32, indicator: u32, value: *mut i8) -> i32;

    #[link_name = "NiFpgaDll_WriteI8"]
    pub fn write_i8(session: u32, control: u32, value: i8) -> i32;

    #[link_name = "NiFpgaDll_ReadArrayI8"]
    pub fn read_array_i8(session: u32, indicator: u32, array: *mut i8, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_WriteArrayI8"]
    pub fn write_array_i8(session: u32, indicator: u32, array: *const i8, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_ReadU8"]
    pub fn read_u8(session: u32, indicator: u32, value: *mut u8) -> i32;

    #[link_name = "NiFpgaDll_WriteU8"]
    pub fn write_u8(session: u32, control: u32, value: u8) -> i32;

    #[link_name = "NiFpgaDll_ReadArrayU8"]
    pub fn read_array_u8(session: u32, indicator: u32, array: *mut u8, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_WriteArrayU8"]
    pub fn write_array_u8(session: u32, indicator: u32, array: *const u8, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_ReadI16"]
    pub fn read_i16(session: u32, indicator: u32, value: *mut i16) -> i32;

    #[link_name = "NiFpgaDll_WriteI16"]
    pub fn write_i16(session: u32, control: u32, value: i16) -> i32;

    #[link_name = "NiFpgaDll_ReadArrayI16"]
    pub fn read_array_i16(session: u32, indicator: u32, array: *mut i16, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_WriteArrayI16"]
    pub fn write_array_i16(session: u32, indicator: u32, array: *const i16, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_ReadU16"]
    pub fn read_u16(session: u32, indicator: u32, value: *mut u16) -> i32;

    #[link_name = "NiFpgaDll_WriteU16"]
    pub fn write_u16(session: u32, control: u32, value: u16) -> i32;

    #[link_name = "NiFpgaDll_ReadArrayU16"]
    pub fn read_array_u16(session: u32, indicator: u32, array: *mut u16, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_WriteArrayU16"]
    pub fn write_array_u16(session: u32, indicator: u32, array: *const u16, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_ReadI32"]
    pub fn read_i32(session: u32, indicator: u32, value: *mut i32) -> i32;

    #[link_name = "NiFpgaDll_WriteI32"]
    pub fn write_i32(session: u32, control: u32, value: i32) -> i32;

    #[link_name = "NiFpgaDll_ReadArrayI32"]
    pub fn read_array_i32(session: u32, indicator: u32, array: *mut i32, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_WriteArrayI32"]
    pub fn write_array_i32(session: u32, indicator: u32, array: *const i32, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_ReadU32"]
    pub fn read_u32(session: u32, indicator: u32, value: *mut u32) -> i32;

    #[link_name = "NiFpgaDll_WriteU32"]
    pub fn write_u32(session: u32, control: u32, value: u32) -> i32;

    #[link_name = "NiFpgaDll_ReadArrayU32"]
    pub fn read_array_u32(session: u32, indicator: u32, array: *mut u32, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_WriteArrayU32"]
    pub fn write_array_u32(session: u32, indicator: u32, array: *const u32, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_ReadI64"]
    pub fn read_i64(session: u32, indicator: u32, value: *mut i64) -> i32;

    #[link_name = "NiFpgaDll_WriteI64"]
    pub fn write_i64(session: u32, control: u32, value: i64) -> i32;

    #[link_name = "NiFpgaDll_ReadArrayI64"]
    pub fn read_array_i64(session: u32, indicator: u32, array: *mut i64, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_WriteArrayI64"]
    pub fn write_array_i64(session: u32, indicator: u32, array: *const i64, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_ReadU64"]
    pub fn read_u64(session: u32, indicator: u32, value: *mut u64) -> i32;

    #[link_name = "NiFpgaDll_WriteU64"]
    pub fn write_u64(session: u32, control: u32, value: u64) -> i32;

    #[link_name = "NiFpgaDll_ReadArrayU64"]
    pub fn read_array_u64(session: u32, indicator: u32, array: *mut u64, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_WriteArrayU64"]
    pub fn write_array_u64(session: u32, indicator: u32, array: *const u64, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_ReadSgl"]
    pub fn read_f32(session: u32, indicator: u32, value: *mut f32) -> i32;

    #[link_name = "NiFpgaDll_WriteSgl"]
    pub fn write_f32(session: u32, control: u32, value: f32) -> i32;

    #[link_name = "NiFpgaDll_ReadArraySgl"]
    pub fn read_array_f32(session: u32, indicator: u32, array: *mut f32, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_WriteArraySgl"]
    pub fn write_array_f32(session: u32, indicator: u32, array: *const f32, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_ReadDbl"]
    pub fn read_f64(session: u32, indicator: u32, value: *mut f64) -> i32;

    #[link_name = "NiFpgaDll_WriteDbl"]
    pub fn write_f64(session: u32, control: u32, value: f64) -> i32;

    #[link_name = "NiFpgaDll_ReadArrayDbl"]
    pub fn read_array_f64(session: u32, indicator: u32, array: *mut f64, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_WriteArrayDbl"]
    pub fn write_array_f64(session: u32, indicator: u32, array: *const f64, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_ReadBool"]
    pub fn read_bool(session: u32, indicator: u32, value: *mut bool) -> i32;

    #[link_name = "NiFpgaDll_WriteBool"]
    pub fn write_bool(session: u32, control: u32, value: bool) -> i32;

    #[link_name = "NiFpgaDll_ReadArrayBool"]
    pub fn read_array_bool(session: u32, indicator: u32, array: *mut bool, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_WriteArrayBool"]
    pub fn write_array_bool(session: u32, indicator: u32, array: *const bool, size: size_t) -> i32;

    #[link_name = "NiFpgaDll_ReserveIrqContext"]
    pub fn reserve_irq_context(session: u32, context: *mut *const c_void) -> i32;

    #[link_name = "NiFpgaDll_UnreserveIrqContext"]
    pub fn unreserve_irq_context(session: u32, context: *const c_void) -> i32;

    #[link_name = "NiFpgaDll_WaitOnIrqs"]
    pub fn wait_on_irqs(
        session: u32,
        context: *const c_void,
        irqs: u32,
        timeout: u32,
        irqsAsserted: *mut u32,
        timedOut: *mut u8,
    ) -> i32;
    
    #[link_name = "NiFpgaDll_AcknowledgeIrqs"]
    pub fn acknowledge_irqs(session: u32, irqs: u32) -> i32;

    #[link_name = "NiFpgaDll_ConfigureFifo"]
    pub fn configure_fifo(session: u32, fifo: u32, depth: size_t) -> i32;

    #[link_name = "NiFpgaDll_ConfigureFifo2"]
    pub fn configure_fifo2(
        session: u32,
        fifo: u32,
        requestedDepth: size_t,
        actualDepth: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_StartFifo"]
    pub fn start_fifo(session: u32, fifo: u32) -> i32;

    #[link_name = "NiFpgaDll_StopFifo"]
    pub fn stop_fifo(session: u32, fifo: u32) -> i32;

    #[link_name = "NiFpgaDll_ReadFifoI8"]
    pub fn read_fifo_i8(
        session: u32,
        fifo: u32,
        data: *mut i8,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_WriteFifoI8"]
    pub fn write_fifo_i8(
        session: u32,
        fifo: u32,
        data: *const i8,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoReadElementsI8"]
    pub fn acquire_fifo_read_elements_i8(
        session: u32,
        fifo: u32,
        elements: *mut *const i8,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoWriteElementsI8"]
    pub fn acquire_fifo_write_elements_i8(
        session: u32,
        fifo: u32,
        elements: *mut *mut i8,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_ReadFifoU8"]
    pub fn read_fifo_u8(
        session: u32,
        fifo: u32,
        data: *mut u8,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_WriteFifoU8"]
    pub fn write_fifo_u8(
        session: u32,
        fifo: u32,
        data: *const u8,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoReadElementsU8"]
    pub fn acquire_fifo_read_elements_u8(
        session: u32,
        fifo: u32,
        elements: *mut *const u8,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoWriteElementsU8"]
    pub fn acquire_fifo_write_elements_u8(
        session: u32,
        fifo: u32,
        elements: *mut *mut u8,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_ReadFifoI16"]
    pub fn read_fifo_i16(
        session: u32,
        fifo: u32,
        data: *mut i16,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_WriteFifoI16"]
    pub fn write_fifo_i16(
        session: u32,
        fifo: u32,
        data: *const i16,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoReadElementsI16"]
    pub fn acquire_fifo_read_elements_i16(
        session: u32,
        fifo: u32,
        elements: *mut *const i16,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoWriteElementsI16"]
    pub fn acquire_fifo_write_elements_i16(
        session: u32,
        fifo: u32,
        elements: *mut *mut i16,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_ReadFifoU16"]
    pub fn read_fifo_u16(
        session: u32,
        fifo: u32,
        data: *mut u16,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_WriteFifoU16"]
    pub fn write_fifo_u16(
        session: u32,
        fifo: u32,
        data: *const u16,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoReadElementsU16"]
    pub fn acquire_fifo_read_elements_u16(
        session: u32,
        fifo: u32,
        elements: *mut *const u16,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoWriteElementsU16"]
    pub fn acquire_fifo_write_elements_u16(
        session: u32,
        fifo: u32,
        elements: *mut *mut u16,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_ReadFifoI32"]
    pub fn read_fifo_i32(
        session: u32,
        fifo: u32,
        data: *mut i32,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_WriteFifoI32"]
    pub fn write_fifo_i32(
        session: u32,
        fifo: u32,
        data: *const i32,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoReadElementsI32"]
    pub fn acquire_fifo_read_elements_i32(
        session: u32,
        fifo: u32,
        elements: *mut *const i32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoWriteElementsI32"]
    pub fn acquire_fifo_write_elements_i32(
        session: u32,
        fifo: u32,
        elements: *mut *mut i32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_ReadFifoU32"]
    pub fn read_fifo_u32(
        session: u32,
        fifo: u32,
        data: *mut u32,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_WriteFifoU32"]
    pub fn write_fifo_u32(
        session: u32,
        fifo: u32,
        data: *const u32,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoReadElementsU32"]
    pub fn acquire_fifo_read_elements_u32(
        session: u32,
        fifo: u32,
        elements: *mut *const u32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoWriteElementsU32"]
    pub fn acquire_fifo_write_elements_u32(
        session: u32,
        fifo: u32,
        elements: *mut *mut u32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_ReadFifoI64"]
    pub fn read_fifo_i64(
        session: u32,
        fifo: u32,
        data: *mut i64,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_WriteFifoI64"]
    pub fn write_fifo_i64(
        session: u32,
        fifo: u32,
        data: *const i64,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoReadElementsI64"]
    pub fn acquire_fifo_read_elements_i64(
        session: u32,
        fifo: u32,
        elements: *mut *const i64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoWriteElementsI64"]
    pub fn acquire_fifo_write_elements_i64(
        session: u32,
        fifo: u32,
        elements: *mut *mut i64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_ReadFifoU64"]
    pub fn read_fifo_u64(
        session: u32,
        fifo: u32,
        data: *mut u64,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_WriteFifoU64"]
    pub fn write_fifo_u64(
        session: u32,
        fifo: u32,
        data: *const u64,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoReadElementsU64"]
    pub fn acquire_fifo_read_elements_u64(
        session: u32,
        fifo: u32,
        elements: *mut *const u64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoWriteElementsU64"]
    pub fn acquire_fifo_write_elements_u64(
        session: u32,
        fifo: u32,
        elements: *mut *mut u64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_ReadFifoSgl"]
    pub fn read_fifo_f32(
        session: u32,
        fifo: u32,
        data: *mut f32,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_WriteFifoSgl"]
    pub fn write_fifo_f32(
        session: u32,
        fifo: u32,
        data: *const f32,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoReadElementsSgl"]
    pub fn acquire_fifo_read_elements_f32(
        session: u32,
        fifo: u32,
        elements: *mut *const f32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoWriteElementsSgl"]
    pub fn acquire_fifo_write_elements_f32(
        session: u32,
        fifo: u32,
        elements: *mut *mut f32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_ReadFifoDbl"]
    pub fn read_fifo_f64(
        session: u32,
        fifo: u32,
        data: *mut f64,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_WriteFifoDbl"]
    pub fn write_fifo_f64(
        session: u32,
        fifo: u32,
        data: *const f64,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoReadElementsDbl"]
    pub fn acquire_fifo_read_elements_f64(
        session: u32,
        fifo: u32,
        elements: *mut *const f64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoWriteElementsDbl"]
    pub fn acquire_fifo_write_elements_f64(
        session: u32,
        fifo: u32,
        elements: *mut *mut f64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_ReadFifoBool"]
    pub fn read_fifo_bool(
        session: u32,
        fifo: u32,
        data: *mut bool,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_WriteFifoBool"]
    pub fn write_fifo_bool(
        session: u32,
        fifo: u32,
        data: *const bool,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoReadElementsBool"]
    pub fn acquire_fifo_read_elements_bool(
        session: u32,
        fifo: u32,
        elements: *mut *const bool,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_AcquireFifoWriteElementsBool"]
    pub fn acquire_fifo_write_elements_bool(
        session: u32,
        fifo: u32,
        elements: *mut *mut bool,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpgaDll_ReleaseFifoElements"]
    pub fn release_fifo_elements(session: u32, fifo: u32, elements: size_t) -> i32;

    #[link_name = "NiFpgaDll_GetPeerToPeerFifoEndpoint"]
    pub fn get_p2p_fifo_endpoint(session: u32, fifo: u32, endpoint: *mut u32) -> i32;

    #[link_name = "NiFpgaDll_GetBitfileContents"]
    pub fn get_bitfile_contents(session: u32, contents: *mut *const c_char) -> i32;

    #[link_name = "NiFpgaDll_ClientFunctionCall"]
    pub fn client_function_call(
        session: u32,
        group: u32,
        functionId: u32,
        inBuffer: *const c_void,
        inBufferSize: size_t,
        outBuffer: *mut c_void,
        outBufferSize: size_t,
    ) -> i32;
}
