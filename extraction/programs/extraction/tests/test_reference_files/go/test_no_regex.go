package main

import "log"

func main() {
	log.Println("Starting")
	go serve()
	connectToPolygon()
}
