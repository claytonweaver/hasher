use std::collections::HashMap;

pub struct SettingsService {
    pub path: String,
}

impl SettingsService {
    pub fn new(path: &str) -> SettingsService {
        SettingsService {
            path: path.to_string(),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let settings_map = self.get_settings();

        match settings_map.get(key) {
            Some(value) => Some(value.to_owned()),
            None => None,
        }
    }

    pub fn set(&self, key: &str, value: &str) {
        let mut settings_map = self.get_settings();
        settings_map.insert(key.to_string(), value.to_string());
        self.save_settings(settings_map);
    }

    pub fn remove_key(&self, key: &str) {
        let mut settings_map = self.get_settings();
        settings_map.remove(key);
        self.save_settings(settings_map);
    }

    pub fn clear(&self) {
        let mut settings_map = self.get_settings();
        settings_map.clear();
        self.save_settings(settings_map);
    }

    fn save_settings(&self, settings_map: HashMap<String, String>) {
        let settings_string = serde_json::to_string(&settings_map)
            .expect("Could not deserialize settings map to string");

        std::fs::write(self.path.to_owned(), settings_string)
            .expect("Could not write settings string to file");
    }

    fn get_settings(&self) -> HashMap<String, String> {
        let settings_string = std::fs::read_to_string(self.path.to_owned())
            .expect(format!("Could not read file with path {}", self.path).as_str());

        let settings_map = serde_json::from_str(&settings_string)
            .expect("Was unable to deserialize string to a hashmap format, check json structure");

        return settings_map;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let settings_service = SettingsService::new("c:/git/rust/hasher/src/settings.json");

        settings_service.set("foo", "bar");

        let foo_value = settings_service.get("foo");

        assert!(foo_value.is_some());
        assert_eq!(foo_value.unwrap(), "bar");
    }

    #[test]
    fn test_clear() {
        let settings_service = SettingsService::new("c:/git/rust/hasher/src/settings.json");

        settings_service.set("foo", "bar");

        let foo_value = settings_service.get("foo");

        assert!(foo_value.is_some());
        assert_eq!(foo_value.unwrap(), "bar");

        settings_service.clear();

        let empty = settings_service.get("foo");

        assert!(empty.is_none());
    }
}
