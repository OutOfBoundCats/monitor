use heim::{
    memory::{self, Memory},
    units,
};
use loggernow_common;

pub async fn get_memory() -> loggernow_common::memory_data {
    let memory = memory::memory().await.unwrap();

    let memory = loggernow_common::memory_data {
        total: memory
            .total()
            .get::<units::information::megabyte>()
            .to_string(),
        available: memory
            .available()
            .get::<units::information::megabyte>()
            .to_string(),
        free: memory
            .free()
            .get::<units::information::megabyte>()
            .to_string(),
    };
    //println!("{}", memory);
    memory
}
