#![no_std]
#![no_main]

use aya_bpf::{
    macros::tracepoint,
    programs::TracePointContext,
};
use aya_log_ebpf::info;

#[tracepoint(name = "aya_attacher")]
pub fn aya_attacher(ctx: TracePointContext) -> u32 {
    match try_aya_attacher(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_aya_attacher(ctx: TracePointContext) -> Result<u32, u32> {
    info!(&ctx, "tracepoint sched_switch called");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
