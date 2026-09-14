module github.com/usalu/semio/repo/coordinator/bin

go 1.25

require github.com/usalu/semio/repo/coordinator v0.0.0

require (
	github.com/usalu/semio/repo/events v0.0.0 // indirect
	github.com/usalu/semio/repo/languages v0.0.0 // indirect
)

replace github.com/usalu/semio/repo/coordinator => ..

replace github.com/usalu/semio/repo/events => ../../../../../📡️events/📦️packages/🐹️go

replace github.com/usalu/semio/repo/languages => ../../../../../🗣️languages/📦️packages/🐹️go
