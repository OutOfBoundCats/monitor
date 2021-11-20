use heim::{
    cpu::{self, CpuFrequency},
    host, units,
};
use loggernow_common;

///get thread count of running instance
pub async fn get_logical_cpu_cores() -> String {
    let cpu_count = cpu::logical_count().await;
    let logical_count = cpu_count.unwrap();
    logical_count.to_string()
}

///get core count of running instance
pub async fn get_physical_cpu_cores() -> String {
    let cpu_count = cpu::physical_count().await;
    let physical_count = cpu_count.unwrap();
    physical_count.unwrap().to_string()
}

///get cpu frequncy of running machine
pub async fn get_cpu_frequncy() -> String {
    let frequency = cpu::frequency().await;

    frequency
        .unwrap()
        .current()
        .get::<units::frequency::megahertz>()
        .to_string()
}

///get max cpu frequncy of running machine
pub async fn get_max_cpu_frequncy() -> String {
    let local = cpu::frequency().await;
    local
        .unwrap()
        .max()
        .unwrap()
        .get::<units::frequency::megahertz>()
        .to_string()
}

pub async fn get_percentage_cpu_usage() -> u64 {
    let frequency = cpu::frequency().await;
    let max = frequency
        .unwrap()
        .max()
        .unwrap()
        .get::<units::frequency::megahertz>();

    let frequency1 = cpu::frequency().await;
    let current = frequency1
        .unwrap()
        .current()
        .get::<units::frequency::megahertz>();
    let percentage = (max - current / max) * 100;
    percentage
}

///get system info of machine
pub async fn get_sys_info() -> loggernow_common::system_info {
    let platform = host::platform().await.unwrap();

    let system_info = loggernow_common::system_info {
        system: platform.system().to_string(),
        release: platform.release().to_string(),
        hostname: platform.hostname().to_string(),
        version: platform.version().to_string(),
        architecture: platform.architecture().as_str().to_string(),
    };
    system_info
}
