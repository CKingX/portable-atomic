use core::ops;

// Some 64-bit architectures have ABI with 32-bit pointer width (e.g., x86_64 X32 ABI,
// aarch64 ILP32 ABI, mips64 N32 ABI). On those targets, AtomicU64 is available and fast,
// so use it to implement normal sequence lock.
// Known architectures that have such ABI are x86_64, aarch64, and mips64. However,
// we list all 64-bit architectures because similar ABIs may exist for other architectures.
// Script to get the list:
// $ (for target in $(rustc --print target-list); do target_spec=$(rustc --print target-spec-json -Z unstable-options --target "${target}"); [[ "$(jq <<<"${target_spec}" -r '."target-pointer-width"')" == "64" ]] && jq <<<"${target_spec}" -r '.arch'; done) | LC_ALL=C sort -u | sed -E 's/^/    target_arch = "/g; s/$/",/g'
macro_rules! cfg_fast_atomic_64 {
    ($($tt:tt)*) => {
        #[cfg(any(
            not(any(target_pointer_width = "16", target_pointer_width = "32")),
            target_arch = "aarch64",
            target_arch = "bpf",
            target_arch = "mips64",
            target_arch = "nvptx64",
            target_arch = "powerpc64",
            target_arch = "riscv64",
            target_arch = "s390x",
            target_arch = "sparc64",
            target_arch = "wasm64",
            target_arch = "x86_64",
        ))]
        $($tt)*
    };
}
macro_rules! cfg_no_fast_atomic_64 {
    ($($tt:tt)*) => {
        #[cfg(not(any(
            not(any(target_pointer_width = "16", target_pointer_width = "32")),
            target_arch = "aarch64",
            target_arch = "bpf",
            target_arch = "mips64",
            target_arch = "nvptx64",
            target_arch = "powerpc64",
            target_arch = "riscv64",
            target_arch = "s390x",
            target_arch = "sparc64",
            target_arch = "wasm64",
            target_arch = "x86_64",
        )))]
        $($tt)*
    };
}

// Adapted from https://github.com/crossbeam-rs/crossbeam/blob/d49a0f8454499ced8af0b61aeb661379c4eb0588/crossbeam-utils/src/cache_padded.rs.
/// Pads and aligns a value to the length of a cache line.
// Starting from Intel's Sandy Bridge, spatial prefetcher is now pulling pairs of 64-byte cache
// lines at a time, so we have to align to 128 bytes rather than 64.
//
// Sources:
// - https://www.intel.com/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-optimization-manual.pdf
// - https://github.com/facebook/folly/blob/1b5288e6eea6df074758f877c849b6e73bbb9fbb/folly/lang/Align.h#L107
//
// ARM's big.LITTLE architecture has asymmetric cores and "big" cores have 128-byte cache line size.
//
// Sources:
// - https://www.mono-project.com/news/2016/09/12/arm64-icache/
//
// powerpc64 has 128-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_ppc64x.go#L9
#[cfg_attr(
    any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "powerpc64"),
    repr(align(128))
)]
// arm, mips, mips64, riscv64, sparc, and hexagon have 32-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_arm.go#L7
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips.go#L7
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mipsle.go#L7
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_mips64x.go#L9
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_riscv64.go#L7
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L17
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/hexagon/include/asm/cache.h#L12
//
// riscv32 is assumed not to exceed the cache line size of riscv64.
#[cfg_attr(
    any(
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "sparc",
        target_arch = "hexagon",
    ),
    repr(align(32))
)]
// m68k has 16-byte cache line size.
//
// Sources:
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/m68k/include/asm/cache.h#L9
#[cfg_attr(target_arch = "m68k", repr(align(16)))]
// s390x has 256-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_s390x.go#L7
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/s390/include/asm/cache.h#L13
#[cfg_attr(target_arch = "s390x", repr(align(256)))]
// x86, wasm, and sparc64 have 64-byte cache line size.
//
// Sources:
// - https://github.com/golang/go/blob/dda2991c2ea0c5914714469c4defc2562a907230/src/internal/cpu/cpu_x86.go#L9
// - https://github.com/golang/go/blob/3dd58676054223962cd915bb0934d1f9f489d4d2/src/internal/cpu/cpu_wasm.go#L7
// - https://github.com/torvalds/linux/blob/3516bd729358a2a9b090c1905bd2a3fa926e24c6/arch/sparc/include/asm/cache.h#L19
//
// All others are assumed to have 64-byte cache line size.
#[cfg_attr(
    not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64",
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "riscv32",
        target_arch = "riscv64",
        target_arch = "sparc",
        target_arch = "hexagon",
        target_arch = "m68k",
        target_arch = "s390x",
    )),
    repr(align(64))
)]
pub(crate) struct CachePadded<T> {
    value: T,
}

impl<T> CachePadded<T> {
    #[inline]
    pub(crate) const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> ops::Deref for CachePadded<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &self.value
    }
}

// Adapted from https://github.com/crossbeam-rs/crossbeam/blob/crossbeam-utils-0.8.7/crossbeam-utils/src/backoff.rs.
// Adjusted to reduce spinning.
/// Performs exponential backoff in spin loops.
pub(crate) struct Backoff {
    step: u32,
}

// https://github.com/oneapi-src/oneTBB/blob/v2021.5.0/include/oneapi/tbb/detail/_utils.h#L46-L48
const SPIN_LIMIT: u32 = 4;

impl Backoff {
    #[inline]
    pub(crate) const fn new() -> Self {
        Self { step: 0 }
    }

    #[inline]
    pub(crate) fn snooze(&mut self) {
        if self.step <= SPIN_LIMIT {
            for _ in 0..1 << self.step {
                #[allow(deprecated)]
                core::sync::atomic::spin_loop_hint();
            }
            self.step += 1;
        } else {
            #[cfg(not(feature = "std"))]
            for _ in 0..1 << self.step {
                #[allow(deprecated)]
                core::sync::atomic::spin_loop_hint();
            }

            #[cfg(feature = "std")]
            std::thread::yield_now();
        }
    }
}
