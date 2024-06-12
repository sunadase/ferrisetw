//! Kernel Providers module
//!
//! Provides an easy way to create a Kernel Provider. Multiple providers are pre-created statically with
//! their appropriate GUID and flags
//! Credits: [KrabsETW::kernel_providers](https://github.com/microsoft/krabsetw/blob/master/krabs/krabs/kernel_providers.hpp)
// TODO: Extremely Verbose and cumbersome, think a way to do this in a more clean way
#![allow(dead_code)]

use super::GUID;

/// List of Kernel Providers GUIDs
///
/// Credits: [KrabsETW::kernel_guids](https://github.com/microsoft/krabsetw/blob/master/krabs/krabs/kernel_guids.hpp)
pub mod kernel_guids {
    use super::GUID;
    pub const ALPC_GUID: GUID = GUID::from_values(
        0x45d8cccd,
        0x539f,
        0x4b72,
        [0xa8, 0xb7, 0x5c, 0x68, 0x31, 0x42, 0x60, 0x9a],
    );
    pub const POWER_GUID: GUID = GUID::from_values(
        0xe43445e0,
        0x0903,
        0x48c3,
        [0xb8, 0x78, 0xff, 0x0f, 0xcc, 0xeb, 0xdd, 0x04],
    );
    pub const DEBUG_GUID: GUID = GUID::from_values(
        0x13976d09,
        0xa327,
        0x438c,
        [0x95, 0x0b, 0x7f, 0x03, 0x19, 0x28, 0x15, 0xc7],
    );
    pub const TCP_IP_GUID: GUID = GUID::from_values(
        0x9a280ac0,
        0xc8e0,
        0x11d1,
        [0x84, 0xe2, 0x00, 0xc0, 0x4f, 0xb9, 0x98, 0xa2],
    );
    pub const UDP_IP_GUID: GUID = GUID::from_values(
        0xbf3a50c5,
        0xa9c9,
        0x4988,
        [0xa0, 0x05, 0x2d, 0xf0, 0xb7, 0xc8, 0x0f, 0x80],
    );
    pub const THREAD_GUID: GUID = GUID::from_values(
        0x3d6fa8d1,
        0xfe05,
        0x11d0,
        [0x9d, 0xda, 0x00, 0xc0, 0x4f, 0xd7, 0xba, 0x7c],
    );
    pub const DISK_IO_GUID: GUID = GUID::from_values(
        0x3d6fa8d4,
        0xfe05,
        0x11d0,
        [0x9d, 0xda, 0x00, 0xc0, 0x4f, 0xd7, 0xba, 0x7c],
    );
    pub const FILE_IO_GUID: GUID = GUID::from_values(
        0x90cbdc39,
        0x4a3e,
        0x11d1,
        [0x84, 0xf4, 0x00, 0x00, 0xf8, 0x04, 0x64, 0xe3],
    );
    pub const PROCESS_GUID: GUID = GUID::from_values(
        0x3d6fa8d0,
        0xfe05,
        0x11d0,
        [0x9d, 0xda, 0x00, 0xc0, 0x4f, 0xd7, 0xba, 0x7c],
    );
    pub const REGISTRY_GUID: GUID = GUID::from_values(
        0xAE53722E,
        0xC863,
        0x11d2,
        [0x86, 0x59, 0x00, 0xC0, 0x4F, 0xA3, 0x21, 0xA1],
    );
    pub const SPLIT_IO_GUID: GUID = GUID::from_values(
        0xd837ca92,
        0x12b9,
        0x44a5,
        [0xad, 0x6a, 0x3a, 0x65, 0xb3, 0x57, 0x8a, 0xa8],
    );
    pub const OB_TRACE_GUID: GUID = GUID::from_values(
        0x89497f50,
        0xeffe,
        0x4440,
        [0x8c, 0xf2, 0xce, 0x6b, 0x1c, 0xdc, 0xac, 0xa7],
    );
    pub const UMS_EVENT_GUID: GUID = GUID::from_values(
        0x9aec974b,
        0x5b8e,
        0x4118,
        [0x9b, 0x92, 0x31, 0x86, 0xd8, 0x00, 0x2c, 0xe5],
    );
    pub const PERF_INFO_GUID: GUID = GUID::from_values(
        0xce1dbfb4,
        0x137e,
        0x4da6,
        [0x87, 0xb0, 0x3f, 0x59, 0xaa, 0x10, 0x2c, 0xbc],
    );
    pub const PAGE_FAULT_GUID: GUID = GUID::from_values(
        0x3d6fa8d3,
        0xfe05,
        0x11d0,
        [0x9d, 0xda, 0x00, 0xc0, 0x4f, 0xd7, 0xba, 0x7c],
    );
    pub const IMAGE_LOAD_GUID: GUID = GUID::from_values(
        0x2cb15d1d,
        0x5fc1,
        0x11d2,
        [0xab, 0xe1, 0x00, 0xa0, 0xc9, 0x11, 0xf5, 0x18],
    );
    pub const POOL_TRACE_GUID: GUID = GUID::from_values(
        0x0268a8b6,
        0x74fd,
        0x4302,
        [0x9d, 0xd0, 0x6e, 0x8f, 0x17, 0x95, 0xc0, 0xcf],
    );
    pub const LOST_EVENT_GUID: GUID = GUID::from_values(
        0x6a399ae0,
        0x4bc6,
        0x4de9,
        [0x87, 0x0b, 0x36, 0x57, 0xf8, 0x94, 0x7e, 0x7e],
    );
    pub const STACK_WALK_GUID: GUID = GUID::from_values(
        0xdef2fe46,
        0x7bd6,
        0x4b80,
        [0xbd, 0x94, 0xf5, 0x7f, 0xe2, 0x0d, 0x0c, 0xe3],
    );
    pub const EVENT_TRACE_GUID: GUID = GUID::from_values(
        0x68fdd900,
        0x4a3e,
        0x11d1,
        [0x84, 0xf4, 0x00, 0x00, 0xf8, 0x04, 0x64, 0xe3],
    );
    pub const MMCSS_TRACE_GUID: GUID = GUID::from_values(
        0xf8f10121,
        0xb617,
        0x4a56,
        [0x86, 0x8b, 0x9d, 0xf1, 0xb2, 0x7f, 0xe3, 0x2c],
    );
    pub const SYSTEM_TRACE_GUID: GUID = GUID::from_values(
        0x9e814aad,
        0x3204,
        0x11d2,
        [0x9a, 0x82, 0x00, 0x60, 0x08, 0xa8, 0x69, 0x39],
    );
    pub const EVENT_TRACE_CONFIG_GUID: GUID = GUID::from_values(
        0x01853a65,
        0x418f,
        0x4f36,
        [0xae, 0xfc, 0xdc, 0x0f, 0x1d, 0x2f, 0xd2, 0x35],
    );
}

/// List of Kernel Providers flags
///
/// More info: [EVENT_TRACE_PROPERTIES->EnableFlags](https://docs.microsoft.com/en-us/windows/win32/api/evntrace/ns-evntrace-event_trace_properties)
pub mod kernel_flags {
    pub const EVENT_TRACE_FLAG_PROCESS: u32 = 0x00000001;
    pub const EVENT_TRACE_FLAG_THREAD: u32 = 0x00000002;
    pub const EVENT_TRACE_FLAG_IMAGE_LOAD: u32 = 0x00000004;
    pub const EVENT_TRACE_FLAG_PROCESS_COUNTERS: u32 = 0x00000008;
    pub const EVENT_TRACE_FLAG_CSWITCH: u32 = 0x00000010;
    pub const EVENT_TRACE_FLAG_DPC: u32 = 0x00000020;
    pub const EVENT_TRACE_FLAG_INTERRUPT: u32 = 0x00000040;
    pub const EVENT_TRACE_FLAG_SYSTEMCALL: u32 = 0x00000080;
    pub const EVENT_TRACE_FLAG_DISK_IO: u32 = 0x00000100;
    pub const EVENT_TRACE_FLAG_DISK_FILE_IO: u32 = 0x00000200;
    pub const EVENT_TRACE_FLAG_DISK_IO_INIT: u32 = 0x00000400;
    pub const EVENT_TRACE_FLAG_DISPATCHER: u32 = 0x00000800;
    pub const EVENT_TRACE_FLAG_MEMORY_PAGE_FAULTS: u32 = 0x00001000;
    pub const EVENT_TRACE_FLAG_MEMORY_HARD_FAULTS: u32 = 0x00002000;
    pub const EVENT_TRACE_FLAG_VIRTUAL_ALLOC: u32 = 0x00004000;
    pub const EVENT_TRACE_FLAG_VAMAP: u32 = 0x00008000;
    pub const EVENT_TRACE_FLAG_NETWORK_TCPIP: u32 = 0x00010000;
    pub const EVENT_TRACE_FLAG_REGISTRY: u32 = 0x00020000;
    pub const EVENT_TRACE_FLAG_DBGPRINT: u32 = 0x00040000;
    pub const EVENT_TRACE_FLAG_ALPC: u32 = 0x00100000;
    pub const EVENT_TRACE_FLAG_SPLIT_IO: u32 = 0x00200000;
    pub const EVENT_TRACE_FLAG_DRIVER: u32 = 0x00800000;
    pub const EVENT_TRACE_FLAG_PROFILE: u32 = 0x01000000;
    pub const EVENT_TRACE_FLAG_FILE_IO: u32 = 0x02000000;
    pub const EVENT_TRACE_FLAG_FILE_IO_INIT: u32 = 0x04000000;
}

/// Contains kernel provider identifiers.
///
/// You'll need to use it with [`crate::provider::Provider::kernel`]
#[derive(Debug)]
pub struct KernelProvider {
    /// Kernel Provider GUID
    pub guid: GUID,
    /// Kernel Provider Flags
    pub flags: u32,
}

impl KernelProvider {
    /// Use the `new` function to create a Kernel Provider which can be then tied into a Provider
    pub const fn new(guid: GUID, flags: u32) -> KernelProvider {
        KernelProvider { guid, flags }
    }
}

/// Represents the VirtualAlloc Kernel Provider
pub static VIRTUAL_ALLOC_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::PAGE_FAULT_GUID,
    kernel_flags::EVENT_TRACE_FLAG_VIRTUAL_ALLOC,
);
/// Represents the VA Map Kernel Provider
pub static VAMAP_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::FILE_IO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_VAMAP,
);
/// Represents the Thread Kernel Provider
pub static THREAD_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::THREAD_GUID,
    kernel_flags::EVENT_TRACE_FLAG_THREAD,
);
/// Represents the Split IO Kernel Provider
pub static SPLIT_IO_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::SPLIT_IO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_SPLIT_IO,
);
/// Represents the SystemCall Kernel Provider
pub static SYSTEM_CALL_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::PERF_INFO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_SYSTEMCALL,
);
/// Represents the Registry Kernel Provider
pub static REGISTRY_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::REGISTRY_GUID,
    kernel_flags::EVENT_TRACE_FLAG_REGISTRY,
);
/// Represents the Profile Kernel Provider
pub static PROFILE_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::PERF_INFO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_PROFILE,
);
/// Represents the Process Counter Kernel Provider
pub static PROCESS_COUNTER_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::PROCESS_GUID,
    kernel_flags::EVENT_TRACE_FLAG_PROCESS_COUNTERS,
);
/// Represents the Process Kernel Provider
pub static PROCESS_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::PROCESS_GUID,
    kernel_flags::EVENT_TRACE_FLAG_PROCESS,
);
/// Represents the TCP-IP Kernel Provider
pub static TCP_IP_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::TCP_IP_GUID,
    kernel_flags::EVENT_TRACE_FLAG_NETWORK_TCPIP,
);
/// Represents the Memory Page Fault Kernel Provider
pub static MEMORY_PAGE_FAULT_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::PAGE_FAULT_GUID,
    kernel_flags::EVENT_TRACE_FLAG_MEMORY_PAGE_FAULTS,
);
/// Represents the Memory Hard Fault Kernel Provider
pub static MEMORY_HARD_FAULT_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::PAGE_FAULT_GUID,
    kernel_flags::EVENT_TRACE_FLAG_MEMORY_HARD_FAULTS,
);
/// Represents the Interrupt Kernel Provider
pub static INTERRUPT_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::PERF_INFO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_INTERRUPT,
);
/// Represents the Driver Kernel Provider
pub static DRIVER_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::DISK_IO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_DISK_IO,
);
/// Represents the DPC Kernel Provider
pub static DPC_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::PERF_INFO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_DPC,
);
/// Represents the Image Load Kernel Provider
pub static IMAGE_LOAD_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::IMAGE_LOAD_GUID,
    kernel_flags::EVENT_TRACE_FLAG_IMAGE_LOAD,
);
/// Represents the Thread Dispatcher Kernel Provider
pub static THREAD_DISPATCHER_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::THREAD_GUID,
    kernel_flags::EVENT_TRACE_FLAG_DISPATCHER,
);
/// Represents the File Init IO Kernel Provider
pub static FILE_INIT_IO_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::FILE_IO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_FILE_IO_INIT,
);
/// Represents the File IO Kernel Provider
pub static FILE_IO_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::FILE_IO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_FILE_IO,
);
/// Represents the Disk IO Init Kernel Provider
pub static DISK_IO_INIT_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::DISK_IO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_DISK_IO_INIT,
);
/// Represents the Disk IO Kernel Provider
pub static DISK_IO_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::DISK_IO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_DISK_IO,
);
/// Represents the Disk File IO Kernel Provider
pub static DISK_FILE_IO_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::DISK_IO_GUID,
    kernel_flags::EVENT_TRACE_FLAG_DISK_FILE_IO,
);
/// Represents the Dbg Pring Kernel Provider
pub static DEBUG_PRINT_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::DEBUG_GUID,
    kernel_flags::EVENT_TRACE_FLAG_DBGPRINT,
);
/// Represents the Context Swtich Kernel Provider
pub static CONTEXT_SWITCH_PROVIDER: KernelProvider = KernelProvider::new(
    kernel_guids::THREAD_GUID,
    kernel_flags::EVENT_TRACE_FLAG_CSWITCH,
);
/// Represents the ALPC Kernel Provider
pub static ALPC_PROVIDER: KernelProvider =
    KernelProvider::new(kernel_guids::ALPC_GUID, kernel_flags::EVENT_TRACE_FLAG_ALPC);

#[cfg(test)]
mod test {
    use super::kernel_flags::*;
    use super::kernel_guids::*;
    use super::*;

    use crate::provider::Provider;

    #[test]
    fn test_kernel_provider_struct() {
        let kernel_provider =
            KernelProvider::new("D396B546-287D-4712-A7F5-8BE226A8C643".into(), 0x10000);

        assert_eq!(0x10000, kernel_provider.flags);
        assert_eq!(
            GUID::from("D396B546-287D-4712-A7F5-8BE226A8C643"),
            kernel_provider.guid
        );
    }

    #[test]
    fn test_kernel_provider_is_binded_to_provider() {
        let kernel_provider = Provider::kernel(&IMAGE_LOAD_PROVIDER).build();

        assert_eq!(EVENT_TRACE_FLAG_IMAGE_LOAD, kernel_provider.kernel_flags());
        assert_eq!(IMAGE_LOAD_GUID, kernel_provider.guid());
    }

    #[test]
    fn test_kernel_provider_guids_correct() {
        assert_eq!(
            ALPC_GUID,
            GUID::from("45d8cccd-539f-4b72-a8b7-5c683142609a")
        );
        assert_eq!(
            POWER_GUID,
            GUID::from("e43445e0-0903-48c3-b878-ff0fccebdd04")
        );
        assert_eq!(
            DEBUG_GUID,
            GUID::from("13976d09-a327-438c-950b-7f03192815c7")
        );
        assert_eq!(
            TCP_IP_GUID,
            GUID::from("9a280ac0-c8e0-11d1-84e2-00c04fb998a2")
        );
        assert_eq!(
            UDP_IP_GUID,
            GUID::from("bf3a50c5-a9c9-4988-a005-2df0b7c80f80")
        );
        assert_eq!(
            THREAD_GUID,
            GUID::from("3d6fa8d1-fe05-11d0-9dda-00c04fd7ba7c")
        );
        assert_eq!(
            DISK_IO_GUID,
            GUID::from("3d6fa8d4-fe05-11d0-9dda-00c04fd7ba7c")
        );
        assert_eq!(
            FILE_IO_GUID,
            GUID::from("90cbdc39-4a3e-11d1-84f4-0000f80464e3")
        );
        assert_eq!(
            PROCESS_GUID,
            GUID::from("3d6fa8d0-fe05-11d0-9dda-00c04fd7ba7c")
        );
        assert_eq!(
            REGISTRY_GUID,
            GUID::from("AE53722E-C863-11d2-8659-00C04FA321A1")
        );
        assert_eq!(
            SPLIT_IO_GUID,
            GUID::from("d837ca92-12b9-44a5-ad6a-3a65b3578aa8")
        );
        assert_eq!(
            OB_TRACE_GUID,
            GUID::from("89497f50-effe-4440-8cf2-ce6b1cdcaca7")
        );
        assert_eq!(
            UMS_EVENT_GUID,
            GUID::from("9aec974b-5b8e-4118-9b92-3186d8002ce5")
        );
        assert_eq!(
            PERF_INFO_GUID,
            GUID::from("ce1dbfb4-137e-4da6-87b0-3f59aa102cbc")
        );
        assert_eq!(
            PAGE_FAULT_GUID,
            GUID::from("3d6fa8d3-fe05-11d0-9dda-00c04fd7ba7c")
        );
        assert_eq!(
            IMAGE_LOAD_GUID,
            GUID::from("2cb15d1d-5fc1-11d2-abe1-00a0c911f518")
        );
        assert_eq!(
            POOL_TRACE_GUID,
            GUID::from("0268a8b6-74fd-4302-9dd0-6e8f1795c0cf")
        );
        assert_eq!(
            LOST_EVENT_GUID,
            GUID::from("6a399ae0-4bc6-4de9-870b-3657f8947e7e")
        );
        assert_eq!(
            STACK_WALK_GUID,
            GUID::from("def2fe46-7bd6-4b80-bd94-f57fe20d0ce3")
        );
        assert_eq!(
            EVENT_TRACE_GUID,
            GUID::from("68fdd900-4a3e-11d1-84f4-0000f80464e3")
        );
        assert_eq!(
            MMCSS_TRACE_GUID,
            GUID::from("f8f10121-b617-4a56-868b-9df1b27fe32c")
        );
        assert_eq!(
            SYSTEM_TRACE_GUID,
            GUID::from("9e814aad-3204-11d2-9a82-006008a86939")
        );
        assert_eq!(
            EVENT_TRACE_CONFIG_GUID,
            GUID::from("01853a65-418f-4f36-aefc-dc0f1d2fd235")
        );
    }
}
