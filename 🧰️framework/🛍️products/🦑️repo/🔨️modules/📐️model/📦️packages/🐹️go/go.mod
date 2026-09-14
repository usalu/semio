module github.com/usalu/semio/repo/model

go 1.25

require (
	github.com/usalu/semio/repo/identity v0.0.0
	github.com/usalu/semio/repo/search v0.0.0
	github.com/usalu/semio/repo/workspace v0.0.0
)

replace github.com/usalu/semio/repo/identity => ../../../🪪️identity/📦️packages/🐹️go

replace github.com/usalu/semio/repo/search => ../../../🔎️search/📦️packages/🐹️go

replace github.com/usalu/semio/repo/workspace => ../../../🏠️workspace/📦️packages/🐹️go
