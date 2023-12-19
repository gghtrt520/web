use axum::body::Body;
use axum::http::{Method, Uri};
use axum::response::Response;
use axum::{http::Request, middleware::Next};

use casbin::{CoreApi, Enforcer};
use error::Error;
use error::Result;

use crate::ctx::Ctx;

pub async fn access(
    uri: Uri,
    ctx: Ctx,
    method: Method,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    tracing::info!("handler=access, ctx={:?}", ctx);
    let mut e = Enforcer::new(
        "/config/access/rbac_with_pattern_model.conf",
        "/config/access/rbac_with_pattern_policy.csv",
    )
    .await
    .map_err(|_| Error::NoAccess)?;
    e.enable_log(true);
    let assscee = e
        .enforce((ctx.username(), uri.path(), method.to_string()))
        .map_err(|_| Error::NoAccess)?;
    if assscee {
        Ok(next.run(req).await)
    } else {
        Err(Error::NoAccess)
    }
}
