use std::sync::RwLock;

use once_cell::sync::Lazy;

static RATE_LIMIT_REQUESTS_SCRIPT: Lazy<RwLock<String>> = 
    Lazy::new(|| RwLock::new(String::from("rate_limit_request_script.lua")));

static QUOTA_TOKEN_SCRIPT: Lazy<RwLock<String>> = 
    Lazy::new(|| RwLock::new(String::from("quota_oken_scrupt.lua")));