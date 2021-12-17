use actix_web::http::header::IntoHeaderValue;
use run_script::ScriptOptions;

pub fn check_service() -> (i32, String) {
    let mut return_msg: String = "".to_string();
    let mut return_int: i32 = -1;

    let options = ScriptOptions::new();

    let args = vec![];
    let (code, output, error) = run_script::run(
        r#"systemctl is-active --quiet '+target+' 2>/dev/null
        "#,
        &args,
        &options,
    )
    .unwrap();

    let output_int: i32 = match output.parse() {
        Ok(value) => value,
        Err(err) => 2,
    };
    if output_int == 0 {
        return_msg = format!("Service is running fine");
        return_int = 0;
    } else if output_int > 0 {
        return_msg = format!("{} intsances of service detected", &output_int);
        return_int = 1;
    } else {
        return_msg = format!("Unexpected error occured while checking the service");
        return_int = 2;
    }

    (return_int, return_msg)
}
