// Dependencies
use terminal_menu::{run, menu, label, scroll, string, submenu, back_button, button, mut_menu, list};
use std::{path::PathBuf, fs::{self, File}, io::{Cursor, Write}};
use platform_dirs::AppDirs;
use crx_dl::{ChromeCRXQuery, crx_to_zip};

// Constants
const PROXIES_URL: &str = "https://raw.githubusercontent.com/Stefanuk12/RoProPatcher/master/proxies.txt";

/// Fetches each proxy.
fn get_proxies() -> Vec<String> {
    reqwest::blocking::Client::new()
        .get(PROXIES_URL)
        .send()
        .expect("unable to grab proxies")
        .text()
        .expect("invalid proxies")
        .lines()
        .map(|x| x.to_string())
        .collect()
}

/// Performs the entire patching process.
fn patch(path: PathBuf, proxy: String) {
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
    let jspage = path.join("js/page");
    for dir_entry in fs::read_dir(jspage).unwrap() {
        // Get the file path
        let file = dir_entry.unwrap();
        let file_name = format!("js/page/{}", file.file_name().to_str().unwrap());
        let file_path = file.path();
    
        // Patch the file
        let file_data = fs::read_to_string(file_path.clone()).unwrap_or_else(|_| panic!("Unable to open file ({})", file_name));
        let new_file_data = re.replace_all(&file_data, &rep).to_string();
        fs::write(file_path.clone(), new_file_data.clone()).unwrap_or_else(|_| panic!("Unable to write file contents ({})", file_name));
    }
}

/// Downloads RoPro source.
fn download_extension() -> Vec<u8> {
    // Download the extension
    let mut crx_query = ChromeCRXQuery::default();
    crx_query.x = "adbacgifemdbhdkfppmeilbgppmhaobf";
    let extension_crx = crx_query.download_blocking().unwrap();

    // Convert it to .zip
    let crx_zip = crx_to_zip(extension_crx, None).unwrap();

    // Done
    crx_zip
}

/// Downloads RoPro source, then output to file as `.zip`.
fn download_extract() {
    // Download the extension's source
    let extension_source = download_extension();

    // Output to file
    let mut file_out = File::create(format!("{}.zip", "RoPro")).unwrap();
    file_out.write_all(&extension_source).unwrap();

    // Output
    println!("Downloaded RoPro.");
}

/// Downloads RoPro source and patches automatically.
fn download_patch(selected_proxy: String) {
    // Download the extension's source
    let extension_source = download_extension();

    // Extract the extension
    let extract_dir = PathBuf::from("RoPro");
    zip_extract::extract(Cursor::new(extension_source), &extract_dir, true).unwrap();

    // Patch
    patch(extract_dir, selected_proxy.to_string());
    println!("Finished patching.");
}

/// Entrypoint.
fn main() {
    // Grab all proxies
    let proxies = get_proxies();

    // Grab vars, checking if using automated process
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        // Figure out which proxy we are using
        let arg = args.get(1).unwrap();
        let selected_proxy = if arg.chars().next().unwrap().is_numeric() {
            proxies
                .get(arg.parse::<usize>().unwrap())
                .expect("unable to get proxy index")
        } else {
            arg
        };

        // Download and patch
        download_patch(selected_proxy.to_string());

        // Done
        return
    }

    // Construct the menu and run it
    let menu = menu(vec![
        label("-------------------------"),
        label("-     RoPro Patcher     -"),
        label("- Created by Stefanuk12 -"),
        label("-------------------------"),
        submenu("Custom Patch", vec![
            label      ("-----------------------------------"),
            label      ("-     RoPro Patcher - Patcher     -"),
            label      ("-      Created by Stefanuk12      -"),
            label      ("-----------------------------------"),
            scroll     ("Select a proxy", proxies.clone()),
            string     ("Custom proxy (overwrites)", "", true),
            label      ("--------------"),
            string     ("RoPro Path", "./", false),
            list       ("Use Opera GX Path", vec!["No", "Yes"]),
            label      ("--------------"),
            button     ("Start"),
            back_button("Back")
        ]),
        button("Download RoPro source as .zip"),
        button("Download and Patch (uses default proxy)"),
        back_button("Exit")
    ]);
    run(&menu);

    // User has exited, process their action
    let mut mm = mut_menu(&menu);
    let selected_item = mm.selected_item_name();
    match selected_item {
        "Exit" => return println!("Goodbye!"),
        "Download RoPro source as .zip" => download_extract(),
        "Download and Patch (uses default proxy)" => download_patch(proxies.get(1).unwrap().to_string()),
        "Patch" => {
            // Grab their selected proxy
            let patch_menu = mm.get_submenu("Patch");
            let custom_proxy = patch_menu.selection_value("Custom proxy (overwrites)");
            let selected_proxy = if custom_proxy.is_empty() { patch_menu.selection_value("Select a proxy") } else { custom_proxy }; 
            
            // Grab their selected path
            let selected_path = if patch_menu.selection_value("Use Opera GX Path") == "Yes" {
                AppDirs::new(Some(r"Opera Software\Opera GX Stable\Extensions\adbacgifemdbhdkfppmeilbgppmhaobf"), false).unwrap().config_dir
            } else {
                PathBuf::from(patch_menu.selection_value("RoPro Path"))
            };

            // Patch
            patch(selected_path, selected_proxy.to_string());
            println!("Finished patching.");
        }
        _ => return println!("You should not be seeing this...")
    };
}