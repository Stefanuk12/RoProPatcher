# RoPro Patcher - Proxy (GO LANG)
This is the source for the proxy linked to the patcher. This proxy adds the correct cookie and "ropro" headers to your request and then directly sends back the response. In turn, it tricks the extension into thinking you have the highest tier and also makes the server filters work.

This version was created by @MaxTCodes and therefore maintained by them. **I will not be updating this myself if anything updates, except possibly the PHPSESSID**.

## This is being hosted for you!
Darkhub Developers have donated some of their server power, to host a version of this proxy for us! It is completly unlimited, and all the code is here! The url is [ropro.darkhub.cloud](//ropro.darkhub.cloud)! 

## Grabbing your own PHPSESSID
- Open the extension's developer console by heading to the extension page for your browser and pressing on the underlined link beside "Inspect views"
- Open the Network tab
- View the servers of a Roblox game
- Press on any request to the RoPro servers (in the Network tab of the developer console)
- Under Headers > Request Headers > cookie, you should find `PHPSESSID=...`. The `...` is your PHPSESSID