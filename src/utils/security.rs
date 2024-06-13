use crate::constants::constants;
use actix_web::dev::ServiceRequest;
use actix_web::http::Method;
use actix_web::{HttpMessage, HttpRequest};
use rand::Rng as _;

/// This assumes that the environment variables are in hex format
pub fn get_bytes_from_env(key: &str) -> Vec<u8> {
    let hex = std::env::var(key).unwrap();
    hex::decode(hex).unwrap()
}

pub fn get_default_secret_key() -> Vec<u8> {
    get_bytes_from_env(constants::SECRET_KEY)
}

pub fn get_default_salt() -> Vec<u8> {
    get_bytes_from_env(constants::SECRET_KEY_SALT)
}

pub fn get_default_key_info(salt: Vec<u8>, info: Vec<u8>) -> hmac_serialiser_rs::KeyInfo {
    hmac_serialiser_rs::KeyInfo {
        key: get_default_secret_key(),
        salt,
        info,
    }
}

pub fn get_auth_signer() -> hmac_serialiser_rs::HmacSigner {
    hmac_serialiser_rs::HmacSigner::new(
        get_default_key_info(get_default_salt(), vec![]),
        hmac_serialiser_rs::algorithm::Algorithm::SHA512,
        hmac_serialiser_rs::Encoder::UrlSafeNoPadding,
    )
}

// https://rust-random.github.io/book/guide-rngs.html
pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut random_bytes = vec![0u8; length];
    rand::thread_rng().fill(&mut random_bytes[..]);
    random_bytes
}

pub fn is_protected(
    whitelist: &Vec<(Method, String)>,
    whitelist_regex: &Vec<(Method, regex::Regex)>,
    req: &ServiceRequest,
) -> bool {
    let method = req.method();
    let path = req.path();
    for (allowed_method, allowed_path) in whitelist {
        if allowed_method == method && allowed_path == path {
            return false;
        }
    }
    for (allowed_method, allowed_path) in whitelist_regex {
        if allowed_method == method && allowed_path.is_match(path) {
            return false;
        }
    }
    true
}

pub fn get_csrf_header_json(req: &HttpRequest) -> String {
    match req.cookie(constants::CSRF_COOKIE_NAME) {
        Some(cookie) => {
            let csrf_token = cookie.value();
            format!(
                "{{\"{}\": \"{}\"}}",
                constants::CSRF_HEADER_NAME,
                csrf_token
            )
        }
        None => {
            log::info!("No CSRF token found in request");
            String::from("{}")
        }
    }
}

pub fn is_logged_in(req: &HttpRequest) -> bool {
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => true,
        None => false,
    }
}

pub struct TemplateValues {
    pub nonce: String,
    pub csrf_header: String,
    pub is_logged_in: bool,
}

pub fn extract_for_template(req: &HttpRequest) -> TemplateValues {
    let is_logged_in = is_logged_in(req);
    let nonce = {
        let extensions = req.extensions();
        let csp_nonce = extensions
            .get::<crate::middleware::csp::CspNonce>()
            .unwrap();
        csp_nonce.get_nonce().to_string()
    };
    let csrf_header = get_csrf_header_json(req).to_string();
    TemplateValues {
        nonce,
        csrf_header,
        is_logged_in,
    }
}