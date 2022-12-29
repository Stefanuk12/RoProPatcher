# RoPro Patcher - Proxy
This is the source for the proxy linked to the patcher. This proxy adds the correct cookie and "ropro" headers to your request and then directly sends back the response. In turn, it tricks the extension into thinking you have the highest tier and also makes the server filters work.

## How to deploy
If you don't trust my own proxy, you can follow these steps:
- Fork this repo
- Create an account at https://deno.dev
- Deploy with that
- Set the proxy domain to your deployed domain