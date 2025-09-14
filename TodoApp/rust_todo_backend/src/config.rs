use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;

/// JWT secret loaded once at program start as a global static
pub static JWT_SECRET: Lazy<&'static [u8]> = Lazy::new(|| {
    dotenv().ok(); // load .env automatically
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
    Box::leak(secret.into_boxed_str()).as_bytes()
});
