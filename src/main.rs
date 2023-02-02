extern crate winreg;
use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    println!("Toggling dark mode");
    let hklm = RegKey::predef(HKEY_CURRENT_USER);
    let personalize = hklm.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize",
        KEY_ALL_ACCESS)?;
    // SystemUsesLightTheme is used for taskbar color
    let app_mode: u32 = personalize.get_value("AppsUseLightTheme")?;
    let new_app_mode = if app_mode == 0 { 0000000001u32 } else { 0000000000u32 };
    personalize.set_value("AppsUseLightTheme", &new_app_mode)?;
    Ok(())
}
