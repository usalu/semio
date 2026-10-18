package goals

import (
	bytes "bytes"
	json "encoding/json"

	model "github.com/usalu/semio/repo/model"
)

// #region 📄️Document Codec

// 📄️goalRequiredMembers are the members a stored goal document must carry.
var goalRequiredMembers = []string{"title", "description", "prompt", "status", "client", "llm"}

// 📥️DecodeGoal decodes a goal document, deriving the identifier and parent from the path the
// document was found at, exactly as the reader does.
func DecodeGoal(id string, document string) (*model.Goal, error) {
	var members map[string]json.RawMessage
	if err := json.Unmarshal([]byte(document), &members); err != nil {
		return nil, errDocument(err.Error())
	}
	for _, required := range goalRequiredMembers {
		if _, found := members[required]; !found {
			return nil, errDocument("missing field `" + required + "`")
		}
	}
	var status string
	if err := json.Unmarshal(members["status"], &status); err != nil {
		return nil, errDocument(err.Error())
	}
	if _, err := ParseGoalStatus(status); err != nil {
		return nil, err
	}
	var goal model.Goal
	if err := json.Unmarshal([]byte(document), &goal); err != nil {
		return nil, errDocument(err.Error())
	}
	goal.ID = id
	goal.Parent = ParentGoalID(id)
	return &goal, nil
}

// 📤️EncodeGoal encodes a goal back into its document: two-space indentation, members in
// declaration order, a trailing newline, and the parent member in its compose form.
func EncodeGoal(goal *model.Goal) (string, error) {
	var buffer bytes.Buffer
	encoder := json.NewEncoder(&buffer)
	encoder.SetEscapeHTML(false)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(goal); err != nil {
		return "", errDocument(err.Error())
	}
	return buffer.String(), nil
}

// #endregion 📄️Document Codec
