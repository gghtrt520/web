use axum::{
    async_trait,
    body::Body,
    extract::FromRequestParts,
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};

use error::Error;
use error::Result;

use crate::{ctx::Ctx, Claims, KEYS};

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::Unauthorized)?;
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| Error::Unauthorized)?;

        Ok(token_data.claims)
    }
}

pub async fn mw_ctx_resolver(
    parts: &mut Parts,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| Error::Unauthorized)?;
    let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
        .map_err(|_| Error::Unauthorized)?;

    let ctx = Ctx::new(token_data.claims.user_id, token_data.claims.username);

    req.extensions_mut().insert(ctx);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        tracing::info!("handler=from_request_parts, ctx=required");
        let ctx = parts.extensions.get::<Ctx>().ok_or(Error::NoCtx)?.clone();
        Ok(ctx)
    }
}

pub async fn auth(mut req: Request<Body>, next: Next) -> Result<Response> {
    let auth_header = req.headers();

    tracing::info!("handler=middleware, header={:?}", auth_header);

    let auth: Authorization<Bearer> = auth_header.typed_get().ok_or(Error::Unauthorized)?;

    tracing::info!("handler=middleware, header={}", auth.token());

    let token_data = decode::<Claims>(auth.token(), &KEYS.decoding, &Validation::default());
    let claims = token_data.map_err(|_| Error::WrongAuthcation)?.claims;
    tracing::info!("handler=auth, claims={:?}", claims);
    let ctx = Ctx::new(claims.user_id, claims.username);
    req.extensions_mut().insert(ctx);
    Ok(next.run(req).await)
}
