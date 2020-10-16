use std::fmt;

/// Enum for the architecture in a target triple
/// (Not all possible architectures listed, will be in the future)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Architecture
{
    X86_64,
    I686,
    Aarch64,
    Riscv64gc,
    Thumbv7em
}

/// Enum for the vendor in a target triple
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vendor
{
    Unknown,
    None
}

/// Enum for the operating system in a target triple
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperatingSystem
{
    None,
    Linux,
    Elf
}

/// Struct containing a target triple
#[derive(Debug, Clone, Copy)]
pub struct TargetTriple
{
    arch: Architecture,
    vendor: Vendor,
    os: OperatingSystem
}

impl TargetTriple
{
    pub fn new(arch: Architecture, vendor: Vendor, os: OperatingSystem) -> Self
    {
        Self
        {
            arch,
            vendor,
            os
        }
    }
}

impl fmt::Display for TargetTriple
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", format!("{:?}-{:?}-{:?}", self.arch, self.vendor, self.os).to_ascii_lowercase())
    }
}