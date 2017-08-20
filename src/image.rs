// args may be 0 for selection screenshots
// 1 for window screenshots, or 2 for fullscreenshots

#![allow(dead_code)]

use std::time::Duration;
use std::process::Command;
use std::path::PathBuf;
use std::{env, fs, process, thread};
use notification;
use error;
use save;
use desktop;

fn screenshot(args: usize, temp: &str, session: String, desktop: String) {

    match session.as_ref() {
        "wayland" => {
            match desktop.as_ref() {
                "gnome" => gnome(args, temp),
                "cinnamon" => gnome(args, temp),
                "budgie-desktop" => gnome(args, temp),
                "budgie:gnome" => gnome(args, temp),
                "plasma" => kde(args, temp),
                "kde" => kde(args, temp),
                "sway" => sway(args, temp),
                _ => {
                    eprintln!("{}", error::message(26));
                    notification::error(26);
                    error::fatal()
                }
            }
        }
        "x11" => {
            match desktop.as_ref() {
                "gnome" => gnome(args, temp),
                "cinnamon" => gnome(args, temp),
                "x-cinnamon" => gnome(args, temp),
                "ubuntu" => gnome(args, temp),
                "unity:unity7" => gnome(args, temp),
                "budgie-desktop" => gnome(args, temp),
                "budgie:gnome" => gnome(args, temp),
                "plasma" => kde(args, temp),
                "kde" => kde(args, temp),
                _ => scrot(args, temp),
            }
        }
        _ => {
            match desktop.as_ref() {
                "gnome" => gnome(args, temp),
                "cinnamon" => gnome(args, temp),
                "x-cinnamon" => gnome(args, temp),
                "ubuntu" => gnome(args, temp),
                "unity:unity7" => gnome(args, temp),
                "budgie-desktop" => gnome(args, temp),
                "budgie:gnome" => gnome(args, temp),
                "plasma" => kde(args, temp),
                "kde" => kde(args, temp),
                "macos" => mac(args, temp),
                _ => scrot(args, temp),
            }
        }
    }
}

fn mac(args: usize, temp: &str) {

    if args == 0 {

        // _image uses screencapture to take area
        let _image = match Command::new("screencapture")
            .args(&["-s", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(8));
                notification::error(8);
                error::fatal()
            }
        };

    } else if args == 1 {

        // _image uses screencapture to get screenshot of current window
        let _image = match Command::new("screencapture")
            .args(&["-w", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(8));
                notification::error(8);
                error::fatal()
            }
        };

    } else if args == 2 {

        // _image uses screencapture to take screenshot
        let _image = match Command::new("screencapture")
            .args(&["-S", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(8));
                notification::error(8);
                error::fatal()
            }
        };
    }
}

fn sway(args: usize, temp: &str) {

    if args == 0 {

        // makes filename for temporary temporary file
        let _tmp = temp_dir(1);
        let tmp = _tmp.to_str().unwrap().clone();

        // _before_image takes a full screenshot using swaygrab
        match Command::new("swaygrab").arg(&tmp).output() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(9));
                notification::error(9);
                error::fatal()
            }
        };

        // _feh displays it
        println!("Feh may not display properly due to tiling and Wayland.");

        let mut _feh = match Command::new("feh").args(&[&tmp, "-F"]).spawn() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(12));
                notification::error(12);
                error::fatal()
            }
        };

        // _image lets use _slop to select
        let _slop = match Command::new("slop").output() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(14));
                notification::error(14);
                error::fatal()
            }
        };
        let slop = String::from_utf8_lossy(&_slop.stdout);
        let _image = match Command::new("convert")
            .args(&[temp.clone(), "-crop", &slop, temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(15));
                notification::error(15);
                error::fatal()
            }
        };

        // closes _feh, gently
        match _feh.kill() {
            Ok(ok) => ok,
            Err(_) => return,
        };

        match fs::remove_file(tmp) {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(0));
                notification::error(0);
                return;
            }
        };

        if _image.code() == Some(1) {
            process::exit(1);
        }

    } else if args == 1 {

        // _image uses swaygrab to get "focused" window and take screenshot
        let _image = match Command::new("swaygrab")
            .args(&["-f", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(9));
                notification::error(9);
                error::fatal()
            }
        };

        if _image.code() == Some(1) {
            process::exit(1);
        }
    } else if args == 2 {

        // _image uses swaygrab to take screenshot
        let _image = match Command::new("swaygrab").arg(temp.clone()).status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(9));
                notification::error(9);
                error::fatal()
            }
        };

        if _image.code() == Some(1) {
            process::exit(1);
        }
    }
}

fn gnome(args: usize, temp: &str) {

    if args == 0 {

        // makes filename for temporary temporary file
        let _tmp = temp_dir(1);
        let tmp = _tmp.to_str().unwrap().clone();

        // _before_image takes a full screenshot using gnome0creenshot
        match Command::new("gnome-screenshot")
            .args(&["-f", &tmp])
            .output() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(7));
                notification::error(7);
                error::fatal()
            }
        };

        // _feh displays it
        let mut _feh = match Command::new("feh").args(&[&tmp, "-F"]).spawn() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(12));
                notification::error(12);
                error::fatal()
            }
        };

        // _image lets you select
        let _image = match Command::new("gnome-screenshot")
            .args(&["-a", "-f", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(7));
                notification::error(7);
                error::fatal()
            }
        };

        match fs::remove_file(tmp) {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(0));
                notification::error(0);
                return;
            }
        };

        // closes _feh, gently
        match _feh.kill() {
            Ok(ok) => ok,
            Err(_) => return,
        };

    } else if args == 1 {

        // _image uses gnome-screenshot to get current window and take screenshot
        let _image = match Command::new("gnome-screenshot")
            .args(&["-w", "-e", "shadow", "-f", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(7));
                notification::error(7);
                error::fatal()
            }
        };
    } else if args == 2 {

        // _image uses gnome-screenshot to take screenshot
        let _image = match Command::new("gnome-screenshot")
            .args(&["-f", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(7));
                notification::error(7);
                error::fatal()
            }
        };
    }
}

fn kde(args: usize, temp: &str) {

    if args == 0 {

        // _image pauses screen and lets you select
        let _image = match Command::new("spectacle")
            .args(&["-rbno", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(8));
                notification::error(8);
                error::fatal()
            }
        };

    } else if args == 1 {

        // _image uses spectacle to get current window and take screenshot
        let _image = match Command::new("spectacle")
            .args(&["-abno", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(8));
                notification::error(8);
                error::fatal()
            }
        };

    } else if args == 2 {

        // _image uses spectacle to take screenshot
        let _image = match Command::new("spectacle")
            .args(&["-fbno", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(8));
                notification::error(8);
                error::fatal()
            }
        };
    }
}

fn scrot(args: usize, temp: &str) {

    if args == 0 {

        // makes filename for temporary temporary file
        let _tmp = temp_dir(1);
        let tmp = _tmp.to_str().unwrap().clone();

        // _before_image takes a full screenshot using scrot
        match Command::new("scrot").arg(&tmp).output() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(10));
                notification::error(10);
                error::fatal()
            }
        };

        // _feh displays it
        let mut _feh = match Command::new("feh").args(&[&tmp, "-F"]).spawn() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(12));
                notification::error(12);
                error::fatal()
            }
        };

        // _image lets you select
        let _image = match Command::new("scrot")
            .args(&["--select", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(10));
                notification::error(10);
                error::fatal()
            }
        };

        // closes _feh, gently
        match _feh.kill() {
            Ok(ok) => ok,
            Err(_) => return,
        };

        match fs::remove_file(tmp) {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(0));
                notification::error(0);
                return;
            }
        };

        if _image.code() == Some(2) {
            process::exit(1);
        }

    } else if args == 1 {

        // _image uses scrot to take window screenshot
        let _image = match Command::new("scrot")
            .args(&["--border", "--focused", temp.clone()])
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(10));
                notification::error(10);
                error::fatal()
            }
        };

    } else if args == 2 {

        // _image uses scrot to take screenshot
        let _image = match Command::new("scrot").arg(temp.clone()).status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(10));
                notification::error(10);
                error::fatal()
            }
        };
    }
}

pub fn image(args: usize) {

    // tmp gets the temporary directory of the system
    let tmp = temp_dir(0);

    // makes a string
    let temp = tmp.to_str().unwrap();

    // x11/wayland session info gotten here
    let session = desktop::session();

    // desktop environment info gotten here
    let desktop = desktop::desktop();

    screenshot(args.clone(), temp.clone(), session.clone(), desktop.clone());

    if !tmp.is_file() {
        eprintln!("{}", error::message(30));
        error::fatal();
    }

    if args == 1 && desktop != "gnome" && !cfg!(target_os = "macos") {
        //  adds a shadow
        match Command::new("convert")
            .arg(temp.clone())
            .args(
                &[
                    "(",
                    "+clone",
                    "-background",
                    "black",
                    "-shadow",
                    "80x3+5+5",
                    ")",
                    "+swap",
                    "-background",
                    "none",
                    "-layers",
                    "merge",
                    "+repage",
                ],
            )
            .arg(temp.clone())
            .status() {
            Ok(ok) => ok,
            Err(_) => {
                eprintln!("{}", error::message(13));
                notification::error(13);
                error::fatal()
            }
        };
        thread::sleep(Duration::new(0, 500000000));
    }

    save::save();
}

pub fn temp_dir(option: usize) -> PathBuf {
    let mut tmp = env::temp_dir();
    if option == 0 {
        tmp.push("sharexin.png");
    } else {
        tmp.push("sharexin-tmp");
        tmp.set_extension("png");
    }
    return tmp;
}
