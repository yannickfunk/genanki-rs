use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn guid_for(fields: &Vec<String>) -> String {
    fields
        .iter()
        .map(|f| u64::to_string(&hash_str(&f)))
        .collect()
}

fn hash_str(to_hash: &str) -> u64 {
    let mut s = DefaultHasher::new();
    to_hash.hash(&mut s);
    s.finish()
}
