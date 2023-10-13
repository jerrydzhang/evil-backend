use std::collections::HashMap;

use jsonwebtoken::{Validation, decode, DecodingKey, jwk::{self, AlgorithmParameters}, decode_header, TokenData};
use cached::proc_macro::cached;

// verify a jwt token
pub(crate) async fn verify_jwt(token: &str) -> Result<TokenData<HashMap<String, serde_json::Value>>, Box<dyn std::error::Error>> {

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
                    decode::<HashMap<String, serde_json::Value>>(token, &decoding_key, &validation)?;
                return Ok(decoded_token);
            }
            _ => unreachable!("this should be a RSA"),
        }
    } else {
        return Err("No matching JWK found for the given kid".into());
    }
}

// get and cache jwks from auth0 website
#[cached(size=1, time = 1200, result = true)]
async fn get_jwks() -> Result<jwk::JwkSet, Box<dyn std::error::Error>> {
    let uri = std::env::var("AUTH0_JWKS").expect("AUTH0_JWKS should be set");
    let res = reqwest::get(uri).await?.text().await?;
    let jwks: jwk::JwkSet = serde_json::from_str(&res)?;
    Ok(jwks)
}