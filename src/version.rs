use anyhow::Context;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    major: u64,
    minor: u64,
    patch: u64,
}

impl Version {
    pub fn major(&self) -> u64 {
        self.major
    }
    pub fn minor(&self) -> u64 {
        self.minor
    }
    pub fn patch(&self) -> u64 {
        self.patch
    }

    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parse the version number from a string, supporting "v1.2.3" or "1.2.3" format
    pub fn parse(v: &str) -> anyhow::Result<Self> {
        let re = regex::Regex::new("v?(\\d+)\\.(\\d+)\\.(\\d+)").unwrap();
        let caps = re
            .captures(v)
            .context(format!("{v} is not a correct version number"))?;
        let major: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
        let minor: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
        let patch: u64 = caps.get(3).unwrap().as_str().parse().unwrap();
        Ok(Self::new(major, minor, patch))
    }
}

/// Implement version number comparison, prioritizing major version, then minor version, and finally patch number
impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.major().cmp(&other.major());
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }
        let ord = self.minor().cmp(&other.minor());
        if ord != std::cmp::Ordering::Equal {
            return ord;
        }
        self.patch().cmp(&other.patch())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Represents a version number matching requirement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionReq {
    major: Option<u64>,
    minor: Option<u64>,
    patch: Option<u64>,
}

impl VersionReq {
    pub fn major(&self) -> Option<u64> {
        self.major
    }
    pub fn minor(&self) -> Option<u64> {
        self.minor
    }
    pub fn patch(&self) -> Option<u64> {
        self.patch
    }

    pub fn new(major: Option<u64>, minor: Option<u64>, patch: Option<u64>) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parse version requirements from a string, supporting "*", "1", "1.2", "1.2.3" formats
    pub fn parse(req: &str) -> anyhow::Result<Self> {
        if req == "*" {
            return Ok(Self::new(None, None, None));
        }
        let re = regex::Regex::new("^v?(\\d+)(?:\\.(\\d+))?(?:\\.(\\d+))?$").unwrap();
        let caps = re
            .captures(req)
            .context(format!("Version number syntax error: {req}"))?;
        let major = caps.get(1).map(|x| x.as_str().parse::<u64>().unwrap());
        let minor = caps.get(2).map(|x| x.as_str().parse::<u64>().unwrap());
        let patch = caps.get(3).map(|x| x.as_str().parse::<u64>().unwrap());
        Ok(Self::new(major, minor, patch))
    }

    /// Determine if the given Version matches the current requirement
    pub fn matches(&self, v: &Version) -> bool {
        if let Some(major) = self.major() {
            if v.major != major {
                return false;
            }
        }
        if let Some(minor) = self.minor() {
            if v.minor != minor {
                return false;
            }
        }
        if let Some(patch) = self.patch() {
            if v.patch != patch {
                return false;
            }
        }
        true
    }
}
