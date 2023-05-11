use hasher::settings::{self, SettingsService};

fn main() {
    let settings_service = SettingsService::new("./src/settings.json");

    settings_service.clear();

    settings_service.set("test", "123");

    let t = settings_service.get("test");
}
