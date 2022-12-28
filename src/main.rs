// Dependencies
use std::{fs, io::{stdin, stdout, Write, Read}, path::PathBuf};
use platform_dirs::AppDirs;

// Pauses the application until userinput
fn pause() {
    let mut stdin = stdin();
    let mut stdout = stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

// Path to the installation root dir
fn patch(path: PathBuf) {
    // Patching the background file
    let background = path.join("background.js");

    // Grabbing background file contents
    let mut background_contents = fs::read_to_string(background.clone()).expect("Unable to open file (background.js)");

    // Finding the things to patch
    background_contents = background_contents.replace("subscription = await getStorage(\"rpSubscription\")", "subscription = \"pro_tier\"");
    background_contents = background_contents.replace("subscription = data", "data = \"pro_tier\"; subscription = data");
    background_contents = background_contents.replace("getStorage('rpSubscription')", "new Promise(resolve => resolve(\"pro_tier\"))");
    background_contents = background_contents.replace("await getStorage(\"rpSubscription\")", "\"pro_tier\"");
    background_contents = background_contents.replace("setStorage(\"rpSubscription\", xhr.getResponseHeader(\"ropro-subscription-tier\"))", "setStorage(\"rpSubscription\", \"pro_tier\")");
    background_contents = background_contents.replace("setStorage(\"rpSubscription\", subscriptionLevel)", "setStorage(\"rpSubscription\", \"pro_tier\")");

    // Write our changes
    fs::write(background.clone(), background_contents).expect("Unable to write file contents (background.js)");

    // Patching the options file
    let options = path.join("js/page/options.js");

    // Grabbing options file contents
    let mut options_contents = fs::read_to_string(options.clone()).expect("Unable to open file (options.js)");

    // Finding things to patch
    options_contents = options_contents.replace("setStorage('rpSubscription', data)", "data=\"pro_tier\"; setStorage('rpSubscription', data)");

    // Write our changes
    fs::write(options.clone(), options_contents).expect("Unable to write file contents (options.js)");
}

// Converts Roblox cookie to CSRF
fn grab_csrf(cookie: String) -> String {
    // Create the request
    let client = reqwest::blocking::Client::new();
    let response = client.post("https://catalog.roblox.com/v1/catalog/items/details")
        .header("Cookie", format!(".ROBLOSECURITY={};", cookie))
        .header("content-length", 0)
        .send()
        .unwrap();

    // Return header
    let csrf = response.headers()["x-csrf-token"].to_str().unwrap().to_owned();
    return csrf;
}

// Grab verification metadata from RoPro
#[derive(serde::Deserialize)]
struct Verification {
    universeId: i64
}
fn grab_verification_md() -> i64 {
    // Create the request
    let client = reqwest::blocking::Client::new();
    let response: Verification = client.post("https://api.ropro.io/verificationMetadata.php")
        .header("content-length", 0)
        .send()
        .unwrap()
        .json()
        .unwrap();

    // Return
    return response.universeId;
}

// (Un)favourite a Roblox universe
fn favourite_universe(cookie: String, csrf: String, universe_id: i64, unfavourite: bool) {
    // Create the request
    let client = reqwest::blocking::Client::new();
    let mut data = std::collections::HashMap::new();
    data.insert("isFavorited", !unfavourite);
    let response = client.post(format!("https://games.roblox.com/v1/games/{}/favorites", universe_id))
        .header("Cookie", format!(".ROBLOSECURITY={};", cookie))
        .header("x-csrf-token", csrf)
        .json(&data)
        .send();

    // Check for error
    if response.is_err() {
        println!("Unable to (un)favourite universe for unknown reason")
    }
}

// Get the verification token
#[derive(serde::Deserialize)]
struct VerificationResponse {
    success: bool,
    error_code: Option<i8>,
    token: Option<String>
}
fn get_verification() -> String {
    // Create the request
    let client = reqwest::blocking::Client::new();
    let response: VerificationResponse = client.post("https://api.ropro.io/generateVerificationToken.php")
        .header("content-length", 0)
        .send()
        .unwrap()
        .json()
        .unwrap();

    // Return
    if response.success == false {
        return response.error_code.unwrap().to_string();
    } else {
        return response.token.unwrap();
    }
}

// Main
fn main() {
    // Grab the input directory
    let mut input_dir = String::new();
    print!("Thanks for using Stefanuk12's RoPro Patcher.\n\nPlease select an option:\n\n0. Generate RoPro Verification Token\n1. Opera GX\n2. Custom Path\n> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_dir).ok().expect("Failed to get user input");

    // All of the options
    let path: PathBuf;
    match input_dir.trim() {
        // Generate RoPro Verification Token
        "0" => {
            // Ask for their Roblox cookie
            let mut cookie = String::new();
            cookie.clear();
            print!("Please enter your Roblox cookie (without the '.ROBLOSECURITY=' part)\n> ");
            stdout().flush().unwrap();
            stdin().read_line(&mut cookie).ok().expect("Failed to get user input");
            cookie = cookie.trim().to_string();

            // Resolve to CSRF
            let csrf = grab_csrf(cookie.to_owned());
            
            // Favourite, grab token, unfavourite
            let universe_id = grab_verification_md();
            favourite_universe(cookie.to_owned(), csrf.to_owned(), universe_id, false);
            let token = get_verification();
            favourite_universe(cookie.to_owned(), csrf.to_owned(), universe_id, true);

            // Check
            if token.len() == 25 {
                println!("Got your RoPro verification token: {}", token);
            } else {
                println!("There was an issue with getting your token (Error {})", token)
            }

            //
            pause();
        }
        // Opera GX
        "1" => {
            path = fs::read_dir(AppDirs::new(Some(r"Opera Software\Opera GX Stable\Extensions\adbacgifemdbhdkfppmeilbgppmhaobf"), false).unwrap().config_dir).expect("Unable to grab Opera GX extension.").next().unwrap().unwrap().path();
            
            patch(path);
            println!("Patched!");
            pause();
        }
        // Custom Path
        "2" => {
            input_dir.clear();
            print!("Please enter the path: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut input_dir).ok().expect("Failed to get user input");
            path = PathBuf::from(input_dir.trim().to_string());

            patch(path);
            println!("Patched!");
            pause();
        }
        // Neither of above
        _ => panic!("Invalid option")
    }
}