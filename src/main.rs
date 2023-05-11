use hasher::settings::{self, SettingsService};

fn main() {
    let settings_service = SettingsService::new("c:/git/rust/hasher/src/settings.json");

    settings_service.clear();
}
