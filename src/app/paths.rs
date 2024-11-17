use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::LazyLock;

#[allow(dead_code)]
pub static CARGO_MANIFEST_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR")].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static LOG_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "log"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static TESTS_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static TMP_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "tmp"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static TARGET_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "target"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static DEBUG_DIR: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "target", "debug"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static COMMAND_FILE: LazyLock<PathBuf> = LazyLock::new(||
    [env!("CARGO_MANIFEST_DIR"), "target", "debug", env!("CARGO_PKG_NAME")].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static COMMAND_OS: LazyLock<OsString> = LazyLock::new(||
    OsString::from([env!("CARGO_MANIFEST_DIR"), "target", "debug", env!("CARGO_PKG_NAME")].iter().collect::<PathBuf>())
);
