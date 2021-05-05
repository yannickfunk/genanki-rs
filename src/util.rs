use uuid::Uuid;

pub fn guid_for(fields: &Vec<String>) -> String {
    Uuid::new_v4().to_string()
}
