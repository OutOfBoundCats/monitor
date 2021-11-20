use actix_web::http::header::IntoHeaderValue;
pub use loggernow_cpu::cpu::get_percentage_cpu_usage;
use sysinfo::{ProcessorExt, System, SystemExt};

pub fn cpu_usage() -> f32 {
    let mut s = System::new_all();
    let mut sum = 0.0;
    let mut count = 0.0;
    for processor in s.processors() {
        sum = sum + processor.cpu_usage();
        count = count + 1.0;
    }
    let average_cpu = sum / count;
    average_cpu
}
