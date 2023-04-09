use minecraft_msa_auth::{MinecraftAccessToken, MinecraftAuthorizationFlow};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use reqwest::{header, Client, Url};
use tauri::AppHandle;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use crate::domain;
use crate::domain::login::LoginInfo;

const DEVICE_CODE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
const MSA_AUTHORIZE_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const MSA_TOKEN_URL: &str = "https://login.microsoftonline.com/common/oauth2/v2.0/token";
const MINECRAFT_API_URL: &str = "https://api.minecraftservices.com";

lazy_static::lazy_static! {
    pub static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

fn get_mcapi_endpoint_url(endpoint: &str) -> String {
    format!("{}{}", MINECRAFT_API_URL, "/minecraft/profile").to_string()
}

pub async fn get_profile_info(
    token: &MinecraftAccessToken,
) -> Result<domain::login::ProfileInfoResponse, anyhow::Error> {
    let access_token = token;

    #[cfg(debug_assertions)] // This is a sensitive information, so we don't want to print it in release builds
    println!("{}", access_token.as_ref());

    let response = HTTP_CLIENT
        .get(get_mcapi_endpoint_url("/minecraft/profile"))
        .header(header::AUTHORIZATION, {
            let mut auth =
                header::HeaderValue::from_str(format!("Bearer {}", access_token.as_ref()).as_str())
                    .unwrap();
            auth.set_sensitive(true); // So that it doesn't get printed in Debug representation
            auth
        })
        .header(
            header::CONTENT_LENGTH,
            header::HeaderValue::from_static("0"),
        )
        .send()
        .await?;

    let info: domain::login::ProfileInfoResponse = response.json().await.expect("error unwrapping");

    Ok(info)
}

pub async fn login_in_ms(login: &mut Option<LoginInfo>, app: &AppHandle) -> anyhow::Result<()> {
    let client = BasicClient::new(
        ClientId::new("01df9ec4-9a16-4251-aaf5-cbadb01eb310".to_string()),
        None,
        AuthUrl::new(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize".to_string(),
        )?,
        Some(TokenUrl::new(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/token".to_string(),
        )?),
    )
    // Microsoft requires client_id in URL rather than using Basic authentication.
    .set_auth_type(AuthType::RequestBody)
    // This example will be running its own server at 127.0.0.1:8114.
    // See below for the server implementation.
    .set_redirect_uri(
        RedirectUrl::new("http://127.0.0.1:8114/redirect".to_string())
            .expect("Invalid redirect URL"),
    );

    // Microsoft supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("XboxLive.signin offline_access".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();
    // println!(
    //     "Open this URL in your browser:\n{}\n",
    //     authorize_url.to_string()
    // );

    // open::that(authorize_url.to_string()).expect("error opening");

    let oauth2_window = tauri::WindowBuilder::new(
        app,
        "Auth",
        tauri::WindowUrl::External(authorize_url.to_string().parse().unwrap()),
    )
    .inner_size(470f64, 500f64)
    .title("Microsoft Oauth2")
    .build()?;

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8114").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        stream.readable().await?;
        let mut stream = BufReader::new(stream);

        let (code, state): (AuthorizationCode, CsrfToken) = {
            let mut request_line = String::new();
            stream.read_line(&mut request_line).await?;

            let redirect_url = request_line.split_whitespace().nth(1).unwrap();
            let url = Url::parse(&("http://localhost".to_string() + redirect_url))?;

            let code = url
                .query_pairs()
                .find_map(|(key, value)| (key == "code").then_some(value))
                .unwrap();
            let code = AuthorizationCode::new(code.into_owned());

            let token = url
                .query_pairs()
                .find_map(|(key, value)| (key == "state").then_some(value))
                .unwrap();

            (code, CsrfToken::new(token.into_owned()))
        };

        let message = "";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        stream.write_all(response.as_bytes()).await?;

        // println!("MS returned the following code:\n{}\n", code.secret());
        // println!(
        //     "MS returned the following state:\n{} (expected `{}`)\n",
        //     state.secret(),
        //     csrf_state.secret()
        // );

        // Exchange the code with a token.
        let token = client
            .exchange_code(code)
            // Send the PKCE code verifier in the token request
            .set_pkce_verifier(pkce_code_verifier)
            .request_async(async_http_client)
            .await?;
        // println!("microsoft token:\n{:?}\n", token);

        // Exchange the Microsoft token with a Minecraft token.
        let mc_flow = MinecraftAuthorizationFlow::new(Client::new());
        let mc_token = mc_flow
            .exchange_microsoft_token(token.access_token().secret())
            .await?;
        // println!("minecraft token: {:?}", mc_token);

        // The server will terminate itself after collecting the first code.
        *login = Some(domain::login::LoginInfo {
            access_token: mc_token.access_token().clone(),
            username: mc_token.username().clone(),
            profile: get_profile_info(&mc_token.access_token()).await.unwrap(),
        });

        oauth2_window.close()?;

        break;
    }

    Ok(())
}
