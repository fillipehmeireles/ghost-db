pub mod processor;
pub mod systemcore;

pub type DarkbirdStorage = darkbird::Storage<std::string::String, std::string::String>;
pub type DbAllData<'a> = dashmap::iter::Iter<'a, std::string::String, std::string::String>;
