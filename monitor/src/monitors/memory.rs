use std::convert::TryInto;

use systemstat::{saturating_sub_bytes, ByteSize, Platform};

pub fn memory_usage() -> (u64, u64, bool) {
    let sys = systemstat::System::new();
    let mut total_memory = 0;
    let mut used_memory = 0;
    let mut err: bool = false;
    match sys.memory() {
        Ok(mem) => {
            // println!(
            //     "\nMemory: {} used / {} ({} bytes) total ({:?})",
            //     saturating_sub_bytes(mem.total, mem.free),
            //     mem.total,
            //     mem.total.as_u64(),
            //     mem.platform_memory
            // );
            total_memory = mem.total.as_u64();
            used_memory = mem.total.as_u64() - mem.free.as_u64();
        }
        Err(x) => {
            err = true;
        }
    }
    // let percentage_used = (used_memory / total_memory) * 100;
    (used_memory, total_memory, err)
}
