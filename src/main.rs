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
fn patch(path: PathBuf, _proxy: Option<String>) {
    // Ask for the proxy url
    let mut proxy = _proxy.unwrap_or("N/A".to_owned());
    if proxy == "N/A" {
        // Ask for the proxy domain, default it to ours
        proxy = String::new();
        print!("Please enter the proxy domain (ropro-proxy.deno.dev): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut proxy).ok().expect("Failed to get user input");

        // Check if input was blank
        if proxy.trim().len() == 0 {
            proxy = "ropro-proxy.deno.dev".to_string();
        }
    }

    // The regex replace thing. We don't want to proxy everything, only the stuff that needs verification
    let re = regex::Regex::new(r#"(https://api\.)ropro\.io/(validateUser\.php|getServerInfo\.php|getServerConnectionScore\.php|getServerAge\.php|getSubscription\.php\?key=)"#).unwrap();
    let rep = format!("https://{}/${{2}}!api", proxy);

    // Patching the background file
    let background = path.join("background.js");
    let mut background_contents = fs::read_to_string(&background).expect("Unable to open file (background.js)");
    background_contents = re.replace_all(&background_contents, &rep).to_string();
    fs::write(&background, background_contents).expect("Unable to write file contents (background.js)");

    // Patching each file in js/page
    let jspage = path.clone().join("js/page");
    for dir_entry in fs::read_dir(jspage).unwrap() {
        let file = dir_entry.unwrap();
        let file_name = format!("js/page/{}", file.file_name().to_str().unwrap());
        let file_path = file.path();
    
        let mut file_data = fs::read_to_string(file_path.clone()).expect(&format!("Unable to open file ({})", file_name));
        file_data = re.replace_all(&file_data, &rep).to_string();
        fs::write(file_path, file_data).expect(&format!("Unable to write file contents ({})", file_name));
    }
}

// Main
fn main() {
    // Grab the input directory
    let mut input_dir = String::new();
    print!("Thanks for using Stefanuk12's RoPro Patcher.\n\nPlease select an option:\n\n1. Opera GX\n2. Custom Path\n> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_dir).ok().expect("Failed to get user input");

    // All of the options
    let path: PathBuf;
    match input_dir.trim() {
        // Opera GX
        "1" => {
            // Grab path
            path = fs::read_dir(AppDirs::new(Some(r"Opera Software\Opera GX Stable\Extensions\adbacgifemdbhdkfppmeilbgppmhaobf"), false).unwrap().config_dir).expect("Unable to grab Opera GX extension.").next().unwrap().unwrap().path();
    
            // Patch
            patch(path, None);
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

            patch(path, None);
            println!("Patched!");
            pause();
        }
        // Neither of above
        _ => panic!("Invalid option")
    }
}