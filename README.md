# RoPro Patcher
This automatically patches the [RoPro](https://chrome.google.com/webstore/detail/ropro-enhance-your-roblox/adbacgifemdbhdkfppmeilbgppmhaobf?hl=en-GB) extension for you, allowing you to have `pro_tier` for free.

## Firefox
This currently does not support Firefox but it easily can be done manually. 

- Go to where your extension is installed, usually at `%appdata%\Mozilla\Firefox\Profiles` followed by your profile name (the most recently modified folder) and finally into `extensions`. 
- From there, find the RoPro file - for me it is named `{fbfda72b-073a-4a24-9e87-6d472b69b66f}.xpi`. 
- Now, change the `.xpi` extension to `.zip` and extract. 
- Next, apply the custom path patch to it. 
- Rezip the file and change the extension back to `.xpi`. 
- Make sure to override the old extension.

## NOTE
Chrome (and possibly other browsers) have a feature that checks the hash of the extension. This means that it will flag as corrupted. Therefore, you will have to download the extension, patch it with a custom path and use developer mode to load an unpacked extension.

- an exception to this rule is Opera GX