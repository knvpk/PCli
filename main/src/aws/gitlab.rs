use jsonwebtoken::{decode, decode_header, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

// Reference: https://docs.gitlab.com/ee/ci/secrets/id_token_authentication.html
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    namespace_id: String,
    namespace_path: String,
    project_id: String,
    project_path: String,
    user_id: String,
    user_login: String,
    user_email: String,
    user_access_level: String,
    pipeline_id: String,
    pipeline_source: String,
    environment: String,
    job_id: String,
    // ref: String,
    ref_type: String,
    ref_path: String,
    ref_protected: String,
    runner_id: u32,
    runner_environment: String,
    sha: String,
    project_visibility: String,
    ci_config_ref_uri: String,
    ci_config_sha: String,
    jti: String,
    iss: String,
    iat: u128,
    nbf: u128,
    exp: u128,
    sub: String,
}

const URL: &str = "https://git.example.com/";

#[derive(Debug, Serialize, Deserialize)]
pub struct JWKS {
    kty: String,
    kid: String,
    e: String,
    n: String,
    // use: String,
    alg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWKSResponse {
    keys: Vec<JWKS>,
}

pub async fn decode_token(token: String) -> TokenData<TokenClaims> {
    let header = decode_header(&token).expect("Unable to decode header.");
    let kid = header.kid.unwrap();

    let url = format!("{}-/jwks", URL);
    let response = reqwest::get(url).await.expect("Unkonw error getting jwks");
    let content = response
        .text()
        .await
        .expect("Unable to extract test from response.");
    let keys: JWKSResponse = from_str(&content).expect("Unable to serialize the jwks response");
    let jwk = keys.keys.iter().find(|k| k.kid == kid).unwrap();
    let token_data = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_rsa_components(jwk.n.as_str(), jwk.e.as_str()).unwrap(),
        &Validation::new(header.alg),
    )
    .expect("Token signature invalid");
    token_data
}

pub fn generate_profile(claims: TokenClaims) -> String {
    format!(
        "gitlab_job_{project_id}_{pipeline_id}_{job_id}_{environ}",
        pipeline_id = claims.pipeline_id,
        job_id = claims.job_id,
        environ = claims.environment,
        project_id = claims.project_id
    )
}
