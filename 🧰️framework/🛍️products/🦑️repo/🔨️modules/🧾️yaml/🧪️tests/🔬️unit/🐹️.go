// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Unit tests for the bounded YAML decoder and the deterministic encoder.

// #endregion 🧲️Header

package yaml

import (
	"reflect"
	"testing"
)

// #region 📥️Decoding

func TestUnmarshalReadsBlockMappingsAndSequences(t *testing.T) {
	var node interface{}
	if err := Unmarshal([]byte("name: semio\npaths:\n  - a\n  - b\nenabled: true\n"), &node); err != nil {
		t.Fatal(err)
	}
	want := map[string]interface{}{
		"name":    "semio",
		"paths":   []interface{}{"a", "b"},
		"enabled": true,
	}
	if !reflect.DeepEqual(node, want) {
		t.Fatalf("node = %#v, want %#v", node, want)
	}
}

func TestUnmarshalAcceptsJsonAsASuperset(t *testing.T) {
	var node interface{}
	if err := Unmarshal([]byte(`{"a": 1}`), &node); err != nil {
		t.Fatal(err)
	}
	object, ok := node.(map[string]interface{})
	if !ok || object["a"] != float64(1) {
		t.Fatalf("node = %#v", node)
	}
}

func TestUnmarshalResolvesScalarKinds(t *testing.T) {
	var node interface{}
	source := "b: true\nf: false\nn: null\nt: ~\ni: 42\nd: 1.5\ns: plain\nq: \"a: b\"\nl: [x, y]\ne: \"\"\n"
	if err := Unmarshal([]byte(source), &node); err != nil {
		t.Fatal(err)
	}
	object := node.(map[string]interface{})
	cases := map[string]interface{}{
		"b": true,
		"f": false,
		"n": nil,
		"t": nil,
		"i": int64(42),
		"d": 1.5,
		"s": "plain",
		"q": "a: b",
		"e": "",
	}
	for key, want := range cases {
		if !reflect.DeepEqual(object[key], want) {
			t.Errorf("%s = %#v, want %#v", key, object[key], want)
		}
	}
	if !reflect.DeepEqual(object["l"], []interface{}{"x", "y"}) {
		t.Errorf("l = %#v", object["l"])
	}
}

func TestUnmarshalStripsCommentsAndDocumentMarkers(t *testing.T) {
	var node interface{}
	if err := Unmarshal([]byte("---\n# leading\nname: semio # trailing\n...\n"), &node); err != nil {
		t.Fatal(err)
	}
	if node.(map[string]interface{})["name"] != "semio" {
		t.Fatalf("node = %#v", node)
	}
}

// 📌️ Indentation is counted in spaces only, so a tab never becomes indentation: the line lands at
// depth zero and reads as a sibling. Both implementations must agree on this, including the fact
// that the tab guard the scanner carries can never fire.
func TestTabsAreNotIndentationAndProduceSiblings(t *testing.T) {
	var node interface{}
	if err := Unmarshal([]byte("a:\n\tb: 1\n"), &node); err != nil {
		t.Fatal(err)
	}
	object := node.(map[string]interface{})
	if _, found := object["b"]; !found {
		t.Fatalf("node = %#v, want b at the top level", node)
	}
}

func TestUnmarshalIntoAStructUsesTheJsonTags(t *testing.T) {
	var target struct {
		Name  string   `json:"name"`
		Paths []string `json:"paths"`
		On    bool     `json:"enabled"`
	}
	if err := Unmarshal([]byte("name: semio\npaths:\n  - a\n  - b\nenabled: true\n"), &target); err != nil {
		t.Fatal(err)
	}
	if target.Name != "semio" || len(target.Paths) != 2 || !target.On {
		t.Fatalf("target = %#v", target)
	}
}

// #endregion 📥️Decoding

// #region 📤️Encoding

func TestMarshalSortsKeysAndQuotesAmbiguousScalars(t *testing.T) {
	encoded, err := Marshal(map[string]interface{}{"b": 1, "a": "x: y"})
	if err != nil {
		t.Fatal(err)
	}
	if string(encoded) != "a: \"x: y\"\nb: 1\n" {
		t.Fatalf("encoded = %q", encoded)
	}
}

func TestMarshalIndentsNestedContainers(t *testing.T) {
	encoded, err := Marshal(map[string]interface{}{"paths": []interface{}{"a", "b"}})
	if err != nil {
		t.Fatal(err)
	}
	if string(encoded) != "paths:\n  - a\n  - b\n" {
		t.Fatalf("encoded = %q", encoded)
	}
}

func TestRoundTripIsStable(t *testing.T) {
	source := "enabled: true\nname: semio\npaths:\n  - a\n  - b\n"
	var node interface{}
	if err := Unmarshal([]byte(source), &node); err != nil {
		t.Fatal(err)
	}
	encoded, err := Marshal(node)
	if err != nil {
		t.Fatal(err)
	}
	if string(encoded) != source {
		t.Fatalf("round trip produced %q, want %q", encoded, source)
	}
}

// #endregion 📤️Encoding
