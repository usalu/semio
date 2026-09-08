// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Field-for-field conformance of the Go coordinator with the owner module
// 🧬️schema/🔣️.json (https://semio.tech/schema/repo/server/coordinator/schema.json).

// #endregion 🧲️Header

package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strings"
	"testing"

	repopkg "github.com/usalu/semio/repo/go"
)

// #region 🧰️Module

const coordinatorSchemaID = "https://semio.tech/schema/repo/server/coordinator/schema.json"

type schemaExport struct {
	Type                 string                     `json:"type"`
	AdditionalProperties *bool                      `json:"additionalProperties"`
	Required             []string                   `json:"required"`
	Properties           map[string]json.RawMessage `json:"properties"`
}

type schemaModule struct {
	Schema string                  `json:"$schema"`
	ID     string                  `json:"$id"`
	Defs   map[string]schemaExport `json:"$defs"`
}

type g3EventLogContract struct {
	Schema   string
	Encoding string
	Scalar   string
	Fields   []string
	Checksum string
}

func loadSchemaModule(t *testing.T) schemaModule {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join("🧬️schema", "🔣️.json"))
	if err != nil {
		t.Fatalf("read owner schema module: %v", err)
	}
	var module schemaModule
	if err := json.Unmarshal(raw, &module); err != nil {
		t.Fatalf("decode owner schema module: %v", err)
	}
	if module.Schema != "http://json-schema.org/draft-07/schema#" || module.ID != coordinatorSchemaID {
		t.Fatalf("unexpected schema identity %q %q", module.Schema, module.ID)
	}
	return module
}

// 📜️loadG3EventLogContract reads the language-neutral log contract from its owner module.
func loadG3EventLogContract(t *testing.T) g3EventLogContract {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join("🧬️schema", "🔣️.json"))
	if err != nil {
		t.Fatalf("read owner schema module: %v", err)
	}
	var module struct {
		Defs struct {
			Contract struct {
				Properties struct {
					Schema   struct{ Const string } `json:"schema"`
					Encoding struct{ Const string } `json:"encoding"`
					Scalar   struct{ Const string } `json:"scalar"`
					Fields   struct {
						Const []string `json:"const"`
					} `json:"fields"`
					Checksum struct{ Const string } `json:"checksum"`
				} `json:"properties"`
			} `json:"G3EventLogContract"`
		} `json:"$defs"`
	}
	if err := json.Unmarshal(raw, &module); err != nil {
		t.Fatalf("decode G3 event log contract: %v", err)
	}
	properties := module.Defs.Contract.Properties
	return g3EventLogContract{Schema: properties.Schema.Const, Encoding: properties.Encoding.Const, Scalar: properties.Scalar.Const, Fields: properties.Fields.Const, Checksum: properties.Checksum.Const}
}

func jsonFieldNames(value interface{}) []string {
	structType := reflect.TypeOf(value)
	names := []string{}
	for index := 0; index < structType.NumField(); index++ {
		tag := structType.Field(index).Tag.Get("json")
		if tag == "" || tag == "-" {
			continue
		}
		names = append(names, strings.Split(tag, ",")[0])
	}
	sort.Strings(names)
	return names
}

func exportPropertyNames(export schemaExport) []string {
	names := make([]string, 0, len(export.Properties))
	for name := range export.Properties {
		names = append(names, name)
	}
	sort.Strings(names)
	return names
}

// #endregion 🧰️Module

// #region 🧪️Parity

func TestGoTypesMirrorOwnerSchemaExports(t *testing.T) {
	module := loadSchemaModule(t)
	for exportID, value := range map[string]interface{}{
		"Ticket":              Ticket{},
		"TicketOpenRequest":   TicketOpenRequest{},
		"TicketCloseRequest":  TicketCloseRequest{},
		"TicketReopenRequest": TicketReopenRequest{},
		"Scope":               Scope{},
		"Warning":             Warning{},
		"Breach":              Breach{},
		"Event":               Event{},
		"FileSnapshot":        FileSnapshot{},
		"DiffIngestRequest":   DiffIngestRequest{},
		"DiffIngestResponse":  DiffIngestResponse{},
		"IndexFileRequest":    IndexFileRequest{},
		"EventPublishRequest": repopkg.Event{},
	} {
		export, ok := module.Defs[exportID]
		if !ok {
			t.Fatalf("missing schema export %q", exportID)
		}
		if export.AdditionalProperties == nil || *export.AdditionalProperties {
			t.Fatalf("export %q must be closed like the Go decoder", exportID)
		}
		actual, expected := jsonFieldNames(value), exportPropertyNames(export)
		if !reflect.DeepEqual(actual, expected) {
			t.Fatalf("export %q fields differ\ngo:     %v\nschema: %v", exportID, actual, expected)
		}
	}
}

func TestCoordinatorDecodesOwnerFixtures(t *testing.T) {
	raw, err := os.ReadFile(filepath.Join("🧫️fixtures", "📨️rest-cases.json"))
	if err != nil {
		t.Fatalf("read rest cases: %v", err)
	}
	var fixtures struct {
		Schema   string `json:"schema"`
		Accepted []struct {
			Export string          `json:"export"`
			Value  json.RawMessage `json:"value"`
		} `json:"accepted"`
	}
	if err := json.Unmarshal(raw, &fixtures); err != nil {
		t.Fatalf("decode rest cases: %v", err)
	}
	if fixtures.Schema != coordinatorSchemaID {
		t.Fatalf("fixtures bound to %q", fixtures.Schema)
	}
	decoders := map[string]func() interface{}{
		"TicketOpenRequest":   func() interface{} { return &TicketOpenRequest{} },
		"TicketCloseRequest":  func() interface{} { return &TicketCloseRequest{} },
		"TicketReopenRequest": func() interface{} { return &TicketReopenRequest{} },
		"DiffIngestRequest":   func() interface{} { return &DiffIngestRequest{} },
		"DiffIngestResponse":  func() interface{} { return &DiffIngestResponse{} },
		"IndexFileRequest":    func() interface{} { return &IndexFileRequest{} },
		"EventPublishRequest": func() interface{} { return &repopkg.Event{} },
		"Ticket":              func() interface{} { return &Ticket{} },
		"Scope":               func() interface{} { return &Scope{} },
		"Warning":             func() interface{} { return &Warning{} },
		"Breach":              func() interface{} { return &Breach{} },
	}
	decoded := 0
	for _, entry := range fixtures.Accepted {
		build, ok := decoders[entry.Export]
		if !ok {
			continue
		}
		target := build()
		decoder := json.NewDecoder(strings.NewReader(string(entry.Value)))
		decoder.DisallowUnknownFields()
		if err := decoder.Decode(target); err != nil {
			t.Fatalf("decode %s fixture: %v", entry.Export, err)
		}
		decoded++
	}
	if decoded == 0 {
		t.Fatal("no owner fixture reached a Go decoder")
	}
}

// #endregion 🧪️Parity
