use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use rand_pcg::Pcg64;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Seed {
    value: u64,
}

impl Seed {
    pub fn new(num: u64) -> Self {
        Self {
            value: num,
        }
    }

    fn from_str<S: AsRef<str>>(str: S) -> Self {
        let mut hasher = DefaultHasher::new();
        str.as_ref().hash(&mut hasher);

        Self {
            value: hasher.finish(),
        }
    }
}

impl Into<Pcg64> for Seed {
    fn into(self) -> Pcg64 {
        use rand::SeedableRng;
        Pcg64::seed_from_u64(self.value)
    }
}

impl Display for Seed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl From<String> for Seed {
    fn from(string: String) -> Self {
        Seed::from_str(string)
    }
}

impl From<&str> for Seed {
    fn from(str: &str) -> Self {
        Seed::from_str(str)
    }
}

impl From<u64> for Seed {
    fn from(num: u64) -> Self {
        Seed::new(num)
    }
}

impl From<u32> for Seed {
    fn from(num: u32) -> Self {
        Seed::new(num as u64)
    }
}

impl From<u16> for Seed {
    fn from(num: u16) -> Self {
        Seed::new(num as u64)
    }
}
