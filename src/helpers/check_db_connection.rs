use std::process;

use mongodb::{bson::doc, Client};
use sysinfo::System;

use crate::error::Result;

pub async fn count_connection(client: &Client) -> Result<i32> {
    let db = client.database("admin"); // Connect to the "admin" database

    // Run the serverStatus command
    let result = db
        .run_command(doc! {"serverStatus": 1}, None)
        .await
        .unwrap();

    // Extract the "connections" information
    let connections = result.get("connections").unwrap().as_document().unwrap();

    let num_active_connection = connections.get("active").unwrap().as_i32().unwrap();

    Ok(num_active_connection)
}

pub async fn check_overload(client: &Client) {
    let num_connection = count_connection(client).await.unwrap();

    let mut sys = System::new_all();
    sys.refresh_all();

    // system
    let num_cores = sys.cpus().len();

    // process
    let pid = process::id(); // Get the current process ID

    // Read process statistics from /proc/[pid]/stat
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content = std::fs::read_to_string(stat_path).unwrap();
    let fields: Vec<&str> = stat_content.split_whitespace().collect();

    // Get resident set size (memory usage)
    let rss: u64 = fields[23].parse().unwrap();
    println!("Memory used by process: {} MB", rss * 4096 / 1024 / 1024); // Convert pages to bytes (page size 4KB)

    // example max num of connections based on num CPU Core
    // each core can handle 5 connections
    let max_connections: u64 = num_cores as u64 * 5;

    println!("Active connections: {}", num_connection);

    if num_connection > max_connections as i32 {
        println!("Connection Overload detected!");
    }
}
