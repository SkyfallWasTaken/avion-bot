use eyre::{eyre, Result};
use vergen::EmitBuilder;

pub fn main() -> Result<()> {
    // NOTE: This will output only a build timestamp and long SHA from git.
    // NOTE: This set requires the build and git features.
    // NOTE: See the EmitBuilder documentation for configuration options.
    EmitBuilder::builder()
        .build_timestamp()
        .git_sha(true)
        .rustc_semver()
        .emit()
        .map_err(|_| eyre!("failed to get build info"))?;
    Ok(())
}
