use notify_rust::Notification;
use std::{env, fs, time::Duration, thread::sleep};

fn main() {
    print!("Welcome to Oof is back! rust (autostart)\nThis app prevents Roblox Updates from changing your oof sound");
    println!("\nYou can minimize this windows, but dont close it.");

    loop {
        check_ver();
        sleep(Duration::from_secs(30));
    }

    fn check_ver() {
        // list all the files in %localappdata%\Roblox\Versions
        let mut versions = vec![];
        for entry in fs::read_dir(
            "C:\\Users\\".to_string()
                + &env::var("USERNAME").unwrap()
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
                    // print the version
                    println!("Oof sound was replaced by Roblox Updates");
                    // delete ouch.ogg
                    fs::remove_file(version.clone() + "\\content\\sounds\\ouch.ogg").unwrap();
                    // copy %appdata%\oof-is-back\ouch.ogg to version\content\sounds\ouch.ogg
                    fs::copy(
                        "C:\\Users\\".to_string()
                            + &env::var("USERNAME").unwrap()
                            + "\\AppData\\Roaming\\oof-is-back\\ouch.ogg",
                        version.clone() + "\\content\\sounds\\ouch.ogg",
                    );
                    // make an empty file named ouch.ogg
                    fs::File::create(version.clone() + "\\content\\sounds\\.ouch").unwrap();
                    // windows notification
                    println!("Oof sound replaced with old one, game restart required");
                    Notification::new()
                        .summary("Oof is back!")
                        .body("Your oof sound was replaced by Roblox Updates. Please restart Roblox for the old oof sound to take effect.")
                        .icon(&("C:\\Users\\".to_string()
                            + &env::var("USERNAME").unwrap()
                            + "\\AppData\\Roaming\\oof-is-back\\icon.png")
                        )
                        .timeout(0) // this however is
                        .show();
                }
            }
        }
    }
}
