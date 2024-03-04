use std::{collections::HashSet, pin::Pin, future::Future};

use actix_web::{FromRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use cached::proc_macro::cached;
use jsonwebtoken::{jwk::{self, AlgorithmParameters}, decode_header, DecodingKey, Validation, decode, TokenData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Claims {
    #[serde(rename = "https://localhost:8080/roles")]
    roles: Option<HashSet<String>>,
    pub(crate) sub: String,
}

impl Claims {
    pub fn validate_roles(&self, required_permissions: &HashSet<String>) -> bool {
        log::info!("Validating permissions: {:?} against {:?}", self.roles, required_permissions);
        if let Some(roles) = &self.roles {
            if !roles.is_superset(required_permissions) {
                false
            } else {
                true
            }
        } else {
            false
        }
    }
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest, 
        _payload: &mut actix_web::dev::Payload
    ) -> Self::Future {
        let extractor = BearerAuth::extract(req);
        Box::pin(async move {
            let credientials = extractor.await?;
            let token = credientials.token();
            let token = verify_jwt(token).await?;
            Ok(token.claims)
        })
    }
}

/// verify a jwt token
pub(crate) async fn verify_jwt(token: &str) -> Result<TokenData<Claims>, Box<dyn std::error::Error>> {

    // get the jwks from auth0
    let jwks: jwk::JwkSet = get_jwks().await?;

    // decode the token header
    let header = decode_header(token)?;

    // get the kid from the header
    let kid = match header.kid {
        Some(k) => k,
        None => return Err("Token doesn't have a `kid` header field".into()),
    };

    // find the jwk that matches the kid
    if let Some(j) = jwks.find(&kid) {
        // make sure the jwk is a RSA
        match &j.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                let validation = Validation::new(j.common.algorithm.unwrap());
                let decoded_token =
                    decode::<Claims>(token, &decoding_key, &validation)?;
                return Ok(decoded_token);
            }
            _ => unreachable!("this should be a RSA"),
        }
    } else {
        return Err("No matching JWK found for the given kid".into());
    }
}

/// get and cache jwks from auth0 website
#[cached(size=1, time = 1200, result = true)]
async fn get_jwks() -> Result<jwk::JwkSet, Box<dyn std::error::Error>> {
    let uri = std::env::var("AUTH0_JWKS").expect("AUTH0_JWKS should be set");
    let res = reqwest::get(uri).await?.text().await?;
    let jwks: jwk::JwkSet = serde_json::from_str(&res)?;
    Ok(jwks)
}