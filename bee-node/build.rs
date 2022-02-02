// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::process::Command;

#[derive(Debug)]
enum BuildError {
    GitCommit,
    GitBranch,
    GitSubmodule,
    NpmInstall,
    NpmBuild,
}

fn main() -> Result<(), BuildError> {
    match Command::new("git").args(&["rev-parse", "HEAD"]).output() {
        Ok(output) => {
            println!(
                "cargo:rustc-env=GIT_COMMIT={}",
                String::from_utf8(output.stdout).unwrap()
            );
        }
        Err(_) => return Err(BuildError::GitCommit),
    }

    match Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
    {
        Ok(output) => {
            println!("cargo:rerun-if-changed=../.git/HEAD");
            println!(
                "cargo:rerun-if-changed=../.git/refs/heads/{}",
                String::from_utf8(output.stdout).unwrap(),
            );
        }
        Err(_) => return Err(BuildError::GitBranch),
    }

    #[cfg(feature = "dashboard")]
    if !Path::new("./bee-node/src/plugins/dashboard/frontend/build").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .status()
            .or(Err(BuildError::GitSubmodule))?;

        Command::new("npm")
            .args(&["install"])
            .current_dir("./bee-node/src/plugins/dashboard/frontend/")
            .status()
            .or(Err(BuildError::NpmInstall))?;

        Command::new("npm")
            .args(&["run", "build-bee"])
            .current_dir("./bee-node/src/plugins/dashboard/frontend/")
            .status()
            .or(Err(BuildError::NpmBuild))?;
    }

    Ok(())
}
