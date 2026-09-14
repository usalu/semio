module github.com/usalu/semio/repo/mcp

go 1.25

require (
	github.com/usalu/semio/repo/cli v0.0.0
	github.com/usalu/semio/repo/move v0.0.0
	github.com/usalu/semio/repo/providers v0.0.0
	github.com/usalu/semio/repo/tickets v0.0.0
	github.com/usalu/semio/repo/workspace v0.0.0
)

replace github.com/usalu/semio/repo/cli => ../../../⌨️cli/📦️packages/🐹️go

replace github.com/usalu/semio/repo/move => ../../../🚚️move/📦️packages/🐹️go

replace github.com/usalu/semio/repo/providers => ../../../🧩️providers/📦️packages/🐹️go

replace github.com/usalu/semio/repo/tickets => ../../../🎫️tickets/📦️packages/🐹️go

replace github.com/usalu/semio/repo/workspace => ../../../🏠️workspace/📦️packages/🐹️go
