# Getting RoPro Verification Token

This will detail how to get your verification token, but before you read the steps, read [this](#please-read).

- This token is used for tracking playtime for a specific user
- By using this token, you are able to track playtime for your main account while on your alt account

## Steps

- Go to your extensions in your browser, e.g. `chrome://extensions`
- You should see something like `Inspect views  background page` under RoPro
- Press on the underlined __background page__
- The developer console for the extensions should open, go to the console tab
- Execute this `await getStorage("userVerification")`
- The format will be `{USERID: 'TOKEN'}`

## Please Read

This is not applicable to most of you. If you are having trouble with verifying your account via RoPro, this is an issue with RoPro, not the patcher. Keep trying, it will work eventually. This is only for developers.