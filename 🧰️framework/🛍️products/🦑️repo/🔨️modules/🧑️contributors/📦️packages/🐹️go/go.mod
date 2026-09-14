module github.com/usalu/semio/repo/contributors

go 1.25

require (
	github.com/usalu/semio/repo/codebase v0.0.0
	github.com/usalu/semio/repo/events v0.0.0
	github.com/usalu/semio/repo/identity v0.0.0
	github.com/usalu/semio/repo/languages v0.0.0
	github.com/usalu/semio/repo/model v0.0.0
	github.com/usalu/semio/repo/workspace v0.0.0
)

replace github.com/usalu/semio/repo/codebase => ../../../🗂️codebase/📦️packages/🐹️go

replace github.com/usalu/semio/repo/events => ../../../📡️events/📦️packages/🐹️go

replace github.com/usalu/semio/repo/identity => ../../../🪪️identity/📦️packages/🐹️go

replace github.com/usalu/semio/repo/languages => ../../../🗣️languages/📦️packages/🐹️go

replace github.com/usalu/semio/repo/model => ../../../📐️model/📦️packages/🐹️go

replace github.com/usalu/semio/repo/workspace => ../../../🏠️workspace/📦️packages/🐹️go
