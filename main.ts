// Dependencies
import { serve } from "https://deno.land/std@0.170.0/http/mod.ts";
import { sleep } from "https://deno.land/x/sleep@v1.2.1/mod.ts";

// Vars
const DataFetchURL = "https://raw.githubusercontent.com/Stefanuk12/RoProPatcher/proxy/data.json"
let Data = {
    "PHPSESSID": "",
    "ropro-id": "",
    "ropro-verification": "",
    "tier": "pro_tier"
}

// See whenever we get an inbound request
async function reqHandler(req: Request) {
    // Replace host, checking the subdomain
    const RoProURL = new URL(req.url)
    const FoundAPI = RoProURL.pathname.indexOf("///api")
    if (FoundAPI != -1) {
        RoProURL.host = "api.ropro.io"
        RoProURL.pathname = RoProURL.pathname.substring(0, FoundAPI)
    } else
        RoProURL.host = "ropro.io"

    // CORS
    console.debug(`Incoming (${req.method.toUpperCase()}): ${RoProURL}`)
    const CORSheaders = new Headers()
    const origin = req.headers.get("origin") || "chrome-extension://adbacgifemdbhdkfppmeilbgppmhaobf"
    CORSheaders.set("access-control-allow-origin", origin)
    CORSheaders.set("access-control-allow-headers", "*")
    CORSheaders.set("access-control-allow-credentials", "true")
    if (req.method.toUpperCase() == "OPTIONS") {
        console.debug(`Sent OPTIONS: ${RoProURL}`)
        return new Response(null, {
            headers: CORSheaders
        })
    }

    // Check if the is the getSubscription one
    if (RoProURL.pathname == "/getSubscription.php") {
        return new Response(Data.tier, {
            status: 200,
            headers: CORSheaders
        })
    }

    // Set the headers, only if they are not "blank". Assume if one is blank, the rest are.
    const headers = new Headers(req.headers)
    if (Data.PHPSESSID != "") {
        headers.set("Cookie", `PHPSESSID=${Data.PHPSESSID}`)
        headers.set("ropro-id", Data["ropro-id"])
        headers.set("ropro-verification", Data["ropro-verification"])
    }

    // Perform the request
    const response = await fetch(RoProURL, {
        method: req.method,
        headers: headers,
        body: req.body
    })
    console.debug(`Performed request: ${RoProURL}`)

    // Add CORS to headers
    const responseHeaders = new Headers(response.headers)
    responseHeaders.set("access-control-allow-origin", origin)
    responseHeaders.set("access-control-allow-headers", "*")
    responseHeaders.set("access-control-allow-credentials", "true")

    // Return
    return new Response(response.body, {
        headers: responseHeaders,
        status: response.status,
        statusText: response.statusText
    })
}

// Serve
serve(reqHandler, {port: 443});

// Refresh the data every 5 minutes
(async () => {
    while (true) {
        // Grab the data, and set.
        Data = await (await fetch(DataFetchURL)).json()
        console.debug("Refreshed data", Data)

        // Wait some time
        await sleep(300)
    }
})()