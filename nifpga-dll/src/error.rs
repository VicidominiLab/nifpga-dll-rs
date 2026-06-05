use std::error;
use std::fmt;

pub struct NifpgaError(i32);

impl fmt::Display for NifpgaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, match self.0 {
            -50400 => "FIFO operation timed out.",
            -50405 => "The client aborted the transfer.",
            -52000 => "Memory allocation failed; reboot and retry.",
            -52003 => "Unexpected software error.",
            -52005 => "Invalid function parameter.",
            -52006 => "Required FPGA API, RIO resource, or other resource is missing.",
            -52010 => "A required resource was not initialized; initialize the FPGA API and reserve any needed IRQ context.",
            -61003 => "The FPGA is already running.",
            -61018 => "Failed to download to the FPGA device; check power, connection, and target configuration.",
            -61024 => "The bitfile targets a different device type.",
            -61046 => "Host-to-FPGA communication failed.",
            -61060 | 61060 => "No IRQ was asserted before timeout.",
            -61070 => "The bitfile is invalid or corrupt.",
            -61072 => "Invalid FIFO depth.",
            -61073 => "Invalid FIFO element count or release count.",
            -61083 => "Derived clock lost lock; check clock source, settings, and electrical limits.",
            -61141 => "FPGA is busy; stop FPGA activity. If the target is in Scan Interface mode, switch it to FPGA Interface mode.",
            -61200 => "FPGA is busy in FPGA Interface C API mode; stop FPGA activity and retry.",
            -61201 => "Chassis is in Scan Interface mode; switch to FPGA programming mode and deploy settings.",
            -61202 => "FPGA is busy in FPGA Interface mode; stop FPGA activity and retry.",
            -61203 => "FPGA is busy in Interactive mode; stop FPGA activity and retry.",
            -61204 => "FPGA is busy in Emulation mode; stop FPGA activity and retry.",
            -61211 => "Reset is not supported for bitfiles with removable implicit enable signals in single-cycle Timed Loops.",
            -61212 => "Abort is not supported for bitfiles with removable implicit enable signals in single-cycle Timed Loops.",
            -61213 => "Close-and-reset on last reference is unsupported for bitfiles with removable implicit enable signals; close without reset instead.",
            -61214 => "The requested method is unsupported before running bitfiles with removable implicit enable signals.",
            -61215 => "Bitfiles with removable implicit enable signals can run only once; download again before rerunning.",
            -61216 => "Gated-clock handshaking failed; check external clock gating, or inspect internally generated gated-clock logic.",
            -61219 => "Requested FIFO elements exceed the unacquired count; release acquired elements first.",
            -61252 => "FPGA is in configuration or discovery mode; wait and retry.",
            -61253 => "Close-and-reset on last reference is unsupported for bitfiles that cannot reset; close without reset.",
            -63001 => "Host-to-FPGA DMA is unsupported on this remote system; use other I/O or a different controller.",
            -63002 => "DMA FIFO depth exceeds this system's limit.",
            -63003 => "RIO driver could not allocate FIFO memory; reduce combined DMA FIFO depth or increase the controller FIFO-memory limit if supported.",
            -63030 => "Device reconfiguration interrupted the operation; close the other FPGA session and retry.",
            -63031 => "Another session is using the device; close it and retry.",
            -61499 => "Unexpected internal error.",
            -63033 => "Remote access denied; check remote device access settings.",
            -63035 => "Cannot open RIO session because the driver is not initialized.",
            -63036 => "Device is unavailable; close RIO FPGA sessions, check network, USB, and power, power-cycle if needed, then open a new session.",
            -63038 => "Host and target RIO software versions are incompatible; update the host.",
            -63040 => "Cannot connect to the remote device; verify network access, RIO software, and RIO server status.",
            -63041 => "Remote device connection was lost; retry, then check power and console diagnostics if it persists.",
            -63042 => "Network fault caused the operation to fail.",
            -63043 => "Invalid RPC session; the target may have reset or rebooted. Check the network and retry.",
            -63044 => "RIO server not found on the remote device; verify RIO software and server configuration.",
            -63045 => "Feature not supported over remote RIO sessions; use a local device name instead of rio://host/device.",
            -63050 => "Trigger line is already reserved.",
            -63051 => "Trigger line is not reserved in this session.",
            -63052 => "Trigger lines are unsupported or disabled; for PXI, check controller and chassis configuration.",
            -63070 => "Invalid event type.",
            -63071 => "RIO event is already enabled for this session.",
            -63072 => "RIO event is not enabled for this session; this can happen when Wait on IRQ is called after Abort.",
            -63073 => "Event did not occur before timeout; extend the timeout or ignore if expected.",
            -63080 => "Allocated buffer is too small.",
            -63081 => "Memory buffer was not allocated by the caller.",
            -63082 => "FIFO is reserved by another session; close that session and retry.",
            -63083 => "Cannot read or write while FIFO elements are acquired; release them first.",
            -63084 => "Misaligned address; align it to the data type size.",
            -63085 => "FPGA control or indicator data exceeds the target limit; check hardware data-type limits.",
            -63086 => "FIFO must be stopped before calling this function.",
            -63087 => "Function does not match the property's data type.",
            -63088 => "Too many FIFO regions acquired; release some and retry.",
            -63100 => "Invalid usage. Syntax: capigen.exe [-e] [<bitfile> [<output_directory> [<prefix_override>]]]",
            -63101 => "A valid compatible .lvbitx bitfile is required; use software matching or newer than the bitfile's build version.",
            -63102 => "Invalid output directory.",
            -63103 => "Invalid prefix override; use only letters, digits, and underscores.",
            -63104 => "Cannot convert '%s' to a valid C/C++ identifier.",
            -63105 => "C/C++ identifier '%s' is already in use.",
            -63106 => "Bitfile signature mismatch; regenerate the C API and rebuild if the bitfile changed.",
            -63107 => "Bitfile and RIO software versions are incompatible; update host/target software to the compile version or newer, or rebuild with the installed version.",
            -63150 => "Unspecified hardware failure.",
            -63160 => "Download the bitfile to FPGA device flash before configuring autoload.",
            -63170 => "Device shut down due to high power draw; improve cooling, reboot, and reduce FPGA power use.",
            -63171 => "Device shut down due to high FPGA temperature; improve cooling, reboot, and reduce temperature.",
            -63180 => "Invalid alias; use allowed characters, avoid reserved words, and do not conflict with another device's default alias.",
            -63181 => "Alias not found.",
            -63182 => "Invalid device access setting; RIO access patterns allow letters, digits, '-', '_', '.', and '*'.",
            -63183 => "Invalid RIO server port; use 0-65535 except reserved port 3580.",
            -63184 => "Bitfile driver is missing or invalid; install required target software, including RIO I/O Scan when modules, user variables, or Scan Clock I/O are used.",
            -63187 => "This remote system cannot connect to other remote systems.",
            -63188 => "Operation no longer supported.",
            -63189 => "Invalid search pattern.",
            -63192 => "Invalid RIO resource name or device not found; verify the correct resource name and that the device is present.",
            -63193 => "Feature not supported.",
            -63194 => "Target RIO software is incompatible; update it.",
            -63195 => "Session is invalid or closed.",
            -63196 => "Invalid attribute.",
            -63197 => "Invalid attribute value.",
            -63198 => "Too many FPGA sessions are open; close some sessions.",
            -63199 => "Device already has a session with a different bitfile; close it before opening another.",
            _ => "Unknown error."
        })
    }
}

impl fmt::Debug for NifpgaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl error::Error for NifpgaError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<i32> for NifpgaError {
    fn from(status: i32) -> NifpgaError {
        NifpgaError(status)
    }
}

pub trait ToResult {
    fn to_result(self) -> Result<(), NifpgaError>;
}

impl ToResult for i32 {
    fn to_result(self) -> Result<(), NifpgaError> {
        match self {
            0 => Ok(()),
            error => Err(NifpgaError(error)),
        }
    }
}
