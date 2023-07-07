// Dependencies
use std::{fs, io::{stdin, stdout, Write}, path::PathBuf};
use platform_dirs::AppDirs;
use console::Term;

/// Pauses the application until the user presses any key.
fn pause() {
    let term = Term::stdout();
    term.write_line("Press any key to continue...").unwrap();
    term.read_key().unwrap();
    term.clear_screen().unwrap();
}

fn patch(path: PathBuf, _proxy: Option<String>) -> String {
    // Ask for the proxy url
    let mut proxy = _proxy.unwrap_or("N/A".to_owned());
    if proxy == "N/A" {
        // Ask for the proxy domain, default it to ours
        proxy = String::new();
        print!("Please enter the proxy domain (either a number or a custom input):\n1. ropro-proxy.deno.dev (default)\n2. ropro.darkhub.cloud\n3. ropro.synapse.rocks\n> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut proxy).ok().expect("Failed to get user input");

        // Set
        proxy = match proxy.trim() {
            "" => "ropro-proxy.deno.dev".to_owned(),
            "1" => "ropro-proxy.deno.dev".to_owned(),
            "2" => "ropro.darkhub.cloud".to_owned(),
            "3" => "ropro.synapse.rocks".to_owned(),
            _ => proxy
        };

        // Trim
        proxy = proxy.trim().to_string();
    }

    // The regex replace thing. We don't want to proxy everything, only the stuff that needs verification
    let re = regex::Regex::new(r#"(https://api\.)ropro\.io/(validateUser\.php|getServerInfo\.php|getServerConnectionScore\.php|getServerAge\.php|getSubscription\.php)"#).unwrap();
    let rep = format!("https://{}/${{2}}///api", proxy);

    // Patching the background file
    let background = path.join("background.js");
    let background_contents = fs::read_to_string(&background).expect("Unable to open file (background.js)");
    let new_background_contents = re.replace_all(&background_contents, &rep).to_string();
    fs::write(&background, new_background_contents.clone()).expect("Unable to write file contents (background.js)");

    // Checking if they changed
    if background_contents == new_background_contents {
        println!("warning: nothing changed while patching `background.js` (and possibly others within js/page) - already patched?");
    }

    // Patching each file in js/page
    let jspage = path.clone().join("js/page");
    for dir_entry in fs::read_dir(jspage).unwrap() {
        // Get the file path
        let file = dir_entry.unwrap();
        let file_name = format!("js/page/{}", file.file_name().to_str().unwrap());
        let file_path = file.path();
    
        // Patch the file
        let file_data = fs::read_to_string(file_path.clone()).expect(&format!("Unable to open file ({})", file_name));
        let new_file_data = re.replace_all(&file_data, &rep).to_string();
        fs::write(file_path.clone(), new_file_data.clone()).expect(&format!("Unable to write file contents ({})", file_name));
    }

    // Done
    proxy
}

fn main() {
    // Grab the input directory
    let mut input_dir = String::new();
    print!("Thanks for using Stefanuk12's RoPro Patcher.\n\nPlease select an option:\n1. Opera GX\n2. Custom Path\n> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_dir).ok().expect("Failed to get user input");

    // Grab the path we want to use
    let path: PathBuf;
    match input_dir.trim() {
        // Opera GX
        "1" => {
            // Grab path
            path = fs::read_dir(AppDirs::new(Some(r"Opera Software\Opera GX Stable\Extensions\adbacgifemdbhdkfppmeilbgppmhaobf"), false).unwrap().config_dir).expect("Unable to grab Opera GX extension.").next().unwrap().unwrap().path();
        }
        // Custom Path
        "2" => {
            input_dir.clear();
            print!("Please enter the path: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut input_dir).ok().expect("Failed to get user input");
            path = PathBuf::from(input_dir.trim().to_string());
        }
        // Neither of above
        _ => panic!("Invalid option")
    }

    // Patching...
    let proxy = patch(path.clone(), None);
    println!("Patched with the following configuration:\n-> Path: {}\n-> Proxy: {}", path.display(), proxy);
    pause();
}
