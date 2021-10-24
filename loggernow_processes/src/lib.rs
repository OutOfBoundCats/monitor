use futures::TryStreamExt;
use loggernow_common;
use std::{convert::TryInto, fmt, ops::Add, thread, time};
use sysinfo::{ProcessExt, Signal, System, SystemExt};

pub async fn get_all_process_info() -> loggernow_common::AllProcessesData {
    let mut system = sysinfo::System::new_all();
    let mut local_list_of_processes = Vec::new();

    system.refresh_all();

    for (pid, proc_) in system.get_processes() {
        let cmd = proc_.cmd();
        let mut cmd_arg = String::new();
        for value in cmd.iter() {
            //println!(" teh iter is {}", value);
            //&mut cmd_arg.add(value);
            cmd_arg.push_str(value)
        }

        let mut parent_id: i32 = 0;
        match proc_.parent() {
            Some(v) => parent_id = v,
            None => (),
        }

        let process = loggernow_common::ProcessesData {
            pid: pid.to_string(),
            parent_id: parent_id.to_string(),
            process_name: proc_.name().to_string(),
            process_status: {
                match proc_.status() {
                    sysinfo::ProcessStatus::Idle => "Idle".to_string(),
                    sysinfo::ProcessStatus::Run => "Run".to_string(),
                    sysinfo::ProcessStatus::Sleep => "Sleep".to_string(),
                    sysinfo::ProcessStatus::Stop => "Stop".to_string(),
                    sysinfo::ProcessStatus::Zombie => "Zombie".to_string(),
                    sysinfo::ProcessStatus::Tracing => "Tracing".to_string(),
                    sysinfo::ProcessStatus::Dead => "Dead".to_string(),
                    sysinfo::ProcessStatus::Wakekill => "Wakekill".to_string(),
                    sysinfo::ProcessStatus::Waking => "Waking".to_string(),
                    sysinfo::ProcessStatus::Parked => "Parked".to_string(),
                    sysinfo::ProcessStatus::Unknown(u32) => "Idle".to_string(),
                }
            },
            command_line_arg: cmd_arg,
            proces_path: proc_.exe().to_string_lossy().to_string(),
            process_memory: proc_.memory().to_string(),
            process_virtual_memory: proc_.virtual_memory().to_string(),
            process_start_time: (proc_.start_time() / 60).to_string(),
            process_cpu_usage: proc_.cpu_usage().to_string(),
            disk_usage_read_bytes: proc_.disk_usage().read_bytes.to_string(),
            disk_usage_total_read_bytes: proc_.disk_usage().total_read_bytes.to_string(),
            disk_usage_write_bytes: proc_.disk_usage().written_bytes.to_string(),
            disk_usage_total_write_bytes: proc_.disk_usage().total_written_bytes.to_string(),
        };
        local_list_of_processes.push(process);
    }
    let all_process_data = loggernow_common::AllProcessesData {
        list_of_processes: local_list_of_processes,
    };
    all_process_data
}
