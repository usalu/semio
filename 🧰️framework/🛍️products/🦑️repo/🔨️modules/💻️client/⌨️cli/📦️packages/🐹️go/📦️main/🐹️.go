package main

import (
 "fmt"
 "os"
 client "github.com/usalu/semio/repo/client"
)

// ▶️ main executes the owned repository client command entry point.
func main() {
 if err := client.RunCLI(); err != nil {
  fmt.Fprintln(os.Stderr, err)
  os.Exit(1)
 }
}
