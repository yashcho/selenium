// Licensed to the Software Freedom Conservancy (SFC) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The SFC licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::config::ManagerConfig;
use reqwest::Client;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::string::ToString;

use crate::files::BrowserPath;

use crate::config::OS::MACOS;
use crate::{create_http_client, format_one_arg, Logger, SeleniumManager, PLIST_COMMAND, STABLE};

pub const SAFARI_NAME: &str = "safari";
pub const SAFARIDRIVER_NAME: &str = "safaridriver";

pub struct SafariManager {
    pub browser_name: &'static str,
    pub driver_name: &'static str,
    pub config: ManagerConfig,
    pub http_client: Client,
    pub log: Logger,
}

impl SafariManager {
    pub fn new() -> Box<Self> {
        let default_config = ManagerConfig::default();
        let default_timeout = default_config.timeout.to_owned();
        let default_proxy = default_config.proxy.to_owned();
        Box::new(SafariManager {
            browser_name: SAFARI_NAME,
            driver_name: SAFARIDRIVER_NAME,
            config: default_config,
            http_client: create_http_client(default_timeout, default_proxy),
            log: Logger::default(),
        })
    }
}

impl SeleniumManager for SafariManager {
    fn get_browser_name(&self) -> &str {
        self.browser_name
    }

    fn get_http_client(&self) -> &Client {
        &self.http_client
    }

    fn set_http_client(&mut self, http_client: Client) {
        self.http_client = http_client;
    }

    fn get_browser_path_map(&self) -> HashMap<BrowserPath, &str> {
        HashMap::from([(
            BrowserPath::new(MACOS, STABLE),
            r#"/Applications/Safari.app"#,
        )])
    }

    fn discover_browser_version(&self) -> Option<String> {
        let mut browser_path = self.get_browser_path();
        if browser_path.is_empty() {
            match self.detect_browser_path() {
                Some(path) => {
                    browser_path = path;
                }
                _ => return None,
            }
        }
        let command = if MACOS.is(self.get_os()) {
            vec![format_one_arg(PLIST_COMMAND, browser_path)]
        } else {
            return None;
        };
        self.detect_browser_version(command)
    }

    fn get_driver_name(&self) -> &str {
        self.driver_name
    }

    fn request_driver_version(&self) -> Result<String, Box<dyn Error>> {
        Ok("(local)".to_string())
    }

    fn get_driver_url(&self) -> Result<String, Box<dyn Error>> {
        Err(format!("{} not available for download", self.get_driver_name()).into())
    }

    fn get_driver_path_in_cache(&self) -> PathBuf {
        PathBuf::from("/usr/bin/safaridriver")
    }

    fn get_config(&self) -> &ManagerConfig {
        &self.config
    }

    fn set_config(&mut self, config: ManagerConfig) {
        self.config = config;
    }

    fn get_logger(&self) -> &Logger {
        &self.log
    }

    fn set_logger(&mut self, log: Logger) {
        self.log = log;
    }
}
