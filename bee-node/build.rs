// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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
        if !Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .status()
            .expect("Failed to run 'git submodule update'")
            .success()
        {
            panic!();
        }

        let frontend_dir = "src/plugins/dashboard/frontend";
        // install dependencies
        if !Command::new("npm")
            .args(&["install"])
            .current_dir(frontend_dir)
            .status()
            .expect("Failed to run 'npm install'")
            .success()
        {
            panic!();
        }

        // bundle dashboard
        if !Command::new("npm")
            .args(&["run", "build-bee"])
            .current_dir(frontend_dir)
            .status()
            .expect("Failed to run 'npm run build-bee'")
            .success()
        {
            panic!();
        }
    }

    Ok(())
}
