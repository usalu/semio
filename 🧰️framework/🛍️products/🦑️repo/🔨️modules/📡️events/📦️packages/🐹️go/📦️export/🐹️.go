package events

import (
	context "context"
	sha256 "crypto/sha256"
	hex "encoding/hex"
	json "encoding/json"
	fmt "fmt"
)

// #region 📦️Export

// 🧭️ExportSource is the port an export reads its entities through.
type ExportSource interface {
	ExportEntities() []ExportEntity
}

// 🏷️ExportEntity is one entity of a repository snapshot.
type ExportEntity struct {
	Kind  string
	ID    string
	Value interface{}
}

// 🧾️ExportSnapshot is the deterministic identity and per-kind counts of one export batch.
type ExportSnapshot struct {
	Snapshot string         `json:"snapshot"`
	Counts   map[string]int `json:"counts"`
	Inputs   []Input        `json:"-"`
}

// 🧮️BuildExportSnapshot sorts entities by id, hashes them and namespaces every input id.
func BuildExportSnapshot(ctx context.Context, entities []ExportEntity) (ExportSnapshot, error) {
	snapshot := ExportSnapshot{Counts: map[string]int{}}
	inputs := make([]Input, 0, len(entities))
	for _, entity := range entities {
		if err := ctx.Err(); err != nil {
			return ExportSnapshot{}, err
		}
		snapshot.Counts[entity.Kind]++
		inputs = append(inputs, Input{ID: entity.Kind + ":" + entity.ID, Kind: entity.Kind + ".recorded", Data: entity.Value})
	}
	sortInputs(inputs)
	hash := sha256.New()
	for _, input := range inputs {
		if err := ctx.Err(); err != nil {
			return ExportSnapshot{}, err
		}
		data, err := json.Marshal(input.Data)
		if err != nil {
			return ExportSnapshot{}, err
		}
		fmt.Fprintf(hash, "%s\x00%s\x00", input.ID, input.Kind)
		hash.Write(data)
	}
	snapshot.Snapshot = hex.EncodeToString(hash.Sum(nil))
	for index := range inputs {
		inputs[index].ID = "snapshot:" + snapshot.Snapshot + ":" + inputs[index].ID
	}
	snapshot.Inputs = inputs
	return snapshot, nil
}

func sortInputs(inputs []Input) {
	for outer := 1; outer < len(inputs); outer++ {
		current := inputs[outer]
		inner := outer - 1
		for inner >= 0 && inputs[inner].ID > current.ID {
			inputs[inner+1] = inputs[inner]
			inner--
		}
		inputs[inner+1] = current
	}
}

// #endregion 📦️Export
