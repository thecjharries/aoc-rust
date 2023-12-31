// Copyright 2023 CJ Harries
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

use std::env;

pub fn session_cookie() -> String {
    let session = env::var("AOC_SESSION").expect("AOC_SESSION not in environment");
    let session = session.trim_start_matches("session=");
    format!("session={}", session)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    // serial_test doesn't play well with tarpaulin
    // You might still need to use cargo test -- --test-threads=1
    use serial_test::serial;

    #[test]
    #[serial]
    fn lib_should_error_when_session_not_in_env() {
        let current_session = match env::var("AOC_SESSION") {
            Ok(session) => Some(session),
            Err(_) => None,
        };
        env::remove_var("AOC_SESSION");
        assert!(session_cookie().is_err());
        match current_session {
            Some(session) => env::set_var("AOC_SESSION", session),
            None => {}
        }
    }

    #[test]
    #[serial]
    fn lib_should_return_session_cookie_when_session_in_env() {
        let current_session = match env::var("AOC_SESSION") {
            Ok(session) => Some(session),
            Err(_) => None,
        };
        env::set_var("AOC_SESSION", "test");
        assert_eq!("session=test", session_cookie().unwrap());
        env::set_var("AOC_SESSION", "session=test");
        assert_eq!("session=test", session_cookie().unwrap());
        match current_session {
            Some(session) => env::set_var("AOC_SESSION", session),
            None => env::remove_var("AOC_SESSION"),
        }
    }
}
