module github.com/usalu/semio/repo/client

go 1.25

require (
	github.com/usalu/semio/repo/yaml v0.0.0
	github.com/usalu/semio/repo/go v0.0.0
	github.com/usalu/semio/repo/search v0.0.0
)

replace github.com/usalu/semio/repo/go => ../../📚️library

replace github.com/usalu/semio/repo/search => ../../🔎️search/📦️packages/🐹️go

replace github.com/usalu/semio/repo/yaml => ../../🧾️yaml/📦️packages/🐹️go
