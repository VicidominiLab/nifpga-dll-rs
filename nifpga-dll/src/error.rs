use std::error;
use std::fmt;

pub struct NifpgaError(i32);

impl fmt::Display for NifpgaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.0, match self.0 {
            -50400 => "The timeout expired before the FIFO operation could complete.",
            -50405 => "No transfer is in progress because the transfer was aborted by the client. The operation could not be completed as specified.",
            -52000 => "A memory allocation failed. Try again after rebooting.",
            -52003 => "An unexpected software error occurred.",
            -52005 => "A parameter to a function was not valid. This could be a NULL pointer, a bad value, etc.",
            -52006 => "A required resource was not found. The NiFpga.* library, the RIO resource, or some other resource may be missing.",
            -52010 => "A required resource was not properly initialized. This could occur if NiFpga_Initialize was not called or a required NiFpga_IrqContext was not reserved.",
            -61003 => "The FPGA is already running.",
            -61018 => "An error occurred downloading the VI to the FPGA device. Verify that the target is connected and powered and that the resource of the target is properly configured.",
            -61024 => "The bitfile was not compiled for the specified resource's device type.",
            -61046 => "An error was detected in the communication between the host computer and the FPGA target.",
            -61060 | 61060 => "The timeout expired before any of the IRQs were asserted.",
            -61070 => "The specified bitfile is invalid or corrupt.",
            -61072 => "The requested FIFO depth is invalid. It is either 0 or an amount not supported by the hardware.",
            -61073 => "The number of FIFO elements is invalid. Either the number is greater than the depth of the host memory DMA FIFO, or more elements were requested for release than had been acquired.",
            -61083 => "A hardware clocking error occurred. A derived clock lost lock with its base clock during the execution of the LabVIEW FPGA VI. If any base clocks with derived clocks are referencing an external source, make sure that the external source is connected and within the supported frequency, jitter, accuracy, duty cycle, and voltage specifications. Also verify that the characteristics of the base clock match the configuration specified in the FPGA Base Clock Properties. If all base clocks with derived clocks are generated from free-running, on-board sources, please contact National Instruments technical support at ni.com/support.",
            -61141 => "The operation could not be performed because the FPGA is busy. Stop all activities on the FPGA before requesting this operation. If the target is in Scan Interface programming mode, put it in FPGA Interface programming mode.",
            -61200 => "The operation could not be performed because the FPGA is busy operating in FPGA Interface C API mode. Stop all activities on the FPGA before requesting this operation.",
            -61201 => "The chassis is in Scan Interface programming mode. In order to run FPGA VIs, you must go to the chassis properties page, select FPGA programming mode, and deploy settings.",
            -61202 => "The operation could not be performed because the FPGA is busy operating in FPGA Interface mode. Stop all activities on the FPGA before requesting this operation.",
            -61203 => "The operation could not be performed because the FPGA is busy operating in Interactive mode. Stop all activities on the FPGA before requesting this operation.",
            -61204 => "The operation could not be performed because the FPGA is busy operating in Emulation mode. Stop all activities on the FPGA before requesting this operation.",
            -61211 => "LabVIEW FPGA does not support the Reset method for bitfiles that allow removal of implicit enable signals in single-cycle Timed Loops.",
            -61212 => "LabVIEW FPGA does not support the Abort method for bitfiles that allow removal of implicit enable signals in single-cycle Timed Loops.",
            -61213 => "LabVIEW FPGA does not support Close and Reset if Last Reference for bitfiles that allow removal of implicit enable signals in single-cycle Timed Loops. Pass NiFpga_CloseAttribute_NoResetIflastSession to NiFpga_Close instead.",
            -61214 => "For bitfiles that allow removal of implicit enable signals in single-cycle Timed Loops, LabVIEW FPGA does not support this method prior to running the bitfile.",
            -61215 => "Bitfiles that allow removal of implicit enable signals in single-cycle Timed Loops can run only once. Download the bitfile again before re-running the VI.",
            -61216 => "A gated clock has violated the handshaking protocol. If you are using external gated clocks, ensure that they follow the required clock gating protocol. If you are generating your clocks internally, please contact National Instruments Technical Support.",
            -61219 => "The number of elements requested must be less than or equal to the number of unacquired elements left in the host memory DMA FIFO. There are currently fewer unacquired elements left in the FIFO than are being requested. Release some acquired elements before acquiring more elements.",
            -61252 => "The operation could not be performed because the FPGA is in configuration or discovery mode. Wait for configuration or discovery to complete and retry your operation.",
            -61253 => "LabVIEW FPGA does not support Close and Reset if Last Reference for bitfiles that do not support Reset. Pass the NiFpga_CloseAttribute_NoResetIfLastSession attribute to NiFpga_Close instead of 0.",
            -63001 => "DMA from host to FPGA target is not supported for this remote system. Use another method for I/O or change the controller associated with the FPGA target.",
            -63002 => "The specified DMA FIFO depth is greater than the maximum size supported by this system.",
            -63003 => "The NI-RIO driver was unable to allocate memory for a FIFO. This can happen when the combined depth of all DMA FIFOs exceeds the maximum depth for the controller, or when the controller runs out of system memory. You may be able to reconfigure the controller with a greater maximum FIFO depth. For more information, refer to the NI KnowledgeBase article 65OF2ERQ.",
            -63030 => "Operation failed due to device reconfiguration. Multiple sessions to FPGA devices are not supported. Close the other session and retry this operation.",
            -63031 => "The operation could not be completed because another session is accessing the device. Close all other sessions and retry.",
            -61499 => "An unexpected internal error occurred.",
            -63033 => "Access to the remote system was denied. Use MAX to check the Remote Device Access settings under Software»NI-RIO»NI-RIO Settings on the remote system.",
            -63035 => "The attempt to open a RIO session failed because the driver was not yet initialized.",
            -63036 => "The operation could not be completed because the device being accessed was unavailable, either because it was disconnected from the network or because power was removed (e.g., the system went into a low-power state). Close any RIO FPGA sessions; check network, USB, and power connections; power-cycle the device if necessary; and open a new RIO session.",
            -63038 => "The NI-RIO software on the host is not compatible with the software on the target. Upgrade the NI-RIO software on the host in order to connect to this target.",
            -63040 => "A connection could not be established to the specified remote device. Ensure that the device is on and accessible over the network, that NI-RIO software is installed, and that the RIO server is running and properly configured.",
            -63041 => "The connection to the remote device has been lost due to an error on the remote device. Retry the operation. If the remote device continues to report this error, check its power supply and look for diagnostic messages on the console.",
            -63042 => "A fault on the network caused the operation to fail.",
            -63043 => "The RPC session is invalid. The target may have reset or been rebooted. Check the network connection and retry the operation.",
            -63044 => "The RIO server could not be found on the specified remote device. Ensure that NI-RIO software is installed and that the RIO server is running and properly configured.  Refer to Software>>NI-RIO>>NI-RIO Settings under the system in MAX.",
            -63045 => "The requested feature is not supported when using a remote RIO session. Opening a remote RIO session occurs when the resource string is specified as rio://hostname/device_name.  A local session can be opened by specifying just the device_name.",
            -63050 => "The specified trigger line is already reserved. Consult the MAX Trigger settings or the trigger reservations for each device within the system.",
            -63051 => "The specified trigger line is not reserved in the current session.",
            -63052 => "Trigger lines are not supported or enabled. For PXI, identify the controller and chassis using MAX.",
            -63070 => "The specified event type is invalid.",
            -63071 => "The specified RIO event has already been enabled for this session.",
            -63072 => "The specified RIO event has not been enabled for this session. Attempting a Wait on IRQ after an Abort causes this error.",
            -63073 => "The specified event did not occur within the specified time period, in milliseconds. Extend the time period, or ignore if the result was expected.",
            -63080 => "The allocated buffer is too small.",
            -63081 => "The caller did not allocate a memory buffer.",
            -63082 => "NiFpga_Status_FifoReserved	The operation could not complete because another session is accessing the FIFO. Close the other session and retry.",
            -63083 => "A Read FIFO or Write FIFO function was called while the host had acquired elements of the FIFO. Release all acquired elements before reading or writing.",
            -63084 => "A function was called using a misaligned address. The address must be a multiple of the size of the datatype.",
            -63085 => "The FPGA Read/Write Control Function is accessing a control or indicator with data that exceeds the maximum size supported on the current target. Refer to the hardware documentation for the limitations on data types for this target.",
            -63086 => "The FIFO function called cannot be used while the FIFO is running. To use this function, ensure the FIFO is stopped.",
            -63087 => "The function called does not match the datatype of the specified property. Ensure you are calling the correct function for the property.",
            -63088 => "Cannot acquire more regions from the selected FIFO. Release some regions and try again.",
            -63100 => "Invalid usage. The syntax is as follows: capigen.exe [-e] [<bitfile> [<output_directory> [<prefix_override>]]]",
            -63101 => "A valid .lvbitx bitfile is required. If you are using a valid .lvbitx bitfile, the bitfile may not be compatible with the software you are using. Determine which version of LabVIEW was used to make the bitfile, update your software to that version or later, and try again.",
            -63102 => "Invalid output directory.",
            -63103 => "Invalid prefix override. Prefixes must include only alphanumerics and underscores.",
            -63104 => "Could not convert '%s' to a valid C/C++ identifier.",
            -63105 => "The C/C++ identifier '%s' is already in use and could not be created.",
            -63106 => "The specified signature does not match the signature of the bitfile. If the bitfile has been recompiled, regenerate the C API and rebuild the application.",
            -63107 => "The bitfile you are trying to use is incompatible with the version of NI-RIO installed on the target and/or host. Update the version of NI-RIO on the target and/or host to the same version (or later) used to compile the bitfile. Alternatively, recompile the bitfile with the same version of NI-RIO that is currently installed on the target and/or host.",
            -63150 => "An unspecified hardware failure has occurred. The operation could not be completed.",
            -63160 => "You must download the bitfile to the flash memory on the FPGA device before you can set when the bitfile is autoloaded. Download the bitfile to the flash memory then try again.",
            -63170 => "The device has shut down to protect against excessive power consumption. Check airflow and cooling and reboot the system. To avoid this error, monitor the power consumption, power shutdown threshold, and margin of the device. If necessary, reduce FPGA power consumption.",
            -63171 => "The device has shut down to protect against excessive FPGA temperature. Check airflow and cooling and reboot the system. To avoid this error, monitor the FPGA temperature, temperature shutdown threshold, and margin of the FPGA. If necessary, reduce FPGA temperature.",
            -63180 => "The alias name is invalid. A valid alias name must meet the following conditions:  1) Cannot be or contain a reserved word. 2) Can use only alphanumeric characters, hyphens, and underscores. 3) Cannot conflict with the default alias of another device. A default alias is the name that the NI-RIO Device Driver assigns to a device based on its attributes if you have not assigned a custom alias to the device using MAX or the System Configuration API. For example, 'PXI1Slot3' is the default alias for the device at Slot 3of PXI Chassis 1, and it cannot be used as an alias name for any other device. Refer to the LabVIEW Help for NI-RIO for more information about aliases.",
            -63181 => "The supplied alias was not found.",
            -63182 => "An invalid device access setting was specified. RIO device access patterns may contain only alphanumerics, '-', '_', '.', and '*'.",
            -63183 => "An invalid port was specified. The RIO server port must be between 0 and 65535, where 0 indicates a dynamically assigned port. Port 3580 is reserved and cannot be used.",
            -63184 => "The selected bitfile requires a driver that could not be loaded or that was invalid. Use MAX to install all necessary software to your target. Note that if your project includes modules under the chassis item, user-defined variables, or references to the Scan Clock Chassis I/O item, you must install NI-RIO IO Scan software to the target.",
            -63187 => "This remote system does not support connections to other remote systems.",
            -63188 => "The operation is no longer supported.",
            -63189 => "The supplied search pattern is invalid.",
            -63192 => "Either the supplied resource name is invalid as a RIO resource name, or the device was not found. Use MAX to find the proper resource name for the intended device.",
            -63193 => "The requested feature is not supported.",
            -63194 => "The NI-RIO software on the target system is not compatible with this software. Upgrade the NI-RIO software on the target system.",
            -63195 => "The session is invalid or has been closed.",
            -63196 => "An invalid attribute has been specified.",
            -63197 => "An invalid attribute value has been specified.",
            -63198 => "The maximum number of open FPGA sessions has been reached. Close some open sessions.",
            -63199 => "The session could not be opened because another session with a different bitfile is already open on the device. Close the current session before opening another.",
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
