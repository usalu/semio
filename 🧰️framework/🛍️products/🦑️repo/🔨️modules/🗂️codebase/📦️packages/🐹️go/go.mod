module github.com/usalu/semio/repo/codebase

go 1.25

require (
	github.com/usalu/semio/repo/languages v0.0.0
	github.com/usalu/semio/repo/model v0.0.0
	github.com/usalu/semio/repo/statutes v0.0.0
	github.com/usalu/semio/repo/workspace v0.0.0
	github.com/usalu/semio/repo/yaml v0.0.0
)

replace github.com/usalu/semio/repo/languages => ../../../🗣️languages/📦️packages/🐹️go

replace github.com/usalu/semio/repo/model => ../../../📐️model/📦️packages/🐹️go

replace github.com/usalu/semio/repo/statutes => ../../../📜️statutes/📦️packages/🐹️go

replace github.com/usalu/semio/repo/workspace => ../../../🏠️workspace/📦️packages/🐹️go

replace github.com/usalu/semio/repo/yaml => ../../../🧾️yaml/📦️packages/🐹️go
