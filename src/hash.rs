// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

use std::fmt::{self, Debug, Formatter};

pub const HASH_LEN: usize = 32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Hash([u8; HASH_LEN]);

impl Hash {
    #[cfg(any(all(test, feature = "mock"), feature = "testing"))]
    pub const ZERO: Self = Hash([0; HASH_LEN]);

    #[cfg(any(all(test, feature = "mock"), feature = "testing"))]
    pub fn from_bytes(bytes: [u8; HASH_LEN]) -> Self {
        Hash(bytes)
    }

    #[cfg(feature = "mock")]
    pub fn as_bytes(&self) -> &[u8; HASH_LEN] {
        &self.0
    }
}

impl<'a> From<&'a [u8]> for Hash {
    fn from(src: &'a [u8]) -> Self {
        Hash(tiny_keccak::sha3_256(src))
    }
}

impl Debug for Hash {
    #[cfg(any(test, feature = "testing", feature = "dump-graphs"))]
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "{:02x}{:02x}{:02x}{:02x}{:02x}..",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4]
        )
    }

    #[cfg(not(any(test, feature = "testing", feature = "dump-graphs")))]
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "{}", self::full::FullDisplay(self))
    }
}

#[cfg(any(feature = "dump-graphs", not(any(test, feature = "testing"))))]
mod full {
    use super::*;
    use std::fmt::Display;

    #[cfg(feature = "dump-graphs")]
    impl Hash {
        pub fn full_display(&self) -> FullDisplay {
            FullDisplay(self)
        }
    }

    pub struct FullDisplay<'a>(pub &'a Hash);

    impl<'a> Display for FullDisplay<'a> {
        fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
            for byte in &(self.0).0 {
                write!(formatter, "{:02x}", byte)?;
            }
            Ok(())
        }
    }
}
