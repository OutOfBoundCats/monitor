use serde::Serialize;
use std::fmt;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Serialize)]
pub struct cpu_info {
    pub virtual_threads: String,
    pub physical_cores: String,
    pub cpu_frequency: String,
}
impl fmt::Display for cpu_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Virtual threds  {}, Physical Cores {} , Cpu Frequnecy {}",
            self.virtual_threads, self.physical_cores, self.cpu_frequency
        )
    }
}

#[derive(Debug, Serialize)]
pub struct system_info {
    pub system: String,
    pub release: String,
    pub hostname: String,
    pub version: String,
    pub architecture: String,
}
impl fmt::Display for system_info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "System is  {}, Release name {} , Host Name {} , Version Number {} ,Architecture {}",
            self.system, self.release, self.hostname, self.version, self.architecture
        )
    }
}

#[derive(Debug, Serialize)]
pub struct PhysicalPartition {
    pub device: String,
    pub total: String,
    pub used: String,
    pub free: String,
    pub partition_type: String,
    pub mount_path: String,
}
impl fmt::Display for PhysicalPartition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Device is  {}, Total space {} , Used Space{} , Free Space  {} ,Partition Type {} , Mount Path {}",
            self.device, self.total, self.used, self.free, self.partition_type, self.mount_path
        )
    }
}

#[derive(Debug, Serialize)]
pub struct PhysicalPartitionList {
    pub list_of_parttions: Vec<PhysicalPartition>,
}

pub struct PhysicalPartitions<'a>(pub &'a Vec<PhysicalPartition>);

impl<'a> fmt::Display for PhysicalPartitions<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, PhysicalPartition| {
            result.and_then(|_| writeln!(f, "{}", PhysicalPartition))
        })
    }
}

#[derive(Debug, Serialize)]
pub struct memory_data {
    pub total: String,
    pub available: String,
    pub free: String,
}

impl fmt::Display for memory_data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Total memory {}, Avalable Memory {} , Free Memory {}",
            self.total, self.available, self.free
        )
    }
}

#[derive(Debug, Serialize)]
pub struct ProcessesData {
    pub pid: String,
    pub parent_id: String,
    pub process_name: String,
    pub process_status: String,
    pub command_line_arg: String,
    pub proces_path: String,
    pub process_memory: String,
    pub process_virtual_memory: String,
    pub process_start_time: String,
    pub process_cpu_usage: String,
    pub disk_usage_read_bytes: String,
    pub disk_usage_total_read_bytes: String,
    pub disk_usage_write_bytes: String,
    pub disk_usage_total_write_bytes: String,
}
impl fmt::Display for ProcessesData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PId is  {}, Paretn ID  {} , Process Name  {} , Command Line arg  {} ,
            Process Path {} , Process Memory {} , Virtual Memory {},
            Process Start Time {} , Cpu Usage {} , Disk Usage Read Bytes {} , 
            Dist Usage Total Read Bytes {} ,Disk Usage Write Bytes {} ,Disk Usage Total writes {}",
            self.pid,
            self.parent_id,
            self.process_name,
            self.command_line_arg,
            self.proces_path,
            self.process_memory,
            self.process_virtual_memory,
            self.process_start_time,
            self.process_cpu_usage,
            self.disk_usage_read_bytes,
            self.disk_usage_total_read_bytes,
            self.disk_usage_write_bytes,
            self.disk_usage_total_write_bytes
        )
    }
}

#[derive(Debug, Serialize)]
pub struct AllProcessesData {
    pub list_of_processes: Vec<ProcessesData>,
}

pub struct _AllProcessesData<'a>(pub &'a Vec<ProcessesData>);

impl<'a> fmt::Display for _AllProcessesData<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, process_data| {
            result.and_then(|_| writeln!(f, "{}", process_data))
        })
    }
}
#[derive(Serialize)]
pub struct Godfather {
    pub all_processes_data: AllProcessesData,
    pub system_info: system_info,
    pub cpu_info: cpu_info,
    pub memory_data: memory_data,
    pub physical_partition_list: PhysicalPartitionList,
}

impl fmt::Display for Godfather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "All Process Data {}, System Info  {} , Cpu Info {} , memory data {} ,physical partiition list {}",
            _AllProcessesData(&self.all_processes_data.list_of_processes), self.system_info, self.cpu_info,
            self.memory_data,PhysicalPartitions(&self.physical_partition_list.list_of_parttions),
        )
    }
}
