# Getting RoPro Verification Token

This will detail how to get your verification token which 99% of you will find no use for. This token is used for tracking playtime for a specific user. By using this token, you are able to track playtime for your main account while on your alt account.

## Steps

- Go to your extensions in your browser, e.g. `chrome://extensions`
- You should see something like `Inspect views  background page` under RoPro
- Press on the underlined __background page__
- The developer console for the extensions should open, go to the console tab
- Execute this `await getStorage("userVerification")`
- The format will be `{USERID: 'TOKEN'}`