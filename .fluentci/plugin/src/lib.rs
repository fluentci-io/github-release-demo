use extism_pdk::*;
use fluentci_pdk::dag;

use crate::helpers::set_envs;

pub mod helpers;

#[plugin_fn]
pub fn clippy() -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("clippy")?
        .pkgx()?
        .with_packages(vec!["curl", "wget"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"])?
        .with_exec(vec!["rustup", "component", "add", "clippy"])?
        .with_exec(vec!["PATH=$HOME/.cargo/bin:$PATH", "cargo", "install", "clippy-sarif", "--version", "0.3.0"])?
        .with_exec(vec!["PATH=$HOME/.cargo/bin:$PATH","cargo", "install", "sarif-fmt", "--version", "0.3.0"])?
        .with_exec(vec![
            "PATH=$HOME/.cargo/bin:$PATH",
            "cargo",
            "clippy",
            "--all-features",
            "--message-format=json",
            " | clippy-sarif | tee rust-clippy-results.sarif | sarif-fmt)"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn llvmcov() -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("llvmcov")?
        .pkgx()?
        .with_packages(vec!["curl", "wget"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"])?
        .with_exec(vec!["rustup", "component", "add", "llvm-tools"])?
        .with_exec(vec![
            "wget",
            "https://github.com/taiki-e/cargo-llvm-cov/releases/download/v0.5.36/cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz"])?
        .with_exec(vec!["tar", "xvf", "cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz"])?
        .with_exec(vec!["mv", "cargo-llvm-cov", "/usr/local/bin"])?
        .with_exec(vec![
            "PATH=$HOME/.cargo/bin:$PATH",
            "cargo", 
            "llvm-cov",
            "--all-features",
            "--lib",
            "--workspace",
            "--lcov",
            "--output-path",
            "lcov.info"
        ])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn test(args: String) -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("test")?
        .pkgx()?
        .with_packages(vec!["curl", "wget"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"])?
        .with_exec(vec!["PATH=$HOME/.cargo/bin:$PATH", "cargo", "test", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn build(args: String) -> FnResult<String> {
    set_envs()?;

    dag().pipeline("")?.with_exec(vec!["mkdir", "-p", "target/x86_64-unknown-linux-gnu/release"])?.stdout()?;

    let stdout = dag()
        .pipeline("build")?
        .pkgx()?
        .with_exec(vec!["pkgx", "install", "curl"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"])?
        .with_exec(vec!["PATH=$HOME/.cargo/bin:$PATH", "cargo", "build", "--release", "--target", "x86_64-unknown-linux-gnu", &args])?
        .with_workdir("target/x86_64-unknown-linux-gnu/release")?
        .with_exec(vec!["tar", "czvf", "hello_${TAG}_x86_64-unknown-linux-gnu.tar.gz", "github-release-demo"])?
        .with_exec(vec!["shasum -a 256 hello_${TAG}_x86_64-unknown-linux-gnu.tar.gz > hello_${TAG}_x86_64-unknown-linux-gnu.tar.gz.sha256"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn target_add(args: String) -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("target_add")?
        .pkgx()?
        .with_exec(vec!["pkgx", "install", "curl"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"])?
        .with_exec(vec!["rustup", "target", "add", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn component_add(args: String) -> FnResult<String> {
    set_envs()?;

    let stdout = dag()
        .pipeline("component_add")?
        .pkgx()?
        .with_exec(vec!["pkgx", "install", "curl"])?
        .with_exec(vec!["type rustup > /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"])?
        .with_exec(vec!["rustup", "component", "add", &args])?
        .stdout()?;
    Ok(stdout)
}
