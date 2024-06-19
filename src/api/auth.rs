use crate::constants::constants;
use crate::db;
use crate::middleware::auth;
use crate::model::auth as auth_model;
use crate::security::cf_turnstile;
use crate::security::pw_hasher;
use crate::templates;
use crate::utils::html::render_template;
use actix_web::cookie::{time as cookie_time, Cookie, SameSite};
use actix_web::http::StatusCode;
use actix_web::{post, web, web::Data, web::Form, HttpRequest, HttpResponse};
use rand::Rng;
use tokio::time as tokio_time;

macro_rules! verify_captcha {
    ($req:expr, $cf_turnstile_res:expr) => {
        if !cf_turnstile::verify_request($req, $cf_turnstile_res).await {
            return Err(auth_model::AuthError::CaptchaFailed);
        }
    };
}

#[post("/api/admin")]
async fn admin_honeypot(
    req: HttpRequest,
    login_data: Form<auth_model::LoginData>,
) -> Result<HttpResponse, auth_model::AuthError> {
    log::warn!(
        "Honeypot triggered! Request IP: {} Username: {} Password: {}",
        cf_turnstile::get_ip_addr(&req).unwrap_or("unknown".to_string()),
        login_data.username,
        login_data.password
    );
    verify_captcha!(&req, &login_data.cf_turnstile_res);
    let sleep_time = rand::thread_rng().gen_range(2000..4000);
    tokio_time::sleep(tokio_time::Duration::from_millis(sleep_time)).await;
    Err(auth_model::AuthError::InvalidCredentials)
}

#[post("/api/auth/login")]
async fn login(
    req: HttpRequest,
    client: Data<db::DbClient>,
    login_data: Form<auth_model::LoginData>,
) -> Result<HttpResponse, auth_model::AuthError> {
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => {
            return Err(auth_model::AuthError::AlreadyLoggedIn);
        }
        None => {}
    }

    verify_captcha!(&req, &login_data.cf_turnstile_res);
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

        let exp_sec = if login_data.remember_session() {
            constants::SESSION_TIMEOUT_REMEMBER
        } else {
            constants::SESSION_TIMEOUT
        };
        let session_col = client.get_session_collection();
        let session = auth_model::Session::new(user._id, exp_sec);
        let session_expiry = session.expiry.timestamp_millis();
        let result = match session_col.insert_one(session, None).await {
            Ok(result) => result,
            Err(e) => {
                log::error!("Failed to insert session into db: {:?}", e);
                return Err(auth_model::AuthError::InternalServerError);
            }
        };

        let claims = auth::create_user_claim(user._id, result.inserted_id.as_object_id().unwrap());
        let token = auth::sign_payload(&claims);
        let max_age = if login_data.remember_session() {
            // offset_dt is 10 seconds before the expiry time for extra leeway
            let offset_dt =
                cookie_time::OffsetDateTime::from_unix_timestamp(session_expiry - 10_000);
            Some(offset_dt.unwrap())
        } else {
            None
        };

        let c = Cookie::build(constants::AUTH_COOKIE_NAME, token.clone())
            .domain(constants::get_domain())
            .path("/")
            .same_site(SameSite::Lax)
            .http_only(true)
            .secure(!constants::DEBUG_MODE)
            .expires(max_age)
            .finish();
        let template = templates::alerts::SucessAlert {
            msg: "You have logged in",
        };
        let mut response = render_template(template, StatusCode::OK);
        response.add_cookie(&c).unwrap();
        response
            .headers_mut()
            .insert("HX-Redirect".parse().unwrap(), "/".parse().unwrap());
        return Ok(response);
    })
    .await
    .unwrap()
    .await
}

#[post("/api/logout")]
async fn logout(req: HttpRequest) -> HttpResponse {
    let msg = "you have logged out";
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => {
            let mut auth_cookie = Cookie::build(constants::AUTH_COOKIE_NAME, "")
                .domain(constants::get_domain())
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .secure(!constants::DEBUG_MODE)
                .finish();
            auth_cookie.make_removal();
            HttpResponse::Ok().cookie(auth_cookie).body(msg)
        }
        None => HttpResponse::Ok().body(msg),
    }
}
