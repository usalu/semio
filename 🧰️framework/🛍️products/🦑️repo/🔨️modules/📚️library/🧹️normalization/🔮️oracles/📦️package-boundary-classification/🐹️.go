package main

import (
	"fmt"
	"go/ast"
	"go/parser"
	"go/token"
	"os"
	"strconv"
)

func delegated(expression ast.Expr) bool {
	switch value := expression.(type) {
	case *ast.CallExpr:
		_, selected := value.Fun.(*ast.SelectorExpr)
		return selected
	case *ast.UnaryExpr:
		return delegated(value.X)
	default:
		return false
	}
}

func main() {
	offset := 1
	if os.Args[1] == "--" {
		offset = 2
	}
	maximum, error := strconv.Atoi(os.Args[offset+1])
	if error != nil {
		panic(error)
	}
	file, error := parser.ParseFile(token.NewFileSet(), os.Args[offset], nil, parser.AllErrors)
	if error != nil {
		fmt.Print("unresolved")
		return
	}
	role := "declaration"
	for _, declaration := range file.Decls {
		switch value := declaration.(type) {
		case *ast.GenDecl:
			if value.Tok != token.IMPORT {
				fmt.Print("implementation")
				return
			}
		case *ast.FuncDecl:
			if value.Name.Name != "main" && value.Name.Name != "init" || value.Body == nil || len(value.Body.List) == 0 || len(value.Body.List) > maximum {
				fmt.Print("implementation")
				return
			}
			for _, statement := range value.Body.List {
				expression, ok := statement.(*ast.ExprStmt)
				if !ok || !delegated(expression.X) {
					fmt.Print("implementation")
					return
				}
			}
			role = "bootstrap"
		default:
			fmt.Print("implementation")
			return
		}
	}
	fmt.Print(role)
}
