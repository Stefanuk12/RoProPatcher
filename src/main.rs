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
    background_contents = background_contents.replace("getStorage('rpSubscription')", "\"pro_tier\"");
    background_contents = background_contents.replace("await getStorage(\"rpSubscription\")", "\"pro_tier\"");
    background_contents = background_contents.replace("setStorage(\"rpSubscription\", xhr.getResponseHeader(\"ropro-subscription-tier\"))", "setStorage(\"rpSubscription\", \"pro_tier\"");
    background_contents = background_contents.replace("setStorage(\"rpSubscription\", subscriptionLevel", "setStorage(\"rpSubscription\", \"pro_tier\"");

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

// Main
fn main() {
    // Grab the input directory
    let mut input_dir = String::new();
    print!("Thanks for using Stefanuk12's RoPro Patcher.\n\nPlease select an option:\n\n1. Opera GX\n2. Google Chrome\n3. Custom Path\n> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_dir).ok().expect("Failed to get user input");

    // All of the options
    let path: PathBuf;
    match input_dir.trim() {
        // Opera GX
        "1" => {
            path = fs::read_dir(AppDirs::new(Some(r"Opera Software\Opera GX Stable\Extensions\adbacgifemdbhdkfppmeilbgppmhaobf"), false).unwrap().config_dir).expect("Unable to grab Opera GX extension.").next().unwrap().unwrap().path();
        }
        // Google Chrome
        "2" => { 
            path = fs::read_dir(AppDirs::new(Some(r"Google\Chrome\User Data\Default\Extensions\adbacgifemdbhdkfppmeilbgppmhaobf"), false).unwrap().cache_dir).expect("Unable to grab Google Chrome extension.").next().unwrap().unwrap().path();
        }
        // Custom Path
        "3" => {
            input_dir.clear();
            print!("Please enter the path: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut input_dir).ok().expect("Failed to get user input");
            path = PathBuf::from(input_dir.trim().to_string());
        }
        // Neither of above
        _ => panic!("Invalid option")
    }

    // Patching
    patch(path);
    println!("Patched!");
    pause();
}