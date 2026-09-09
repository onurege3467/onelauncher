
pub const DUMMY_REPLACE_NEWLINE: &str = "\n";

pub const MICROSOFT_CLIENT_ID: &str = "9419b7ee-1448-4d1b-b52a-550d8f36ab56";
pub const MINECRAFT_SCOPES: &str = "XboxLive.SignIn XboxLive.offline_access";
pub const CURSEFORGE_API_KEY: &str = "";
pub const DISCORD_CLIENT_ID: &str = "1350936009596211353";

pub const MODRINTH_API_URL: &str = "https://api.modrinth.com";
pub const MODRINTH_CDN_PREFIX: &str = "https://cdn.modrinth.com/data/";
pub const CURSEFORGE_API_URL: &str = "https://api.curseforge.com/v1";
pub const CURSEFORGE_GAME_ID: u32 = 432;
pub const METADATA_API_URL: &str = "https://meta.polyfrost.org";
pub const MCLOGS_API_URL: &str = "https://api.mclo.gs/1";
pub const SKYCLIENT_BASE_URL: &str =
	"https://raw.githubusercontent.com/SkyblockClient/SkyblockClient-REPO/refs/heads/main/v1";
pub const META_URL_BASE: &str = "https://raw.githubusercontent.com/onurege3467/onelauncher/main";
pub const TOS_URL: &str = "https://github.com/onurege3467/onelauncher";
pub const PRIVACY_URL: &str = "https://github.com/onurege3467/onelauncher";
pub const PLUS_BACKEND_URL: &str = "https://plus.polyfrost.org";

pub const SENTRY_DSN: &str = match option_env!("ONECLIENT_SENTRY_DSN") {
	Some(dsn) => dsn,
	None => "",
};

pub const TARGET_OS: &str = cfg_select! {
    target_os = "windows" => "windows",
    target_os = "macos" => "osx",
    target_os = "linux" => "linux"
};

pub const NATIVE_ARCH: &str = cfg_select! {
    target_arch = "x86" => "32",
    target_arch = "x86_64" => "64",
    _ => "64"
};

pub const JAVA_BIN: &str = if cfg!(windows) { "javaw.exe" } else { "java" };

pub const ARCH_WIDTH: &str = if cfg!(target_pointer_width = "64") {
	"64"
} else {
	"32"
};

pub const LINE_ENDING: &str = if cfg!(windows) { "\r\n" } else { "\n" };
pub const CLASSPATH_SEPARATOR: &str = if cfg!(windows) { ";" } else { ":" };
