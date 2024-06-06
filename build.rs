use vergen::EmitBuilder;

fn main() {
    // NOTE: This will output only a build timestamp and long SHA from git.
    // NOTE: This set requires the build and git features.
    // NOTE: See the EmitBuilder documentation for configuration options.
    EmitBuilder::builder()
        .build_timestamp()
        .git_sha(true)
        .rustc_semver()
        .emit()
        .unwrap();
}
