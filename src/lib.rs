use serde::{Deserialize, Serialize};
use serde_json;
use std::{env, fs::File, io::Write};

pub fn get_env_var(desired_env_var: &str) -> String {
    match env::var(desired_env_var) {
        Ok(v) => v,
        Err(e) => panic!("${} is not set ({})", desired_env_var, e),
    }
}

#[derive(Serialize, Deserialize)]
pub struct EndpointNames {
    pub health_check: &'static str,
}

impl EndpointNames {
    const fn new(endpoint: &'static str) -> Self {
        EndpointNames {
            health_check: endpoint,
        }
    }
}

pub const API_ENDPOINTS: EndpointNames = EndpointNames::new("/api/v1/health_check");

pub fn write_api_endpoints_to_json_file() -> std::io::Result<()> {
    let mut f = File::create(format!(
        "{}{}",
        get_env_var("DUST_CHAT_PATH"),
        "endpoints_v1.json"
    ))?;

    f.write_all(serde_json::to_string(&API_ENDPOINTS).unwrap().as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_env_var, write_api_endpoints_to_json_file};
    use std::path::Path;

    #[test]
    #[should_panic]
    fn test_get_env_var_panic() {
        get_env_var("desired_env_var");
    }

    #[test]
    fn test_get_env_var_dust_root() {
        assert_eq!("3000", get_env_var("DUST_SERVER_PORT"));
    }

    #[test]
    fn test_write_api_endpoints_to_json_file() {
        let _ = write_api_endpoints_to_json_file();

        let path = format!("{}{}", get_env_var("DUST_CHAT_PATH"), "endpoints_v1.json");

        assert_eq!(true, Path::new(&path).exists());
    }
}
