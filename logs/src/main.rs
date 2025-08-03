use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() -> Result<(), Error> {
    // Try operator behaves like do notation: unwrap Ok or return Error
    // let text = fs::read_to_string("logs.txt")?;
    // let error_logs = extract_errors(&text);
    // fs::write("errors.txt", error_logs.join("\n"))?;
    //
    // Ok(())

    fs::read_to_string("logs.txt")
        .map(|text| extract_errors(&text))
        .and_then(|error_logs| fs::write("errors.txt", error_logs.join("\n")))

    // let mut error_logs = vec![];
    // let text = fs::read_to_string("logs.txt");
    // match text {
    //     Ok(str_read) => {
    //         error_logs = extract_errors(&str_read);
    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(_) => println!("File written successfully"),
    //             Err(why_failed) => println!("Error in writing file: {}", why_failed),
    //         }
    //     }
    //     Err(why_failed) => {
    //         println!("Error in reading file: {}", why_failed);
    //     }
    // };
    // println!("{:?}", error_logs);

}
