use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::{self, Read},
};

#[derive(Debug, Deserialize)]
struct Task {
    package: String,
    cache: Cache,
}

#[derive(Debug, Deserialize)]
struct Cache {
    status: String,
}

#[derive(Debug, Deserialize)]
struct InputJson {
    tasks: Vec<Task>,
}

#[derive(Debug, Serialize)]
struct OutputJson {
    apps: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter: &str = "hit";
    let args: Vec<String> = env::args().collect();
    let json_data = if args.len() > 1 {
        fs::read_to_string(&args[1])?
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    let parsed_json: InputJson = serde_json::from_str(&json_data)?;

    let filtered_apps: Vec<String> = parsed_json
        .tasks
        .iter()
        .filter(|task| task.cache.status.to_lowercase() == filter)
        .map(|task| task.package.clone())
        .collect();

    let output_json = OutputJson {
        apps: filtered_apps,
    };

    let output_json_string = serde_json::to_string_pretty(&output_json)?;
    println!("{}", output_json_string);
    
    let output_file = "output.json";
    fs::write(output_file, output_json_string)?;

    Ok(())
}
