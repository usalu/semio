// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Unit coverage of the owned GraphQL grammar. The language-neutral contract lives in
// `🔗️graphql/🧪️tests/**`; these cases pin the exact diagnostics the projection profile drops.

// #endregion 🧲️Header

package graphql

import (
	"encoding/json"
	"testing"
)

// #region 🧩️Parser

func projectionJSON(t *testing.T, source string) string {
	t.Helper()
	document, err := Parse(source)
	if err != nil {
		t.Fatalf("Parse(%q) failed: %v", source, err)
	}
	encoded, err := json.Marshal(document.Projection())
	if err != nil {
		t.Fatalf("Marshal failed: %v", err)
	}
	return string(encoded)
}

func TestParseProjectsCanonicalShape(t *testing.T) {
	cases := []struct{ source, want string }{
		{"{ repo }", `{"operation":"query","selections":[{"alias":null,"arguments":[],"fields":[],"name":"repo"}]}`},
		{"query { a b }", `{"operation":"query","selections":[{"alias":null,"arguments":[],"fields":[],"name":"a"},{"alias":null,"arguments":[],"fields":[],"name":"b"}]}`},
		{"mutation Open { ticketOpen }", `{"operation":"mutation","selections":[{"alias":null,"arguments":[],"fields":[],"name":"ticketOpen"}]}`},
		{"{ first: repo }", `{"operation":"query","selections":[{"alias":"first","arguments":[],"fields":[],"name":"repo"}]}`},
		{"{ t(id: \"x\") }", `{"operation":"query","selections":[{"alias":null,"arguments":[{"name":"id","value":{"kind":"literal","value":"x"}}],"fields":[],"name":"t"}]}`},
		{"{ t(n: 3, f: 1.5, b: true, e: OPEN, z: null) }", `{"operation":"query","selections":[{"alias":null,"arguments":[{"name":"b","value":{"kind":"literal","value":true}},{"name":"e","value":{"kind":"literal","value":"OPEN"}},{"name":"f","value":{"kind":"literal","value":1.5}},{"name":"n","value":{"kind":"literal","value":3}},{"name":"z","value":{"kind":"null"}}],"fields":[],"name":"t"}]}`},
		{"{ t(v: $id) }", `{"operation":"query","selections":[{"alias":null,"arguments":[{"name":"v","value":{"kind":"variable","name":"id"}}],"fields":[],"name":"t"}]}`},
		{"{ t(l: [1, 2]) }", `{"operation":"query","selections":[{"alias":null,"arguments":[{"name":"l","value":{"items":[{"kind":"literal","value":1},{"kind":"literal","value":2}],"kind":"list"}}],"fields":[],"name":"t"}]}`},
		{"{ t(o: {a: 1}) }", `{"operation":"query","selections":[{"alias":null,"arguments":[{"name":"o","value":{"fields":[{"name":"a","value":{"kind":"literal","value":1}}],"kind":"object"}}],"fields":[],"name":"t"}]}`},
		{"query Q($id: ID!) { t(v: $id) }", `{"operation":"query","selections":[{"alias":null,"arguments":[{"name":"v","value":{"kind":"variable","name":"id"}}],"fields":[],"name":"t"}]}`},
		{"{ t @include(if: true) }", `{"operation":"query","selections":[{"alias":null,"arguments":[],"fields":[],"name":"t"}]}`},
		{"# lead\n{ t # trail\n}", `{"operation":"query","selections":[{"alias":null,"arguments":[],"fields":[],"name":"t"}]}`},
		{"{ a { b { c } } }", `{"operation":"query","selections":[{"alias":null,"arguments":[],"fields":[{"alias":null,"arguments":[],"fields":[{"alias":null,"arguments":[],"fields":[],"name":"c"}],"name":"b"}],"name":"a"}]}`},
	}
	for _, item := range cases {
		if got := projectionJSON(t, item.source); got != item.want {
			t.Errorf("Parse(%q)\n got %s\nwant %s", item.source, got, item.want)
		}
	}
}

func TestParseRejectsWithExactDiagnostics(t *testing.T) {
	cases := []struct{ source, want string }{
		{"", `expected "{" at 0, got ""`},
		{"{", "unterminated selection set"},
		{"{ 1 }", "expected field name at 2"},
		{"{ a(1: 2) }", "expected argument name at 4"},
		{"{ a(b) }", `expected ":" at 5, got ")"`},
		{"{ a: }", "expected aliased field name"},
		{"{ a(b: $) }", "expected variable name"},
		{"{ a } extra", `unexpected token "extra" at 6`},
		{"{ a(b: \"unterminated) }", "invalid value at 0"},
		{"{ a(b: {1: 2}) }", "expected object field"},
		{"{ a. }", "unterminated selection set"},
		{"fragment F on T { a }", `expected "{" at 0, got "fragment"`},
		{"{ ... on T { a } }", "unterminated selection set"},
	}
	for _, item := range cases {
		_, err := Parse(item.source)
		if err == nil {
			t.Errorf("Parse(%q) unexpectedly succeeded", item.source)
			continue
		}
		if err.Error() != item.want {
			t.Errorf("Parse(%q)\n got %q\nwant %q", item.source, err.Error(), item.want)
		}
	}
}

// #endregion 🧩️Parser

// #region 🔢️Coercion

func TestCoerceArgumentsAppliesDefaultsOnlyWhenAbsent(t *testing.T) {
	document, err := Parse("{ tickets(state: $state, limit: $limit) }")
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}
	args := CoerceArguments(document.Selections[0].Arguments, map[string]interface{}{"state": "open", "limit": 10, "cursor": "start"}, map[string]interface{}{"limit": 5})
	if args["limit"] != 5 {
		t.Errorf("explicit variable must win over the default, got %v", args["limit"])
	}
	if args["state"] != nil {
		t.Errorf("a present argument bound to a missing variable stays null, got %v", args["state"])
	}
	if args["cursor"] != "start" {
		t.Errorf("an absent argument takes its default, got %v", args["cursor"])
	}
}

func TestOperationType(t *testing.T) {
	for source, want := range map[string]string{"{ a }": "query", "query { a }": "query", "mutation { a }": "mutation"} {
		got, err := OperationType(source)
		if err != nil || got != want {
			t.Errorf("OperationType(%q) = %q, %v; want %q", source, got, err, want)
		}
	}
	if err := Validate("{"); err == nil {
		t.Error("Validate must reject an unterminated selection set")
	}
}

// #endregion 🔢️Coercion
