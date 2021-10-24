use futures;
use language_runner;
use loggernow_common;
use loggernow_cpu;
use loggernow_disk;
use loggernow_memory;
use loggernow_processes;

pub async fn get_cpu_data() -> loggernow_common::cpu_info {
    let virtual_threads = loggernow_cpu::cpu::get_logical_cpu_cores().await;
    //println!("thread count from master lib is {}",thread_count);
    let physical_cores = loggernow_cpu::cpu::get_physical_cpu_cores().await;
    let cpu_frequency = loggernow_cpu::cpu::get_cpu_frequncy().await;
    let cpu_info = loggernow_common::cpu_info {
        virtual_threads,
        physical_cores,
        cpu_frequency,
    };
    cpu_info
}

pub async fn get_physical_partitions() -> loggernow_common::PhysicalPartitionList {
    let partitions = loggernow_disk::get_physical_partitions().await;
    partitions
}

pub async fn get_system_info() -> loggernow_common::system_info {
    let sys_info = loggernow_cpu::cpu::get_sys_info().await;
    sys_info
}

pub async fn get_memory_data() -> loggernow_common::memory_data {
    let memory = loggernow_memory::get_memory().await;
    memory
}

pub async fn get_processes_data() -> loggernow_common::AllProcessesData {
    let all_process_data = loggernow_processes::get_all_process_info().await;
    all_process_data
}

pub async fn get_god_father_date() -> loggernow_common::Godfather {
    let all_processes_data = get_processes_data();
    let system_info = get_system_info();
    let cpu_info = get_cpu_data();
    let memory_data = get_memory_data();
    let physical_partition_list = get_physical_partitions();

    let (all_processes_data, system_info, cpu_info, memory_data, physical_partition_list) = futures::join!(
        all_processes_data,
        system_info,
        cpu_info,
        memory_data,
        physical_partition_list
    );
    let god = loggernow_common::Godfather {
        all_processes_data,
        system_info,
        cpu_info,
        memory_data,
        physical_partition_list,
    };
    god
}

pub fn run() {
    language_runner::run_java();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
