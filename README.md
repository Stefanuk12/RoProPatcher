# RoPro Patcher - Proxy
This is the source for the proxy linked to the patcher. This proxy adds the correct cookie and "ropro" headers to your request and then directly sends back the response. In turn, it tricks the extension into thinking you have the highest tier and also makes the server filters work.

There is another proxy hosted by darkhub: `ropro.darkhub.cloud`. You may opt for theirs. The code for their proxy can be seen in the go-lang branch.

## Grabbing your own PHPSESSID
- Open the extension's developer console by heading to the extension page for your browser and pressing on the underlined link beside "Inspect views"
- Open the Network tab
- View the servers of a Roblox game
- Press on any request to the RoPro servers (in the Network tab of the developer console)
- Under Headers > Request Headers > cookie, you should find `PHPSESSID=...`. The `...` is your PHPSESSID

## How to deploy
If you don't trust my own proxy, you can follow these steps:
- Fork this repo
- Create an account at https://deno.dev
- Deploy with that
- Set the proxy domain to your deployed domain
