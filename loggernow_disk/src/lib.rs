use futures::TryStreamExt;
use heim::{disk, units};
use loggernow_common;
use std::ffi::OsStr;
use tokio_stream::StreamExt;

///get physical parttions
pub async fn get_physical_partitions() -> loggernow_common::PhysicalPartitionList {
    let mut parttion_vector = Vec::new();

    // println!(
    //     "{:<17} {:<10} {:<10} {:<10} {:<10} Mount",
    //     "Device", "Total, Mb", "Used, Mb", "Free, Mb", "Type"
    // );

    let partitions = disk::partitions_physical().await;
    let partitions = partitions.unwrap(); //adderd
    futures::pin_mut!(partitions);

    while let Some(part) = partitions.next().await {
        let part = part.unwrap();
        let usage = part.usage().await;
        let usage = usage.unwrap();
        // println!(
        //     "{:<17} {:<10} {:<10} {:<10} {:<10} {}",
        //     part.device()
        //         .unwrap_or_else(|| OsStr::new("N/A"))
        //         .to_string_lossy(),
        //     usage.total().get::<units::information::megabyte>(),
        //     usage.used().get::<units::information::megabyte>(),
        //     usage.free().get::<units::information::megabyte>(),
        //     part.file_system().as_str(),
        //     part.mount_point().to_string_lossy(),
        // );
        let partition = loggernow_common::PhysicalPartition {
            device: part
                .device()
                .unwrap_or_else(|| OsStr::new("N/A"))
                .to_string_lossy()
                .to_string(),
            total: usage
                .total()
                .get::<units::information::megabyte>()
                .to_string(),
            used: usage
                .used()
                .get::<units::information::megabyte>()
                .to_string(),
            free: usage
                .free()
                .get::<units::information::megabyte>()
                .to_string(),
            partition_type: part.file_system().as_str().to_string(),
            mount_path: part.mount_point().to_string_lossy().to_string(),
        };
        parttion_vector.push(partition);
    }
    let PhysicalPartitionList = loggernow_common::PhysicalPartitionList {
        list_of_parttions: parttion_vector,
    };
    PhysicalPartitionList
}


