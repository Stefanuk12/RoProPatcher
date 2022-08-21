// Dependencies
use std::{fs, io::{stdin, stdout, Write}};
use platform_dirs::AppDirs;

// Path to the installation root dir
fn patch(path: String) {
    // Patching the background file
    let background = path.clone() + "/background.js";

    // Grabbing background file contents
    let mut background_contents = fs::read_to_string(background.clone()).expect("Unable to open file (background.js)");

    // Finding the things to patch
    background_contents = background_contents.replace("subscription = await getStorage(\"rpSubscription\")", "subscription = \"pro_tier\"");
    background_contents = background_contents.replace("subscription = data", "data = \"pro_tier\"; subscription = data");
    background_contents = background_contents.replace("getStorage('rpSubscription')", "\"pro_tier\"");
    background_contents = background_contents.replace("await getStorage(\"rpSubscription\")", "\"pro_tier\"");
    background_contents = background_contents.replace("setStorage(\"rpSubscription\", xhr.getResponseHeader(\"ropro-subscription-tier\"))", "setStorage(\"rpSubscription\", \"pro_tier\"");
    background_contents = background_contents.replace("setStorage(\"rpSubscription\", subscriptionLevel", "setStorage(\"rpSubscription\", \"pro_tier\"");

    // Write our changes
    fs::write(background.clone(), background_contents).expect("Unable to write file contents (background.js)");

    // Patching the options file
    let options = path.clone() + "/js/page/options.js";

    // Grabbing options file contents
    let mut options_contents = fs::read_to_string(options.clone()).expect("Unable to open file (options.js)");

    // Finding things to patch
    options_contents = options_contents.replace("setStorage('rpSubscription', data)", "data=\"pro_tier\"; setStorage('rpSubscription', data)");

    // Write our changes
    fs::write(options.clone(), options_contents).expect("Unable to write file contents (options.js)");
}

// Main
fn main() {
    // Grab the input directory
    let mut input_dir = String::new();
    print!("Thanks for using Stefanuk12's RoPro Patcher.\n\nPlease select an option:\n\n1. Opera GX\n2. Google Chrome\n3. Custom Path\n> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_dir).ok().expect("Failed to get user input");

    // Resolve what we want to do
    input_dir = input_dir.trim().to_string();
    if !(input_dir == "1" || input_dir == "2" || input_dir == "3") {
        panic!("Invalid option");
    }

    // Custom Path
    if input_dir == "3" {
        input_dir.clear();
        print!("Please enter the path: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input_dir).ok().expect("Failed to get user input");
        input_dir = input_dir.trim().to_string();
    } else {
        // Opera GX
        if input_dir == "1" {
            input_dir.clear();
            input_dir = fs::read_dir(AppDirs::new(Some(r"Opera Software\Opera GX Stable\Extensions\adbacgifemdbhdkfppmeilbgppmhaobf"), false).unwrap().config_dir).expect("Unable to grab Opera GX extension.").next().unwrap().unwrap().path().to_str().unwrap().to_string();
        } else if input_dir == "2" { // Google Chrome
            input_dir.clear();
            input_dir = fs::read_dir(AppDirs::new(Some(r"Google\Chrome\User Data\Default\Extensions\adbacgifemdbhdkfppmeilbgppmhaobf"), false).unwrap().cache_dir).expect("Unable to grab Google Chrome extension.").next().unwrap().unwrap().path().to_str().unwrap().to_string();
        }
    }

    //
    patch(input_dir.to_string());
    println!("Patched! Press enter to exit.");
    stdin();
}