use std::env;
use walkdir::WalkDir;
use std::fs;
use serde_json::Value;
use reqwest::get;
use semver::Version;
use csv::Writer;
use petgraph::Graph;
use petgraph::dot::{ Dot, Config };
use walkdir::DirEntry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Please provide the path to the directory as an argument.");
        std::process::exit(1);
    }

    let path = &args[1];
    let mut writer = Writer::from_path("report.csv")?;

    // Write headers to the CSV file
    writer.write_record(&["File Path", "Dependency", "Current Version", "Latest Version"])?;

    // Initialize a new empty directed graph
    let mut graph = Graph::<String, &str>::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e)) {
        let entry = entry?;
        if entry.file_name().to_string_lossy() == "package.json" {
            let file = fs::File::open(entry.path())?;
            let json: Value = serde_json::from_reader(file)?;
            let package_name = json["name"].as_str().unwrap_or("").to_string(); // Cloned here
            let dependencies = match json.get("dependencies") {
                Some(d) =>
                    match d.as_object() {
                        Some(o) => o,
                        None => {
                            continue;
                        } // Continue to next file if "dependencies" is not an object
                    }
                None => {
                    continue;
                } // Continue to next file if "dependencies" does not exist
            };

            // Create a node for this package
            let package_node = graph.add_node(package_name);

            for (dep, version) in dependencies {
                let dep_clone = dep.clone(); // Clone `dep` here
                let current_version = version.as_str().unwrap();
                let current_version_replaced = current_version.replace("^", "").replace("~", "");
                let current_version_str = current_version_replaced
                    .split_whitespace()
                    .next()
                    .unwrap_or("");

                if current_version_str == "*" {
                    continue; // Skip this dependency if the version is "*"
                }

                let url = format!("https://registry.npmjs.org/{}", &dep_clone);
                let response = get(&url).await?.json::<Value>().await?;
                let latest_version = response["dist-tags"]["latest"].as_str().unwrap();
                let latest_version_replaced = latest_version.replace("^", "").replace("~", "");
                let latest_version_str = latest_version_replaced
                    .split_whitespace()
                    .next()
                    .unwrap_or("");

                // Create a node for this dependency and an edge from the package to the dependency
                let dep_node = graph.add_node(dep_clone.clone()); // Create a node for this dependency and hold the `NodeIndex` in `dep_node`
                graph.add_edge(package_node, dep_node, ""); // Link the package to the `dep_node` using its `NodeIndex`

                if Version::parse(latest_version_str)? > Version::parse(current_version_str)? {
                    writer.write_record(
                        &[
                            entry.path().to_str().unwrap(),
                            &dep_clone, // Use the clone of `dep_clone` here
                            current_version_str,
                            latest_version_str,
                        ]
                    )?;
                }
            }
        }
    }

    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("node_modules"))
            .unwrap_or(false)
    }

    // Output the graph to a file
    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    fs::write("dependency_graph.dot", format!("{}", dot))?;

    Ok(())
}
