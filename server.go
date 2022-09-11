package main

import (
	"log"
	"net/http"
)

func main() {
	fs := http.FileServer(http.Dir("."))
	log.Fatal(http.ListenAndServe(":3000", fs))
}
