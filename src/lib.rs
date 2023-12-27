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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn lib_should_panic_when_session_not_in_env() {
        let current_session = env::var("AOC_SESSION");
        env::remove_var("AOC_SESSION");
        session_cookie();
        match current_session {
            Ok(session) => env::set_var("AOC_SESSION", session),
            Err(_) => {}
        }
    }

    #[test]
    fn lib_should_return_session_cookie_when_session_in_env() {
        let current_session = env::var("AOC_SESSION");
        env::set_var("AOC_SESSION", "test");
        assert_eq!("session=test", session_cookie());
        env::set_var("AOC_SESSION", "session=test");
        assert_eq!("session=test", session_cookie());
        match current_session {
            Ok(session) => env::set_var("AOC_SESSION", session),
            Err(_) => {}
        }
    }
}
