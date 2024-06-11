use crate::constants::constants;
use crate::db;
use crate::model::auth as auth_model;
use crate::security::pw_hasher;
use crate::security::jwt;
use actix_web::cookie::{time as cookie_time, Cookie, SameSite};
use actix_web::{get, post, web, web::Data, web::Json, Error, HttpRequest, HttpResponse};
use rand::Rng;
use tokio::time as tokio_time;
use crate::middleware::auth;
use crate::security::jwt::JwtSignerLogic;
use crate::utils::security;

macro_rules! honeypot_logic {
    ($login_data:expr) => {
        log::info!(
            "Honeypot triggered! Username: {} Password: {}",
            $login_data.username,
            $login_data.password
        );
        let sleep_time = rand::thread_rng().gen_range(500..1500);
        tokio_time::sleep(tokio_time::Duration::from_millis(sleep_time)).await;
        return Err(actix_web::error::ErrorForbidden(
            "wrong username or password",
        ));
    };
}

#[post("/wp-admin.php")]
async fn wp_honeypot(login_data: Json<auth_model::LoginData>) -> Result<HttpResponse, Error> {
    honeypot_logic!(login_data);
}

#[post("/admin")]
async fn admin_honeypot(login_data: Json<auth_model::LoginData>) -> Result<HttpResponse, Error> {
    honeypot_logic!(login_data);
}

#[post("/login")]
async fn login_honeypot(login_data: Json<auth_model::LoginData>) -> Result<HttpResponse, Error> {
    honeypot_logic!(login_data);
}

#[post("auth/login")]
async fn login(
    req: HttpRequest,
    client: Data<db::DbClient>,
    login_data: Json<auth_model::LoginData>,
) -> Result<HttpResponse, auth_model::AuthError> {
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => {
            return Err(auth_model::AuthError::AlreadyLoggedIn);
        }
        None => {}
    }

    web::block(move || async move {
        let user = client.get_user_by_username(&login_data.username).await?;
        let is_valid = match pw_hasher::verify_password(&login_data.password, user.get_password()) {
            Ok(is_valid) => is_valid,
            Err(_) => {
                return Err(auth_model::AuthError::InternalServerError);
            }
        };
        if !is_valid {
            return Err(auth_model::AuthError::InvalidCredentials);
        }

        let signer = jwt::JwtSigner::new(security::get_default_jwt_key(), jsonwebtoken::Algorithm::HS512);
        let claims = auth::create_user_claim(user.get_id());
        let token = match signer.sign(&claims) {
            Ok(token) => token,
            Err(_) => {
                return Err(auth_model::AuthError::InternalServerError);
            }
        };

        let max_age = claims.exp.timestamp() - chrono::Utc::now().timestamp();
        let c = Cookie::build(constants::AUTH_COOKIE_NAME, token.clone())
            .domain(constants::get_domain())
            .path("/")
            .same_site(SameSite::Lax)
            .http_only(true)
            .secure(!constants::DEBUG_MODE)
            .max_age(cookie_time::Duration::seconds(max_age))
            .finish();
        let response = auth_model::LoginResponse {
            token,
            username: user.get_username().to_string(),
        };
        return Ok(HttpResponse::Ok().cookie(c).json(response));
    })
    .await
    .unwrap()
    .await
}

#[get("/auth/logout")]
async fn logout(req: HttpRequest) -> HttpResponse {
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => {
            let c = Cookie::build(constants::AUTH_COOKIE_NAME, "")
                .domain(constants::DOMAIN)
                .path("/")
                .http_only(true)
                .secure(!constants::DEBUG_MODE)
                .finish();
            HttpResponse::Ok().cookie(c).finish()
        }
        None => HttpResponse::Ok().finish(),
    }
}