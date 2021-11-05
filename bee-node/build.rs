// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::process::Command;

#[derive(Debug)]
enum BuildError {
    GitCommit,
    GitBranch,
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

    if cfg!(feature = "dashboard") {
        // check out frontend submodule
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "frontend"])
            .status()
            .expect("Couldn't check out dashboard submodule.");

        let _ = Command::new("cd")
            .args(&["src/plugins/dashboard/frontend"])
            .status()
            .expect("Couldn't switch to directory");

        let _ = Command::new("npm")
            .args(&["install"])
            .status()
            .expect("Couldn't install dashboard dependencies.");

        let _ = Command::new("npm")
            .args(&["run", "bee-build"])
            .status()
            .expect("Couldn't build dashboard.");
    }

    Ok(())
}
