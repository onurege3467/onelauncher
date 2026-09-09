use chrono::{Duration, Utc};
use md5::{Digest, Md5};
use uuid::Uuid;

use super::error::AuthError;
use super::data::{AccountKind, MinecraftAccount};
use crate::error::AuthResult;

pub fn offline_uuid(username: &str) -> Uuid {
    let mut hasher = Md5::new();
    hasher.update(format!("OfflinePlayer:{username}").as_bytes());
    let digest = hasher.finalize();

    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest);
    bytes[6] = (bytes[6] & 0x0f) | 0x30;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}

pub fn validate_offline_username(username: &str) -> AuthResult<()> {
    let len = username.chars().count();
    if !(3..=16).contains(&len) {
        return Err(AuthError::InvalidOfflineUsername {
            reason: "username must be 3-16 characters".into(),
        }
        );
    }

    if !username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(AuthError::InvalidOfflineUsername {
            reason: "username may only contain letters, digits, and underscores".into(),
        }
        );
    }

    Ok(())
}

#[tracing::instrument(level = "debug", fields(username = %username))]
pub fn offline_account(username: String) -> MinecraftAccount {
    let uuid = offline_uuid(&username);
    offline_account_with_uuid(username, uuid)
}

pub fn offline_account_with_uuid(username: String, uuid: Uuid) -> MinecraftAccount {
    tracing::info!("creating offline account");
    MinecraftAccount {
        id: uuid,
        username,
        access_token: String::new(),
        refresh_token: String::new(),
        expires: Utc::now() + Duration::days(3650),
        kind: AccountKind::Offline,
    }
}

/// Fxes sunucusu offline modda calisir, ikisi de kabul edilir - ama gercek
/// (premium) bir Mojang hesabiysa skin/cape gorunsun diye asil uuid'i
/// kullanmak deterministik offline uuid'den daha iyidir. Ag hatasi veya
/// hesap bulunamamasi sessizce offline uuid'e duser (eski Helios launcher'in
/// authmanager.js'teki `premiumUuid` davranisinin portu).
#[tracing::instrument(level = "debug", skip(client), fields(username = %username))]
pub async fn resolve_offline_uuid(client: &reqwest::Client, username: &str) -> Uuid {
    match lookup_premium_uuid(client, username).await {
        Some(uuid) => uuid,
        None => offline_uuid(username),
    }
}

async fn lookup_premium_uuid(client: &reqwest::Client, username: &str) -> Option<Uuid> {
    #[derive(serde::Deserialize)]
    struct Profile {
        id: String,
    }

    let url = format!("https://api.mojang.com/users/profiles/minecraft/{username}");
    let response = client.get(url).send().await.ok()?;
    if !response.status().is_success() {
        return None;
    }

    let profile: Profile = response.json().await.ok()?;
    parse_undashed_uuid(&profile.id)
}

fn parse_undashed_uuid(raw: &str) -> Option<Uuid> {
    if raw.len() != 32 || !raw.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }

    let dashed = format!(
        "{}-{}-{}-{}-{}",
        &raw[0..8],
        &raw[8..12],
        &raw[12..16],
        &raw[16..20],
        &raw[20..32]
    );
    Uuid::parse_str(&dashed).ok()
}
