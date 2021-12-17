use std::convert::TryInto;

use systemstat::{saturating_sub_bytes, ByteSize, Platform};

pub fn disk_capacity_usage() -> Vec<(u64, String)> {
    let sys = systemstat::System::new();

    let mut percent_free: Vec<(u64, String)> = Vec::new();

    match sys.mounts() {
        Ok(mounts) => {
            println!("\nMounts:");
            for mount in mounts.iter() {
                // println!(
                //     "{} ---{}---> {} (available {} of {})",
                //     mount.fs_mounted_from,
                //     mount.fs_type,
                //     mount.fs_mounted_on,
                //     mount.avail,
                //     mount.total
                // );

                let temnp_total_size = mount.total.as_u64();
                let temp_free_size = mount.avail.as_u64();
                let mut temp_percent_free = match (temp_free_size / temnp_total_size).try_into() {
                    Ok(value) => value,
                    Err(e) => 0,
                };
                temp_percent_free = temp_percent_free * 100;
                let mounted_on = mount.fs_mounted_on.clone();

                percent_free.push((temp_percent_free, mounted_on));
            }
        }
        Err(x) => println!("\nMounts: error: {}", x),
    }

    percent_free
}
