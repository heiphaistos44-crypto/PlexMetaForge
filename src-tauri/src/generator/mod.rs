use std::path::PathBuf;
use crate::error::Result;

pub fn create_plugin(plugins_dir: &PathBuf, plugin_name: &str) -> Result<String> {
    let safe_name = plugin_name.trim().replace(' ', "_");
    let bundle_path = plugins_dir.join(format!("{}.bundle", safe_name));

    let code_dir = bundle_path.join("Contents").join("Code");
    let resources_dir = bundle_path.join("Contents").join("Resources");
    std::fs::create_dir_all(&code_dir)?;
    std::fs::create_dir_all(&resources_dir)?;

    std::fs::write(
        bundle_path.join("Contents").join("Info.plist"),
        generate_info_plist(&safe_name, plugin_name),
    )?;

    std::fs::write(code_dir.join("__init__.py"), generate_init_py(plugin_name))?;

    Ok(bundle_path.to_string_lossy().to_string())
}

fn generate_info_plist(id: &str, display_name: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>com.plexapp.plugins.{}</string>
    <key>CFBundleName</key>
    <string>{}</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>PlexClientPlatforms</key>
    <string>*</string>
    <key>PlexFrameworkVersion</key>
    <string>2</string>
    <key>PlexPluginMode</key>
    <string>Resident</string>
</dict>
</plist>
"#,
        id.to_lowercase(),
        display_name
    )
}

fn generate_init_py(name: &str) -> String {
    format!(
        r#"# PlexMetaForge — Generated Plugin
# Plugin: {name}

PLUGIN_LOG_TITLE = '{name}'

def Start():
    Log.Info("[{name}] Plugin started")

def ValidatePrefs():
    pass
"#,
        name = name
    )
}
