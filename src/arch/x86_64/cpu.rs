use raw_cpuid::CpuId;
use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};
use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

pub fn enable_nxe_bit() {
    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

// Enable write protection bits so we can't write into .code and .rodata
pub fn enable_write_protect_bit() {
    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

pub fn print_cpu_info() {
    let cpuid = CpuId::new();

    // CPU Type
    if let Some(info) = cpuid.get_vendor_info() {
        println!("Vendor: {}\n", info.as_string());
    }

    // CPU Specifications
    if let Some(info) = cpuid.get_processor_frequency_info() {
        println!("CPU Base MHz: {}\n", info.processor_base_frequency());
        println!("CPU Base MHz: {}\n", info.processor_max_frequency());
        println!("Bus MHz: {}\n", info.bus_frequency());
    }

    // Cache Specs
    match cpuid.get_cache_parameters() {
        Some(cparams) => {
            for cache in cparams {
                let size = cache.associativity() * cache.physical_line_partitions() * cache.coherency_line_size() * cache.sets();
                println!("L{}-Cache size is {}", cache.level(), size);
            }
        },
        None => println!("No cache parameter information available"),
    }

    // CPU Features
    if let Some(info) = cpuid.get_feature_info() {
        println!("Features:");
        if info.has_fpu() { println!(" - fpu"); };
        if info.has_apic() { println!(" - apic"); };
        if info.has_acpi() { println!(" - acpi"); };
    }

    if let Some(info) = cpuid.get_extended_function_info() {
        if info.has_64bit_mode() { println!(" - 64bit"); };        
    }
}