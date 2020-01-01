use crate::config;
use reqwest::{header::HeaderMap, Client};
use std::env;
use std::result;
use std::sync::{LockResult, PoisonError, RwLockReadGuard, RwLockWriteGuard};
use uname::uname;
use uuid::Uuid;

/// A method to set custom HTTP-client, e.g. to change request timeout or to set a proxy.
///
/// Error would be returned if [`RwLock`] had been poisoned.
///
/// [`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
///
/// # Examples
///
/// ```
/// use plex_api::set_http_client;
/// use reqwest::{Client, Proxy};
/// use std::time::Duration;
///
///     set_http_client(Client::builder()
///                         .timeout(Duration::from_secs(1))
///                         .proxy(Proxy::http("http://example.com").expect("Proxy failed"))
///                         .build()
///                         .expect("Build failed")
///     ).expect("Mutex poisoned");
/// ```
pub fn set_http_client(
    c: Client,
) -> result::Result<(), PoisonError<RwLockWriteGuard<'static, Client>>> {
    let mut client = config::HTTP_CLIENT.write()?;
    *client = c;
    Ok(())
}

pub fn get_http_client() -> LockResult<RwLockReadGuard<'static, Client>> {
    config::HTTP_CLIENT.read()
}

pub fn base_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let i = uname().unwrap();

    let provides = *config::X_PLEX_PROVIDES.read().unwrap();
    headers.insert("X-Plex-Provides", provides.parse().unwrap());

    let mut product = *config::X_PLEX_PRODUCT.read().unwrap();
    if product == "" {
        product = config::PROJECT.unwrap_or("plex-api");
    }

    headers.insert("X-Plex-Product", product.parse().unwrap());

    let mut version = *config::X_PLEX_VERSION.read().unwrap();
    if version == "" {
        version = config::VERSION.unwrap_or("unknown");
    }

    headers.insert("X-Plex-Version", version.parse().unwrap());
    headers.insert(
        "X-Plex-Sync-Version",
        config::X_PLEX_SYNC_VERSION.read().unwrap().parse().unwrap(),
    );

    let mut platform = *config::X_PLEX_PLATFORM.read().unwrap();
    if platform == "" {
        platform = &i.sysname;
    }

    headers.insert("X-Plex-Platform", platform.parse().unwrap());

    let mut platform_version = *config::X_PLEX_PLATFORM_VERSION.read().unwrap();
    if platform_version == "" {
        platform_version = &i.release;
    }

    headers.insert("X-Plex-Platform-Version", platform_version.parse().unwrap());

    let mut client_identifier: String =
        String::from(*config::X_PLEX_CLIENT_IDENTIFIER.read().unwrap());
    if client_identifier == "" {
        let client_id = env::var("X_PLEX_CLIENT_IDENTIFIER");
        client_identifier = match client_id {
            Ok(id) => id,
            Err(_) => {
                warn!(target: "plex-api", "Generating random identifier for the machine! Set X_PLEX_CLIENT_IDENTIFIER to avoid this");
                let random_uuid = Uuid::new_v4();
                random_uuid.to_string()
            }
        };
    }

    headers.insert(
        "X-Plex-Client-Identifier",
        client_identifier.parse().unwrap(),
    );

    let mut device = *config::X_PLEX_DEVICE.read().unwrap();
    if device == "" {
        device = platform
    }

    headers.insert("X-Plex-Device", device.parse().unwrap());

    let mut device_name = *config::X_PLEX_DEVICE_NAME.read().unwrap();
    if device_name == "" {
        device_name = &i.nodename;
    }

    headers.insert("X-Plex-Device-Name", device_name.parse().unwrap());

    headers
}