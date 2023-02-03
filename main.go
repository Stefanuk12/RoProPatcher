package main

import (
	"bytes"
	"encoding/base64"
	"github.com/goccy/go-json"
	"github.com/gofiber/fiber/v2"
	"io"
	"log"
	"math/rand"
	"net/http"
	"os"
	"strconv"
	"strings"
	"time"
)

type (
	DataResponse struct {
		SessID string `json:"PHPSESSID"`
		Tier   string `json:"tier"`
	}
	AlertResponse struct {
		Alert string `json:"alert"`
	}
)

const DataUrl = "https://raw.githubusercontent.com/Stefanuk12/RoProPatcher/proxy/data.json"

var (
	proxyData *DataResponse
)

func init() {
	// seed random :heart:
	rand.Seed(time.Now().UnixNano())
}

func updateData() {
	resp, err := http.Get(DataUrl)
	if err != nil {
		panic(err)
	}
	defer func() { _ = resp.Body.Close() }()
	bod, err := io.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}
	var proxData DataResponse
	err = json.Unmarshal(bod, &proxData)
	if err != nil {
		panic(err)
	}
	proxyData = &proxData
	log.Println("Successfully got data from source")
}

func stayAlive() {
	req, _ := http.NewRequest("GET", "https://api.ropro.io/handleRoProAlert.php?timestamp="+strconv.Itoa(rand.Intn(int(time.Now().UnixMicro()))), nil)

	// add cookie lazy way
	req.Header.Set("Cookie", "PHPSESSID="+(*proxyData).SessID)

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		log.Println("Error: " + err.Error())
		return
	}
	defer func() { _ = resp.Body.Close() }()
	bod, err := io.ReadAll(resp.Body)
	if err != nil {
		log.Println("Error: " + err.Error())
		return
	}
	alertDecoded := make([]byte, base64.URLEncoding.DecodedLen(len(bod)))
	_, err = base64.URLEncoding.Decode(alertDecoded, bod)
	if err != nil {
		log.Println("Error: " + err.Error())
		return
	}
	if string(alertDecoded) != "{\"alert\": null}" {
		var alert AlertResponse
		err := json.Unmarshal(alertDecoded, &alert)
		if err != nil {
			log.Println("Error: " + err.Error())
			return
		}
		log.Println("Received Alert: " + alert.Alert)
	}
}

func runBgTasks() {
	updateData()
	go stayAlive()
}

func runPollingThread() {
	for {
		go runBgTasks()
		<-time.After(300 * time.Second)
	}
}

func main() {
	log.SetOutput(os.Stdout)
	go runPollingThread()
	app := fiber.New(fiber.Config{
		AppName:     "RoPro Proxy",
		JSONEncoder: json.Marshal,
		JSONDecoder: json.Unmarshal,
	})

	// this is the equiv of "reqhandler"
	app.All("/*", func(c *fiber.Ctx) error {
		var newUrl = c.Protocol() + "://"
		originalPath := c.Path()
		// do some url parsing stuff
		if strings.Contains(originalPath, "///api") {
			newUrl += "api.ropro.io/" + strings.ReplaceAll(originalPath, "///api", "")
		} else {
			newUrl += "ropro.io" + originalPath
		}

		// get values for cors
		var origin = c.Get("origin", "")
		if len(origin) == 0 {
			origin = "chrome-extension://adbacgifemdbhdkfppmeilbgppmhaobf"
		}

		var allowedHeaders = ""
		for name, _ := range c.GetReqHeaders() {
			allowedHeaders += name + ","
		}
		allowedHeaders += "ropro-id, ropro-verification"

		c.Set("Access-Control-Allow-Origin", origin)
		c.Set("Access-Control-Allow-Headers", allowedHeaders)
		c.Set("Access-Control-Allow-Credentials", "true")
		if strings.ToLower(c.Method()) == "options" {
			return c.Send([]byte{})
		}

		// filter for subscription requests
		if strings.Contains(originalPath, "/getSubscription.php") {
			return c.SendString((*proxyData).Tier)
		}

		// build request
		req, err := http.NewRequest(c.Method(), newUrl, bytes.NewReader(c.Body()))
		if err != nil {
			log.Println("Error:" + err.Error())
			return c.SendStatus(fiber.StatusInternalServerError)
		}

		// add cookie lazy way
		req.Header.Set("Cookie", "PHPSESSID="+(*proxyData).SessID)

		// build list of allowed headers and add them to the req
		for name, value := range c.GetReqHeaders() {
			req.Header.Set(name, value)
		}

		// ig I need to add cors to it
		req.Header.Set("access-Control-Allow-Origin", origin)
		req.Header.Set("access-control-allow-headers", allowedHeaders)
		req.Header.Set("access-control-allow-credentials", "true")

		// make go do the request
		resp, err := http.DefaultClient.Do(req)
		if err != nil {
			log.Println("Error: " + err.Error())
			return c.SendStatus(fiber.StatusInternalServerError)
		}
		defer func() { _ = resp.Body.Close() }()

		// put all body in a byte array
		body, err := io.ReadAll(resp.Body)
		if err != nil {
			log.Println("Error: " + err.Error())
			return c.SendStatus(fiber.StatusInternalServerError)
		}

		// copy headers
		for k, vv := range resp.Header {
			for _, v := range vv {
				// remove cf headers
				c.Set(k, v)
			}
		}

		return c.Status(resp.StatusCode).Send(body)
	})

	if err := app.Listen(":5006"); err != nil {
		panic(err)
	}
}
