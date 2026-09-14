module github.com/usalu/semio/repo/testrunner

go 1.25

require (
	github.com/usalu/semio/repo/codebase v0.0.0
	github.com/usalu/semio/repo/languages v0.0.0
	github.com/usalu/semio/repo/model v0.0.0
	github.com/usalu/semio/repo/statutes v0.0.0
	github.com/usalu/semio/repo/tickets v0.0.0
	github.com/usalu/semio/repo/todos v0.0.0
	github.com/usalu/semio/repo/workspace v0.0.0
)

replace github.com/usalu/semio/repo/codebase => ../../../🗂️codebase/📦️packages/🐹️go

replace github.com/usalu/semio/repo/languages => ../../../🗣️languages/📦️packages/🐹️go

replace github.com/usalu/semio/repo/model => ../../../📐️model/📦️packages/🐹️go

replace github.com/usalu/semio/repo/statutes => ../../../📜️statutes/📦️packages/🐹️go

replace github.com/usalu/semio/repo/tickets => ../../../🎫️tickets/📦️packages/🐹️go

replace github.com/usalu/semio/repo/todos => ../../../📝️todos/📦️packages/🐹️go

replace github.com/usalu/semio/repo/workspace => ../../../🏠️workspace/📦️packages/🐹️go
