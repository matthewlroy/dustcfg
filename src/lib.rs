use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    env,
    fs::File,
    io::{self, Write},
    num::ParseIntError,
};

pub fn get_env_var(desired_env_var: &str) -> String {
    match env::var(desired_env_var) {
        Ok(v) => v,
        Err(e) => panic!("${} is not set ({})", desired_env_var, e),
    }
}

#[derive(Serialize, Deserialize)]
pub struct EndpointNames {
    pub health_check: &'static str,
    pub create_user: &'static str,
}

impl EndpointNames {
    const fn new(health_check_api: &'static str, create_user_api: &'static str) -> Self {
        EndpointNames {
            health_check: health_check_api,
            create_user: create_user_api,
        }
    }
}

pub const API_ENDPOINTS: EndpointNames =
    EndpointNames::new("/api/v1/health_check", "/api/v1/create_user");

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

pub fn decode_hex_to_utf8(text_to_decode: &str) -> Result<String, io::Error> {
    let v: Result<Vec<u8>, ParseIntError> = (0..text_to_decode.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text_to_decode[i..i + 2], 16))
        .collect();

    match v.is_ok() {
        true => {
            let v_as_bytes: Vec<u8> = v.unwrap();
            Ok(String::from_utf8_lossy(&v_as_bytes).to_string())
        }
        false => {
            let e_kind = io::ErrorKind::InvalidInput;
            let e = format!("Could not decode: \"{}\", invalid input", text_to_decode).to_owned();
            let error = io::Error::new(e_kind, e);
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::{
        decode_hex_to_utf8, generate_v4_uuid, get_env_var, write_api_endpoints_to_json_file,
    };
    use std::{io, path::Path};

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

    #[test]
    fn test_decode_hex_to_utf8() {
        assert_eq!("z", decode_hex_to_utf8("7A").unwrap());
        assert_eq!("�", decode_hex_to_utf8("AA").unwrap());
        assert_eq!(
            "{\"name\":\"John\", \"age\":30, \"car\":null}",
            decode_hex_to_utf8(
                "7B226E616D65223A224A6F686E222C2022616765223A33302C2022636172223A6E756C6C7D"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_decode_hex_to_utf8_should_error() {
        let result = decode_hex_to_utf8("testy").map_err(|e| e.kind());
        let expected = Err(io::ErrorKind::InvalidInput);
        assert_eq!(expected, result);
    }
}
