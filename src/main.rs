use dustcfg::write_api_endpoints_to_json_file;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process::Command;

fn main() -> std::io::Result<()> {
    // WRITE ENDPOINTS to FE
    write_api_endpoints_to_json_file()?;
    println!("Successfully wrote API endpoints to JSON file");

    setup_services()?;
    println!("Successfully set up services in systemd [dustweb, dustdb]!\n\n* * * * * * * * \nStarted app!\n* * * * * * * *\n");

    Ok(())
}

fn setup_services() -> std::io::Result<()> {
    for service_name in ["dustweb", "dustdb"] {
        // Install the respective cargo binaries!
        println!("This will take a few minutes . . . installing cargo release for: {}", &service_name);
        Command::new("cargo")
            .arg("install")
            .arg("--path")
            .arg(format!("/dust/{}/.", &service_name))
            .output()?;
        println!("SUCCESS! cargo release for: {}", &service_name);

        // Write (and replaces) service files in systemd
        Command::new("cp")
            .arg(format!("/dust/dustcfg/{}.service", &service_name))
            .arg("/etc/systemd/system/.")
            .output()?;
        println!("Copied service: {}", &service_name);

        // Remove any configuration folders (if exist)
        Command::new("rm")
            .arg("-rf")
            .arg(format!("/etc/systemd/system/{}.service.d/", &service_name))
            .output()?;
        println!("Deleted original config folder: {}", &service_name);

        // Re-create config folders
        Command::new("mkdir")
            .arg(format!("/etc/systemd/system/{}.service.d/", &service_name))
            .output()?;
        println!("Created new config folder: {}", &service_name);

        // Write new environment variable files
        let mut out_file = File::create(format!(
            "/etc/systemd/system/{}.service.d/{}.conf",
            &service_name, &service_name
        ))?;
        out_file.write(b"[Service]\n")?;

        let in_file = File::open("dust.settings")?;
        let reader = BufReader::new(in_file);
        for line in reader.lines() {
            out_file.write(format!("{}\n", line?.replace("export ", "Environment=")).as_bytes())?;
        }
        out_file.flush()?;
        println!("Created sytem config file for: {}", &service_name);
    }

    // Reset systemctl daemons
    Command::new("systemctl").arg("daemon-reload").output()?;
    println!("Reset systemctl daemons!");

    // Enable new services and then start!
    for service_name in ["dustweb", "dustdb"] {
        Command::new("systemctl")
            .arg("enable")
            .arg(&service_name)
            .output()?;
        println!("Enabled service for: {}", &service_name);

        Command::new("systemctl")
            .arg("start")
            .arg(&service_name)
            .output()?;
        println!("Started service for: {}", &service_name);
    }

    Ok(())
}
