pub const SUPPORTED_SERVER_API_VERSION: u32 = 1;
pub const CACHE_VERSION: u8 = 2;

// Filesystem

#[cfg(windows)]
pub const VOXYGEN_FILE: &str = "veloren-voxygen.exe";
#[cfg(unix)]
pub const VOXYGEN_FILE: &str = "veloren-voxygen";

#[cfg(windows)]
pub const LOGS_DIR: &str = "userdata\\voxygen\\logs";

#[cfg(unix)]
pub const LOGS_DIR: &str = "userdata/voxygen/logs";

//#[cfg(windows)]
//pub const SERVER_CLI_FILE: &str = "veloren-server-cli.exe";
#[cfg(unix)]
pub const SERVER_CLI_FILE: &str = "veloren-server-cli";

pub const SAVED_STATE_FILE: &str = "airshipper_state.ron";
pub const LOG_FILE: &str = "airshipper.log";

// Networking

pub const GITHUB_RELEASES_API_URL: &str =
    "https://api.github.com/repos/tylerjgoodhue1995/Rune-Haven/releases?per_page=20";
pub const RUNEHAVEN_RELEASES_URL: &str =
    "https://github.com/tylerjgoodhue1995/Rune-Haven/releases";
// For user linking
pub const NEWS_URL: &str = "https://veloren.net/rss.xml";

pub const COMMUNITY_SHOWCASE_URL: &str = "https://veloren.net/community-showcase/rss.xml";

pub const AIRSHIPPER_RELEASE_URL: &str =
    "https://github.com/tylerjgoodhue1995/Spaceship/releases";
