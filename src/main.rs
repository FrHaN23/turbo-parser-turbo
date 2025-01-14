use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::{self, BufRead, BufReader},
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
    let filter: &str = "miss";
    let args: Vec<String> = env::args().collect();

    let json_data = if args.len() > 1 {
        let file = fs::File::open(&args[1])?;
        let reader = BufReader::new(file);

        let json_data: String = reader
            .lines()
            .skip_while(|line| match line {
                Ok(line) => !line.contains('{'),
                Err(_) => true,
            })
            .collect::<Result<String, _>>()?;

        json_data
    } else {
        let stdin = io::stdin();
        let reader = stdin.lock().lines();

        let mut buffer = String::new();
        for line in reader.skip_while(|line| match line {
            Ok(line) => !line.contains('{'),
            Err(_) => true,
        }) {
            buffer.push_str(&line?);
        }

        buffer
    };

    let parsed_json: InputJson = serde_json::from_str(&json_data)?;
    let filtered_apps: Vec<String> = parsed_json
        .tasks
        .iter()
        .filter(|task| {
            task.cache.status.to_lowercase() == filter && !task.package.starts_with("@package/")
        })
        .map(|task| task.package.clone())
        .collect();

    let output_json = OutputJson {
        apps: filtered_apps,
    };

    let output_json_string = serde_json::to_string_pretty(&output_json)?;
    println!("{}", output_json_string);

    fs::write("apps_changes.json", output_json_string)?;

    Ok(())
}
