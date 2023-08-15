use std::process::Command;

#[allow(dead_code)]
fn shell(command: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .unwrap_or_else(|_| panic!("Failed to run {command}"));

    if !output.status.success() {
        panic!(
            "Command {} failed with status code {:?}",
            command, output.status
        );
    }
}

fn main() {
    #[cfg(not(debug_assertions))]
    shell("cd frontend && pnpm install --frozen-lockfile");

    #[cfg(not(debug_assertions))]
    shell("cd frontend && pnpm build");
}
