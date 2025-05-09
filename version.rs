#[derive(Clone)]
pub struct Version {
    major: i32,
    minor: i32,
    patch: i32,
}

impl Default for Version {
    fn default() -> Self {
        Version {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
}

impl Version {
    pub fn new(major: i32, minor: i32, patch: i32) -> Self {
        Version { major, minor, patch }
    }
}