// This file is @generated by portable-atomic-internal-codegen
// (gen function at tools/codegen/src/ffi.rs).
// It is not intended for manual editing.

#![cfg_attr(rustfmt, rustfmt::skip)]
#![allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    unreachable_pub,
    unused_imports,
    clippy::unreadable_literal,
)]
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "gnu",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
mod aarch64_linux_gnu;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "gnu",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
pub use aarch64_linux_gnu::*;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "gnu",
        target_endian = "big",
        target_pointer_width = "64"
    )
)]
mod aarch64_be_linux_gnu;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "gnu",
        target_endian = "big",
        target_pointer_width = "64"
    )
)]
pub use aarch64_be_linux_gnu::*;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "gnu",
        target_endian = "little",
        target_pointer_width = "32"
    )
)]
mod aarch64_linux_gnu_ilp32;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "gnu",
        target_endian = "little",
        target_pointer_width = "32"
    )
)]
pub use aarch64_linux_gnu_ilp32::*;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "gnu",
        target_endian = "big",
        target_pointer_width = "32"
    )
)]
mod aarch64_be_linux_gnu_ilp32;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "gnu",
        target_endian = "big",
        target_pointer_width = "32"
    )
)]
pub use aarch64_be_linux_gnu_ilp32::*;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "musl",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
mod aarch64_linux_musl;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "musl",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
pub use aarch64_linux_musl::*;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "uclibc",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
mod aarch64_linux_uclibc;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "linux",
        target_env = "uclibc",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
pub use aarch64_linux_uclibc::*;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "android",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
mod aarch64_linux_android;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "android",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
pub use aarch64_linux_android::*;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "macos",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
mod aarch64_apple_darwin;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "macos",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
pub use aarch64_apple_darwin::*;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "freebsd",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
mod aarch64_freebsd;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "freebsd",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
pub use aarch64_freebsd::*;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "openbsd",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
mod aarch64_openbsd;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "openbsd",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
pub use aarch64_openbsd::*;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "fuchsia",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
mod aarch64_fuchsia;
#[cfg(
    all(
        target_arch = "aarch64",
        target_os = "fuchsia",
        target_endian = "little",
        target_pointer_width = "64"
    )
)]
pub use aarch64_fuchsia::*;
