// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧩️parser carries the GraphQL document parser and its validation.

// #endregion 🧲️Header

package graphql

import (
	fmt "fmt"
	strconv "strconv"
	strings "strings"
)

// #region 🧩️Parser

/** 🧩️ A single-token-lookahead recursive descent parser over the lexer. */
type parser struct {
	lexer   lexer
	current token
	err     error
}

/** 🧩️ Starts a parser primed with the first token. */
func newParser(source string) *parser {
	parser := &parser{lexer: lexer{source: source}}
	parser.advance()
	return parser
}

/** 🧩️ Reads the next token unless the parser already failed. */
func (parser *parser) advance() {
	if parser.err != nil {
		return
	}
	parser.current, parser.err = parser.lexer.next()
}

/** 🧩️ Consumes an exact punctuation or keyword, reporting the offset when it is missing. */
func (parser *parser) take(text string) error {
	if parser.err != nil {
		return parser.err
	}
	if parser.current.text != text {
		return fmt.Errorf("expected %q at %d, got %q", text, parser.current.offset, parser.current.text)
	}
	parser.advance()
	return nil
}

/** 🧩️ Skips a balanced parenthesised group — variable definitions and directive arguments. */
func (parser *parser) skipParenthesized() {
	depth := 0
	for parser.current.kind != tokenEOF {
		if parser.current.text == "(" {
			depth++
		}
		if parser.current.text == ")" {
			depth--
			parser.advance()
			if depth == 0 {
				break
			}
			continue
		}
		parser.advance()
	}
}

/** 🧩️ Parses one document: an optional operation header followed by the root selection set. */
func Parse(source string) (Document, error) {
	parser := newParser(source)
	doc := Document{Operation: "query"}
	if parser.current.kind == tokenName && (parser.current.text == "query" || parser.current.text == "mutation") {
		doc.Operation = parser.current.text
		parser.advance()
		if parser.current.kind == tokenName {
			parser.advance()
		}
		if parser.current.text == "(" {
			parser.skipParenthesized()
		}
	}
	selections, err := parser.selectionSet()
	if err != nil {
		return Document{}, err
	}
	doc.Selections = selections
	if parser.err != nil {
		return Document{}, parser.err
	}
	if parser.current.kind != tokenEOF {
		return Document{}, fmt.Errorf("unexpected token %q at %d", parser.current.text, parser.current.offset)
	}
	return doc, nil
}

/** 🧩️ Parses a brace-delimited selection set, including aliases, arguments and dropped directives. */
func (parser *parser) selectionSet() ([]Selection, error) {
	if err := parser.take("{"); err != nil {
		return nil, err
	}
	var selections []Selection
	for parser.current.text != "}" {
		if parser.current.kind == tokenEOF {
			return nil, fmt.Errorf("unterminated selection set")
		}
		if parser.current.kind != tokenName {
			return nil, fmt.Errorf("expected field name at %d", parser.current.offset)
		}
		item := Selection{Name: parser.current.text, Arguments: map[string]Value{}}
		parser.advance()
		if parser.current.text == ":" {
			parser.advance()
			item.Alias = item.Name
			if parser.current.kind != tokenName {
				return nil, fmt.Errorf("expected aliased field name")
			}
			item.Name = parser.current.text
			parser.advance()
		}
		if parser.current.text == "(" {
			parser.advance()
			for parser.current.text != ")" {
				if parser.current.kind != tokenName {
					return nil, fmt.Errorf("expected argument name at %d", parser.current.offset)
				}
				name := parser.current.text
				parser.advance()
				if err := parser.take(":"); err != nil {
					return nil, err
				}
				argument, err := parser.value()
				if err != nil {
					return nil, err
				}
				item.Arguments[name] = argument
			}
			parser.advance()
		}
		for parser.current.text == "@" {
			parser.advance()
			if parser.current.kind == tokenName {
				parser.advance()
			}
			if parser.current.text == "(" {
				parser.skipParenthesized()
			}
		}
		if parser.current.text == "{" {
			fields, err := parser.selectionSet()
			if err != nil {
				return nil, err
			}
			item.Fields = fields
		}
		selections = append(selections, item)
	}
	parser.advance()
	return selections, nil
}

/** 🧩️ Parses one argument value: variable, list, input object, string, number, boolean, null or enum. */
func (parser *parser) value() (Value, error) {
	current := parser.current
	switch {
	case current.text == "$":
		parser.advance()
		if parser.current.kind != tokenName {
			return Value{}, fmt.Errorf("expected variable name")
		}
		result := Value{Variable: parser.current.text}
		parser.advance()
		return result, nil
	case current.text == "[":
		parser.advance()
		result := Value{}
		for parser.current.text != "]" {
			item, err := parser.value()
			if err != nil {
				return Value{}, err
			}
			result.List = append(result.List, item)
		}
		parser.advance()
		return result, nil
	case current.text == "{":
		parser.advance()
		result := Value{Object: map[string]Value{}}
		for parser.current.text != "}" {
			if parser.current.kind != tokenName {
				return Value{}, fmt.Errorf("expected object field")
			}
			name := parser.current.text
			parser.advance()
			if err := parser.take(":"); err != nil {
				return Value{}, err
			}
			item, err := parser.value()
			if err != nil {
				return Value{}, err
			}
			result.Object[name] = item
		}
		parser.advance()
		return result, nil
	case current.kind == tokenString:
		parser.advance()
		return Value{Literal: current.text}, nil
	case current.kind == tokenNumber:
		parser.advance()
		if strings.ContainsAny(current.text, ".eE") {
			parsed, err := strconv.ParseFloat(current.text, 64)
			return Value{Literal: parsed}, err
		}
		parsed, err := strconv.Atoi(current.text)
		return Value{Literal: parsed}, err
	case current.kind == tokenName:
		parser.advance()
		switch current.text {
		case "true":
			return Value{Literal: true}, nil
		case "false":
			return Value{Literal: false}, nil
		case "null":
			return Value{Literal: nil}, nil
		}
		return Value{Literal: current.text}, nil
	default:
		return Value{}, fmt.Errorf("invalid value at %d", current.offset)
	}
}

// #endregion 🧩️Parser

// #region ✅️Validation

/** ✅️ Reports whether a request string is a document this grammar accepts. */
func Validate(source string) error { _, err := Parse(source); return err }

/** ✅️ The operation kind of a request string, without executing it. */
func OperationType(source string) (string, error) {
	doc, err := Parse(source)
	return doc.Operation, err
}

// #endregion ✅️Validation
