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

use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapacityDiskRef {
    pub endpoint: String,
    pub drive_path: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct CapacityScanResult {
    pub used_bytes: u64,
    pub file_count: usize,
    pub sampled_count: usize,
    pub is_estimated: bool,
    pub scan_duration: Duration,
    pub had_partial_errors: bool,
}

impl CapacityScanResult {
    pub(crate) fn with_partial_errors(mut self) -> Self {
        self.had_partial_errors = true;
        self
    }
}

/// Public summary type for external tooling such as benches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapacityScanSummary {
    pub used_bytes: u64,
    pub file_count: usize,
    pub sampled_count: usize,
    pub is_estimated: bool,
    pub had_partial_errors: bool,
    pub scan_duration: Duration,
}

impl From<CapacityScanResult> for CapacityScanSummary {
    fn from(scan: CapacityScanResult) -> Self {
        Self {
            used_bytes: scan.used_bytes,
            file_count: scan.file_count,
            sampled_count: scan.sampled_count,
            is_estimated: scan.is_estimated,
            had_partial_errors: scan.had_partial_errors,
            scan_duration: scan.scan_duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capacity_disk_ref_equality() {
        let a = CapacityDiskRef {
            endpoint: "ep1".to_string(),
            drive_path: "/drive1".to_string(),
        };
        let b = CapacityDiskRef {
            endpoint: "ep1".to_string(),
            drive_path: "/drive1".to_string(),
        };
        assert_eq!(a, b);

        let c = CapacityDiskRef {
            endpoint: "ep2".to_string(),
            drive_path: "/drive1".to_string(),
        };
        assert_ne!(a, c);
    }

    #[test]
    fn test_capacity_scan_result_with_partial_errors() {
        let result = CapacityScanResult {
            used_bytes: 1024,
            file_count: 10,
            sampled_count: 0,
            is_estimated: false,
            scan_duration: Duration::from_millis(50),
            had_partial_errors: false,
        };
        assert!(!result.had_partial_errors);

        let modified = result.with_partial_errors();
        assert!(modified.had_partial_errors);
        assert_eq!(modified.used_bytes, 1024);
        assert_eq!(modified.file_count, 10);
    }

    #[test]
    fn test_capacity_scan_result_into_summary() {
        let result = CapacityScanResult {
            used_bytes: 2048,
            file_count: 42,
            sampled_count: 5,
            is_estimated: true,
            scan_duration: Duration::from_millis(100),
            had_partial_errors: true,
        };
        let summary: CapacityScanSummary = result.into();
        assert_eq!(summary.used_bytes, 2048);
        assert_eq!(summary.file_count, 42);
        assert_eq!(summary.sampled_count, 5);
        assert!(summary.is_estimated);
        assert!(summary.had_partial_errors);
        assert_eq!(summary.scan_duration, Duration::from_millis(100));
    }

    #[test]
    fn test_capacity_scan_result_default() {
        let result = CapacityScanResult::default();
        assert_eq!(result.used_bytes, 0);
        assert_eq!(result.file_count, 0);
        assert_eq!(result.sampled_count, 0);
        assert!(!result.is_estimated);
        assert_eq!(result.scan_duration, Duration::ZERO);
        assert!(!result.had_partial_errors);
    }
}
