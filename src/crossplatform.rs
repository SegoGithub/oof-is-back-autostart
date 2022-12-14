#[cfg(target_os = "windows")]
pub fn check_ver() {
    static USERNAME: &str = "USERNAME";
    // list all the files in %localappdata%\Roblox\Versions
    let mut versions = vec![];
    for entry in fs::read_dir(
        "C:\\Users\\".to_string()
            + &env::var(USERNAME).unwrap()
            + "\\AppData\\Local\\Roblox\\Versions",
    )
    .unwrap()
    {
        versions.push(entry.unwrap().path().to_str().unwrap().to_string());
    }
    // sort the list of versions
    versions.sort();
    // reverse the list of versions
    versions.reverse();
    // print the list of versions
    for version in versions {
        // check if RobloxPlayerBeta.exe exists
        if fs::metadata(version.clone() + "\\RobloxPlayerBeta.exe").is_ok() {
            // check if .ouch exists
            if fs::metadata(version.clone() + "\\content\\sounds\\.ouch").is_err() {
                // delete ouch.ogg
                fs::remove_file(version.clone() + "\\content\\sounds\\ouch.ogg").unwrap();
                // copy %appdata%\oof-is-back\ouch.ogg to version\content\sounds\ouch.ogg
                fs::copy(
                    "C:\\Users\\".to_string()
                        + &env::var(USERNAME).unwrap()
                        + "\\AppData\\Roaming\\oof-is-back\\ouch.ogg",
                    version.clone() + "\\content\\sounds\\ouch.ogg",
                );
                // make an empty file named ouch.ogg
                fs::File::create(version.clone() + "\\content\\sounds\\.ouch").unwrap();
                // windows notification
                Notification::new()
                    .summary("Oof is back!")
                    .body("Your oof sound was replaced by Roblox Updates. Please restart Roblox for the old oof sound to take effect.")
                    .icon(&("C:\\Users\\".to_string()
                        + &env::var(USERNAME).unwrap()
                        + "\\AppData\\Roaming\\oof-is-back\\icon.png")
                    )
                    .timeout(0) // this however is
                    .show();
            }
        }
    }
}




#[cfg(target_os = "linux")]
pub fn check_ver() {
    // list all the files in %localappdata%\Roblox\Versions

    use std::fs;

    use notify_rust::Notification;
    let mut versions = vec![];
    for entry in fs::read_dir(
        "/home/".to_string() + &whoami::username() + &"/.local/share/grapejuice/prefixes/player/drive_c/users/".to_string() + &whoami::username() + &"/AppData/Local/Roblox/Versions".to_string(),
    )
    .unwrap()
    {
        versions.push(entry.unwrap().path().to_str().unwrap().to_string());
    }
    // sort the list of versions
    versions.sort();
    // reverse the list of versions
    versions.reverse();
    // print the list of versions
    for version in versions {
        // check if RobloxPlayerBeta.exe exists
        if fs::metadata(version.clone() + "/RobloxPlayerBeta.exe").is_ok() {
            // check if .ouch exists
            if fs::metadata(version.clone() + "/content/sounds/.ouch").is_err() {
                // delete ouch.ogg
                fs::remove_file(version.clone() + "/content/sounds/ouch.ogg").unwrap();
                // copy %appdata%\oof-is-back\ouch.ogg to version\content\sounds\ouch.ogg
                fs::copy(
                    "/home/".to_string() + &whoami::username() + &"/.oof-is-back/ouch.ogg".to_string(),
                    version.clone() + &"/content/sounds/ouch.ogg".to_string(),
                ).unwrap();
                // make an empty file named .ouch
                fs::File::create(version.clone() + "/content/sounds/.ouch").unwrap();
                // windows notification
                Notification::new()
                        .summary("Oof is back!")
                        .body("Your oof sound was replaced by Roblox Updates. Please restart Roblox for the old oof sound to take effect.")
                        .icon(&("/home/".to_string() + &whoami::username() + &"/.oof-is-back/icon.png".to_string()))
                        .timeout(0)
                        .show().unwrap();
            }
        }
    }
}