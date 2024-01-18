use std::env;
use std::path::PathBuf;

use deno_core::snapshot_util::{create_snapshot, CreateSnapshotOptions};

fn main() {
    // Create the runjs extension.
    let js_files = vec![deno_core::ExtensionFileSource {
        specifier: "ext:src/runtime.js",
        code: deno_core::ExtensionFileSourceCode::IncludedInBinary(include_str!("src/runtime.js")),
    }]
    .into();
    let runjs_extension = deno_core::Extension {
        name: "runjs",
        js_files,
        ..Default::default()
    };

    // Build the file path to the snapshot.
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let snapshot_path = out_dir.join("RUNJS_SNAPSHOT.bin");

    // Create the snapshot.
    let snapshot_output = create_snapshot(CreateSnapshotOptions {
        cargo_manifest_dir: env!("CARGO_MANIFEST_DIR"),
        snapshot_path,
        extensions: vec![runjs_extension],
        startup_snapshot: None,
        skip_op_registration: false,
        compression_cb: None,
        with_runtime_cb: None,
    });
    for file_path in snapshot_output.files_loaded_during_snapshot {
        println!("cargo:rerun-if-changed={}", file_path.display());
    }
}
