//  Copyright 2021 Datafuse Labs.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//

use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct RpcClientTlsConfig {
    pub rpc_tls_server_root_ca_cert: String,
    pub domain_name: String,
}

impl RpcClientTlsConfig {
    pub fn enabled(&self) -> bool {
        !self.rpc_tls_server_root_ca_cert.is_empty() && !self.domain_name.is_empty()
    }
}

#[derive(Clone, Debug, Default)]
pub struct RpcClientConf {
    pub address: String,
    pub endpoints: Vec<String>,
    pub username: String,
    pub password: String,
    pub tls_conf: Option<RpcClientTlsConfig>,

    /// Timeout for an RPC
    pub timeout: Option<Duration>,
}

impl RpcClientConf {
    /// Whether a remote metasrv is specified.
    ///
    /// - `address` is an old config that accept only one address.
    /// - `endpoints` accepts multiple endpoint candidates.
    ///
    /// If either of these two is configured(non-empty), use remote metasrv.
    /// Otherwise, use a local embedded meta
    pub fn local_mode(&self) -> bool {
        self.address.is_empty() && self.endpoints.is_empty()
    }

    /// Returns a list of endpoints.
    ///
    /// It is compatible with the old single `address` config, by converting it to a vec.
    pub fn get_endpoints(&self) -> Vec<String> {
        if !self.endpoints.is_empty() {
            self.endpoints.clone()
        } else {
            let addr = self.address.to_string();
            vec![addr]
        }
    }
}
