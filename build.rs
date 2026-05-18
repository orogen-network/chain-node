// build.rs — embed git commit info and build version into the node binary
// via `substrate-build-script-utils::generate_cargo_keys()`.

fn main() {
    substrate_build_script_utils::generate_cargo_keys();
    substrate_build_script_utils::rerun_if_git_head_changed();
}
