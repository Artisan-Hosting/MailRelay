package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

type Data struct {
	Name    string `json:"name"`
	Email   string `json:"email"`
	Message string `json:"message"`
}

func sendMail(data Data) {
	url := "https://relay.artisanhosting.net:8000/api/sendmail"
	jsonData, err := json.Marshal(data)
	if err != nil {
		fmt.Println("Error encoding JSON:", err)
		return
	}

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return
	}
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{Timeout: time.Second * 10}
	resp, err := client.Do(req)
	if err != nil {
		fmt.Println("Error sending request:", err)
		return
	}
	defer resp.Body.Close()

	var responseData map[string]interface{}
	err = json.NewDecoder(resp.Body).Decode(&responseData)
	if err != nil {
		fmt.Println("Error decoding response:", err)
		return
	}

	if responseData["status"] == "success" {
		fmt.Println("Success:", responseData["message"])
	} else {
		fmt.Println("Failed:", responseData["message"])
	}
}

func main() {
	data := Data{
		Name:    "John Doe",
		Email:   "john@example.com",
		Message: "Hello, this is a test message!",
	}
	sendMail(data)
}
