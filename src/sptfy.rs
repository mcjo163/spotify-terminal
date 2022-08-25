use eyre::{Result, bail};
use std::net::TcpListener;
use std::io::{BufRead, BufReader, Write};
use rspotify::{prelude::*, scopes, Credentials, OAuth, AuthCodeSpotify, Config};

pub struct Sptfy {
    _spotify: AuthCodeSpotify,
}

impl Sptfy {

    pub fn default() -> Self {
        Self {
            _spotify: futures::executor::block_on(get_client()),
        }
    }
}

async fn get_client() -> AuthCodeSpotify {
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-read-private")).unwrap();

    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    });

    let cache_status = spotify
        .read_token_cache(true).await;

    // if a token is cached, use it
    if let Ok(Some(token)) = cache_status {
        println!("using cached token");
        *spotify.token.lock().await.unwrap() = Some(token);

    // otherwise, re-authenticate
    } else {
        println!("generating new token");
        let url = spotify.get_authorize_url(true).unwrap();
        open::that(&url).unwrap();
    
        let code = wait_for_code(&spotify).await.unwrap();
        spotify.request_token(&code).await.unwrap();
    }

    spotify
}

async fn wait_for_code(spotify: &AuthCodeSpotify) -> Result<String> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut stream = listener.incoming().flatten().next().unwrap();
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();

    reader.read_line(&mut request_line)?;

    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    match spotify.parse_response_code(&format!("http://localhost{redirect_url}")) {

        Some(code) => {
            let message = "Done! You can close this window now.";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes())?;
            Ok(code)
        },

        _ => bail!("unable to parse code"),
    }
}
