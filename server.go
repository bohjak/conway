package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"sync"
)

type connection struct {
	writer     http.ResponseWriter
	flusher    http.Flusher
	requestCtx context.Context
}

type broker struct {
	connections      map[string]*connection
	connectionsMutex sync.RWMutex
}

func (b *broker) handleConnection(rw http.ResponseWriter, req *http.Request) {
	flusher, ok := rw.(http.Flusher)
	if !ok {
		http.Error(rw, "streaming unsupported", http.StatusInternalServerError)
		return
	}

	requestCtx := req.Context()
	b.connectionsMutex.Lock()
	b.connections[req.RemoteAddr] = &connection{
		writer:     rw,
		flusher:    flusher,
		requestCtx: requestCtx,
	}
	b.connectionsMutex.Unlock()
	defer b.removeConnection(req.RemoteAddr)

	rw.Header().Set("Content-Type", "text/event-stream")
	rw.Header().Set("Cache-Control", "no-cache")
	rw.Header().Set("Connection", "keep-alive")
	rw.Header().Set("Access-Control-Allow-Origin", "*")

	<-requestCtx.Done()
}

func (b *broker) removeConnection(key string) {
	b.connectionsMutex.Lock()
	delete(b.connections, key)
	b.connectionsMutex.Unlock()
}

func (b *broker) sendReload() {
	b.connectionsMutex.RLock()
	defer b.connectionsMutex.RUnlock()

	msgBytes := []byte("data: reload\n\n")
	for client, connection := range b.connections {
		_, err := connection.writer.Write(msgBytes)
		if err != nil {
			b.removeConnection(client)
			continue
		}

		connection.flusher.Flush()
	}
}

func (b *broker) handleReloadReq(res http.ResponseWriter, req *http.Request) {
	b.sendReload()
}

func main() {
	fs := http.FileServer(http.Dir("."))
	http.Handle("/", fs)

	broker := broker{connections: map[string]*connection{}}
	http.HandleFunc("/sse", broker.handleConnection)
	http.HandleFunc("/sse/reload", broker.handleReloadReq)

	fmt.Println("starting on http://localhost:3000")
	fmt.Println("send requests to http://localhost:3000/sse/reload to trigger reloads")
	log.Fatal(http.ListenAndServe(":3000", nil))
}
