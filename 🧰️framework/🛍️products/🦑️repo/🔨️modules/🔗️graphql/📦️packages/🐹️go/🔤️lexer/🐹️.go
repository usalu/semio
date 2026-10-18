// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🔤️lexer carries the GraphQL document lexer.

// #endregion 🧲️Header

package graphql

import (
	fmt "fmt"
	strconv "strconv"
	strings "strings"
	unicode "unicode"
)

// #region 🔤️Lexer

/** 🔤️ The five token classes the grammar distinguishes. */
type tokenKind int

const (
	tokenEOF tokenKind = iota
	tokenName
	tokenString
	tokenNumber
	tokenPunct
)

/** 🔤️ One lexed token and the byte offset it starts at. */
type token struct {
	kind   tokenKind
	text   string
	offset int
}

/** 🔤️ A byte-oriented scanner over one document source. */
type lexer struct {
	source string
	offset int
}

/** 🔤️ Reads the next token, skipping insignificant bytes, commas and comments. */
func (lexer *lexer) next() (token, error) {
	for lexer.offset < len(lexer.source) {
		r := rune(lexer.source[lexer.offset])
		if unicode.IsSpace(r) || r == ',' {
			lexer.offset++
			continue
		}
		if r == '#' {
			for lexer.offset < len(lexer.source) && lexer.source[lexer.offset] != '\n' {
				lexer.offset++
			}
			continue
		}
		break
	}
	if lexer.offset >= len(lexer.source) {
		return token{kind: tokenEOF, offset: lexer.offset}, nil
	}
	start := lexer.offset
	ch := lexer.source[lexer.offset]
	if strings.ContainsRune("{}()[]:$!@=", rune(ch)) {
		lexer.offset++
		return token{kind: tokenPunct, text: string(ch), offset: start}, nil
	}
	if ch == '"' {
		lexer.offset++
		for lexer.offset < len(lexer.source) {
			if lexer.source[lexer.offset] == '\\' {
				lexer.offset += 2
				continue
			}
			if lexer.source[lexer.offset] == '"' {
				lexer.offset++
				raw := lexer.source[start:lexer.offset]
				value, err := strconv.Unquote(raw)
				return token{kind: tokenString, text: value, offset: start}, err
			}
			lexer.offset++
		}
		return token{}, fmt.Errorf("unterminated string at %d", start)
	}
	if ch == '-' || ch >= '0' && ch <= '9' {
		lexer.offset++
		for lexer.offset < len(lexer.source) && strings.ContainsRune("0123456789.eE+-", rune(lexer.source[lexer.offset])) {
			lexer.offset++
		}
		return token{kind: tokenNumber, text: lexer.source[start:lexer.offset], offset: start}, nil
	}
	if ch == '_' || unicode.IsLetter(rune(ch)) {
		lexer.offset++
		for lexer.offset < len(lexer.source) {
			r := rune(lexer.source[lexer.offset])
			if r != '_' && !unicode.IsLetter(r) && !unicode.IsDigit(r) {
				break
			}
			lexer.offset++
		}
		return token{kind: tokenName, text: lexer.source[start:lexer.offset], offset: start}, nil
	}
	return token{}, fmt.Errorf("unexpected character %q at %d", ch, start)
}

// #endregion 🔤️Lexer
