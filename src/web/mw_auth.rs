use axum::{http::{Request}, middleware::Next};
use tower_cookies::Cookies;
use crate::Result;
use crate::{web::AUTH_TOKEN, Error};
use axum::response::Response;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>, 
    next: Next<B>
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    //TO DO: Real auth token parsing
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}