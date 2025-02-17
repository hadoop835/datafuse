// Copyright 2021 Datafuse Labs.
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

use chrono_tz::Tz;
use common_exception::ErrorCode;

#[derive(Debug, Clone)]
pub struct EvalContext {
    pub factor: i64,
    pub precision: usize,
    pub error: Option<ErrorCode>,
    pub tz: Tz,
}

impl Default for EvalContext {
    fn default() -> Self {
        let tz = "UTC".parse::<Tz>().unwrap();
        Self {
            factor: 1,
            precision: 0,
            error: None,
            tz,
        }
    }
}

impl EvalContext {
    pub fn new(factor: i64, precision: usize, error: Option<ErrorCode>, tz: Tz) -> Self {
        Self {
            factor,
            precision,
            error,
            tz,
        }
    }

    pub fn set_error(&mut self, e: ErrorCode) {
        if self.error.is_none() {
            self.error = Some(e);
        }
    }
}
