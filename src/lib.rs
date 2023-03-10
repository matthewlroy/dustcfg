use rand::Rng;
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

/// The procedure to generate a version 4 UUID is as follows:
///
/// >> In RFC Technical Terms:
/// >> https://www.rfc-editor.org/rfc/rfc4122#page-14
///
/// 4.4. Algorithms for Creating a UUID from Truly Random or Pseudo-Random
/// Numbers. The version 4 UUID is meant for generating UUIDs from truly-random
/// or pseudo-random numbers. The algorithm is as follows:
///
/// o Set the two most significant bits (bits 6 and 7) of the
///     clock_seq_hi_and_reserved to zero and one, respectively.
/// o Set the four most significant bits (bits 12 through 15) of the
///     time_hi_and_version field to the 4-bit version number from Section 4.1.3
/// o Set all the other bits to randomly (or pseudo-randomly) chosen values.
///
/// >> In Plain Language Terms:
/// >> https://www.cryptosys.net/pki/uuid-rfc4122.html
///
/// 1. Generate 16 random bytes (=128 bits)
/// 2. Adjust certain bits according to RFC 4122 section 4.4 as follows:
///     a. set the four most significant bits of the 7th byte to 0100'B, so the
///         high nibble is "4"
///     b. set the two most significant bits of the 9th byte to 10'B, so the
///         high nibble will be one of "8", "9", "A", or "B" (see Note 1).
/// 3. Encode the adjusted bytes as 32 hexadecimal digits
/// 4. Add four hyphen "-" characters to obtain blocks of 8, 4, 4, 4 and 12 hex
///     digits
/// 5. Output the resulting 36-character string
///     "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"
pub fn generate_v4_uuid() -> String {
    let mut uuid_v4 = String::new();

    // 1. Generate 16 random bytes (=128 bits)
    let mut rng = rand::thread_rng();
    for x in 0..16 {
        let mut n = rng.gen::<u8>();

        // 2a. set the four most significant bits of the 7th byte to 0100'B, so
        // the high nibble is "4"
        if x == 6 {
            let first_and = 0b00001111u8;
            let second_or = 0b01000000u8;
            n = (n & first_and) | second_or;
        }

        // 2b. set the two most significant bits of the 9th byte to 10'B, so the
        // high nibble will be one of "8", "9", "A", or "B" (see Note 1).
        if x == 8 {
            let first_and = 0b00111111u8;
            let second_or = 0b10000000u8;
            n = (n & first_and) | second_or;
        }

        // 4. Add four hyphen "-" characters to obtain blocks of 8, 4, 4, 4 and
        // 12 hex digits
        if uuid_v4.len() == 8 {
            uuid_v4.push('-');
        }

        if uuid_v4.len() == 8 + 4 + 1 {
            uuid_v4.push('-');
        }

        if uuid_v4.len() == 8 + 4 + 4 + 2 {
            uuid_v4.push('-');
        }

        if uuid_v4.len() == 8 + 4 + 4 + 4 + 3 {
            uuid_v4.push('-');
        }

        uuid_v4.push_str(&format!("{:02x}", n));

        // println!(
        //     "Index [{}]:\t{:#010b}\t(Byte #{})\t=>\t{}\t=>\t{:02x}",
        //     x,
        //     n,
        //     x + 1,
        //     n,
        //     n
        // );
    }

    // 5. Output the resulting 36-character string
    // "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"
    uuid_v4
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::{generate_v4_uuid, get_env_var, write_api_endpoints_to_json_file};
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

    #[test]
    fn test_generate_v4_uuid() {
        let re = Regex::new(
            r"^[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12}$",
        )
        .unwrap();
        assert!(re.is_match(&generate_v4_uuid()));
    }
}
