// Copyright 2024 RustFS Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![allow(dead_code)]

use base64_simd::STANDARD;
use std::error::Error;

#[derive(Debug)]
pub(crate) struct DecodeError(base64_simd::Error);

impl Error for DecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to decode base64")
    }
}

pub(crate) fn decode(input: impl AsRef<str>) -> Result<Vec<u8>, DecodeError> {
    STANDARD.decode_to_vec(input.as_ref()).map_err(DecodeError)
}

pub(crate) fn encode(input: impl AsRef<[u8]>) -> String {
    STANDARD.encode_to_string(input.as_ref())
}

pub(crate) fn encoded_length(length: usize) -> usize {
    STANDARD.encoded_length(length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_empty() {
        assert_eq!(encode(b""), "");
    }

    #[test]
    fn test_roundtrip() {
        let data = b"hello, rustfs!";
        let encoded = encode(data);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_encode_known_value() {
        // "Hello" -> "SGVsbG8="
        let encoded = encode(b"Hello");
        assert_eq!(encoded, "SGVsbG8=");
    }

    #[test]
    fn test_decode_invalid_input() {
        assert!(decode("!!!not-base64!!!").is_err());
    }

    #[test]
    fn test_encoded_length_matches_actual() {
        for len in [0, 1, 5, 16, 100] {
            let data = vec![0xAB_u8; len];
            let actual_len = encode(&data).len();
            assert_eq!(encoded_length(len), actual_len, "mismatch for length {len}");
        }
    }
}
