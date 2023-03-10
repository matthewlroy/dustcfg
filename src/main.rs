use dustcfg::write_api_endpoints_to_json_file;

fn main() -> std::io::Result<()> {
    write_api_endpoints_to_json_file()?;

    println!("Successfully wrote API endpoints to JSON file");

    Ok(())
}
