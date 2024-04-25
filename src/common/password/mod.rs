mod hash_type;
mod password;
mod ssha;

pub use password::Password;
pub use hash_type::Hash;

pub const DEFAULT_HASH: Hash = Hash::SSHA;