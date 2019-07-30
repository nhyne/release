use std::env;
use std::fs::File;
use std::io::{Read, Write};
//use std::fmt::Error;
use std::io::{Error as IoError, ErrorKind};
use std::borrow::Borrow;
use std::error::Error;
use std::sync::atomic::Ordering::Release;


fn main() {
    let args : Vec<String> = env::args().collect();
    let bump_arg = &args[1];
    let bump_type = read_bump_type(bump_arg).unwrap_or(ReleaseType::Patch);

//    // first read in version from existing version file
//        // if the file doesn't exist we should create and push it
//    // set version appropriately based on argument
//    // add and push git tag
//    // add patch and -snapshot to VERSION file
//    // commit version file
    let read_version = read_version();
    match (bump_type, read_version) {
        (ReleaseType::Major, Ok(read_version)) => {
            let new_version = ReleaseVersion{
                major: read_version.major + 1,
                minor: 0,
                patch: 0
            };

        }
        (ReleaseType::Minor, Ok(read_version)) => {
            let new_version = ReleaseVersion{
                major: read_version.major,
                minor: read_version.minor + 1,
                patch: 0
            };

        }
        (ReleaseType::Patch, Ok(read_version)) => {
            let new_version = ReleaseVersion{
                major: read_version.major,
                minor: read_version.minor,
                patch: read_version.patch + 1
            };

        }
        (_, Err(e)) => panic!(e.to_string())
    }
}

#[derive(Debug)]
struct ReleaseVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl ReleaseVersion {
    fn bump_major(&mut self) {
        self.major = self.major + 1;
    }

    fn bump_minor(&mut self) {
        self.minor = self.minor + 1;
    }

    fn bump_patch(&mut self) {
        self.patch = self.patch + 1;
    }
}

enum ReleaseType {
    Major,
    Minor,
    Patch,
}

fn read_bump_type(arg: &str) -> Result<ReleaseType, &str> {
    match arg {
        "major" => Ok(ReleaseType::Major),
        "minor" => Ok(ReleaseType::Minor),
        "patch" => Ok(ReleaseType::Patch),
        _ => Err("Bad release type argument"),
    }
}

fn read_version() -> Result<ReleaseVersion, &'static str> {
    let mut file = File::open("VERSION").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    println!("{}", buffer);

    // convert string input to release version
    let split = buffer.split(".");
    let vec: Vec<&str> = split.collect();

    let major = vec[0];
    println!("major: {}", major);
    let minor = vec[1];
    println!("minor: {}", minor);
    let initial_patch = vec[2];
    println!("patch: {}", initial_patch);

    let major = major.parse::<u32>().ok();
    let minor = minor.parse::<u32>().ok();

    // I believe that the patch version is reading a \n on top of the regular number
    //   need to account for this by attempting to pop off the \n
    let patch = initial_patch.parse::<u32>();

    match (major, minor, patch) {
        (Some(major), Some(minor), Ok(patch)) => {
            let r = ReleaseVersion{
                major,
                minor,
                patch,
            };
            Ok(r)

        }
        (None, _, _) =>  {
            Err("Could not read major version")
        }
        (_, None, _) => {
            Err("Could not read minor version")
        }
        (Some(major), Some(minor), Err(_)) => {
            let mut patcher: String = String::from(initial_patch);
            patcher.pop();
            let val = patcher.parse::<u32>();
            match val {
                Ok(patch) => {
                    let r = ReleaseVersion{
                        major,
                        minor,
                        patch
                    };
                    println!("recovered from newline");
                    Ok(r)
                }
                Err(e) => {
                    println!("attempted to recover from newline read and failed");
                    Err("failed to read newline file")
                }
            }
        }
    }

}
