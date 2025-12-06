use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn guid_for(fields: &[String]) -> String {
    fields.iter().map(|f| hash_str(f).to_string()).collect()
}

fn hash_str(to_hash: &str) -> u64 {
    let mut s = DefaultHasher::new();
    to_hash.hash(&mut s);
    s.finish()
}
