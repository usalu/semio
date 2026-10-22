package tickets

import (
	"encoding/json"
	"strings"
	"github.com/usalu/semio/repo/model"

)

// #region 📄️Codec

// 🧱️goMember is one member of a Go-shaped JSON object, in declaration order.
type goMember struct {
	name    string
	text    string
	nested  []goMember
	strings []string
	objects [][]goMember
	kind    int
}

// 🖨️renderGoMembers renders members the way MarshalIndent renders a struct.
func renderGoMembers(members []goMember, depth int) string {
	if len(members) == 0 {
		return "{}"
	}
	indent := strings.Repeat("  ", depth+1)
	closing := strings.Repeat("  ", depth)
	lines := make([]string, 0, len(members))
	for _, member := range members {
		switch member.kind {
		case 0:
			lines = append(lines, indent+GoJSONString(member.name)+": "+GoJSONString(member.text))
		case 1:
			lines = append(lines, indent+GoJSONString(member.name)+": "+renderGoMembers(member.nested, depth+1))
		case 3:
			if len(member.objects) == 0 { lines = append(lines, indent+GoJSONString(member.name)+": []"); continue }
			elementIndent := strings.Repeat("  ", depth+2)
			elements := make([]string, 0, len(member.objects))
			for _, object := range member.objects { elements = append(elements, elementIndent+renderGoMembers(object, depth+2)) }
			lines = append(lines, indent+GoJSONString(member.name)+": [\n"+strings.Join(elements, ",\n")+"\n"+indent+"]")
		default:
			if len(member.strings) == 0 {
				lines = append(lines, indent+GoJSONString(member.name)+": []")
				continue
			}
			elementIndent := strings.Repeat("  ", depth+2)
			elements := make([]string, 0, len(member.strings))
			for _, value := range member.strings {
				elements = append(elements, elementIndent+GoJSONString(value))
			}
			lines = append(lines, indent+GoJSONString(member.name)+": [\n"+strings.Join(elements, ",\n")+"\n"+indent+"]")
		}
	}
	return "{\n" + strings.Join(lines, ",\n") + "\n" + closing + "}"
}

// 🔤️textMember returns a text member, or the empty string when it is absent or not a string.
func textMember(members map[string]interface{}, name string) string {
	if value, ok := members[name].(string); ok {
		return value
	}
	return ""
}

// 🧺️objectsOf returns every object element of an array member.
func objectsOf(value interface{}) []map[string]interface{} {
	entries, ok := value.([]interface{})
	if !ok {
		return nil
	}
	found := make([]map[string]interface{}, 0, len(entries))
	for _, entry := range entries {
		if members, ok := entry.(map[string]interface{}); ok {
			found = append(found, members)
		}
	}
	return found
}

// 🟫️decodeInteraction decodes one interaction leniently, every absent member taking its zero value.
func decodeInteraction(members map[string]interface{}) model.Interaction {
	files := make([]model.InteractionFile, 0)
	for _, file := range objectsOf(members["files"]) {
		files = append(files, model.InteractionFile{Path: textMember(file, "path"), ID: textMember(file, "id"), URI: textMember(file, "uri")})
	}
	if len(files) == 0 {
		files = nil
	}
	return model.Interaction{
		Kind:       textMember(members, "kind"),
		Date:       textMember(members, "date"),
		Author:     textMember(members, "author"),
		System:     textMember(members, "system"),
		Client:     textMember(members, "client"),
		Checkpoint: textMember(members, "checkpoint"),
		Prompt:     textMember(members, "prompt"),
		Summary:    textMember(members, "summary"),
		LLM:        textMember(members, "llm"),
		Effort:     textMember(members, "effort"),
		Files:      files,
	}
}

// 🔳️decodeAgent decodes one agent record leniently, plan included only when it decodes cleanly.
func decodeAgent(members map[string]interface{}) model.TicketAgent {
	agent := model.TicketAgent{
		Session:     textMember(members, "session"),
		Contributor: textMember(members, "contributor"),
		System:      textMember(members, "system"),
		Client:      textMember(members, "client"),
		LLM:         textMember(members, "llm"),
		Effort:      textMember(members, "effort"),
		Transcript:  textMember(members, "transcript"),
	}
	if raw, found := members["plan"]; found {
		if data, err := json.Marshal(raw); err == nil {
			var plan model.TicketAgentPlan
			if err := json.Unmarshal(data, &plan); err == nil {
				agent.Plan = &plan
			}
		}
	}
	return agent
}

// 📖️DecodeTicketDocument decodes one ticket document. The status member is mandatory and closed.
func DecodeTicketDocument(text string) (*model.Ticket, error) {
	var value interface{}
	if err := json.Unmarshal([]byte(text), &value); err != nil {
		return nil, errInvalid(err.Error())
	}
	members, ok := value.(map[string]interface{})
	if !ok {
		return nil, errInvalid("ticket document must be a JSON object")
	}
	status, ok := members["status"].(string)
	if !ok || (status != string(model.TicketStatusOpen) && status != string(model.TicketStatusClosed)) {
		return nil, errInvalid("ticket status must be explicitly \"open\" or \"closed\"")
	}
	var management *model.TicketManagementData
	if nested, ok := members["github"].(map[string]interface{}); ok {
		management = &model.TicketManagementData{Issue: textMember(nested, "issue")}
	}
	var plan *model.TicketPlan
	if nested, ok := members["plan"].(map[string]interface{}); ok {
		plan = &model.TicketPlan{
			Client: textMember(nested, "client"),
			ID:     textMember(nested, "id"),
			Source: textMember(nested, "source"),
			Local:  textMember(nested, "local"),
		}
	}
	interactions := make([]model.Interaction, 0)
	for _, entry := range objectsOf(members["interactions"]) {
		interactions = append(interactions, decodeInteraction(entry))
	}
	agents := make([]model.TicketAgent, 0)
	for _, entry := range objectsOf(members["agents"]) {
		agents = append(agents, decodeAgent(entry))
	}

	ticket := &model.Ticket{
		Title:        textMember(members, "title"),
		Emoji:        textMember(members, "emoji"),
		Status:       model.TicketStatus(status),
		Description:  textMember(members, "description"),
		Summary:      textMember(members, "summary"),
		Management:   management,
		Goal:         textMember(members, "goal"),
		Parent:       textMember(members, "parent"),
		Plan:         plan,
		Interactions: interactions,
	}
	if entries, ok := members["sessions"].([]interface{}); ok {
		for _, entry := range entries {
			if sessionID, ok := entry.(string); ok {
				AppendSessionID(ticket, sessionID)
			}
		}
	}
	ticket.Agents = agents
	if ticket.Summary == "" {
		for index := len(ticket.Interactions) - 1; index >= 0; index-- {
			interaction := ticket.Interactions[index]
			if IsTicketInteractionKind(interaction.Kind, "ticket.close") && interaction.Summary != "" {
				ticket.Summary = interaction.Summary
				break
			}
		}
	}
	for _, agent := range ticket.Agents {
		AppendSessionID(ticket, agent.Session)
	}
	return ticket, nil
}

// 🖨️EncodeTicketDocument encodes one ticket document, member order and omission rules included.
func EncodeTicketDocument(ticket *model.Ticket) string {
	members := make([]goMember, 0, 12)
	members = append(members, goMember{name: "title", text: ticket.Title})
	if ticket.Emoji != "" {
		members = append(members, goMember{name: "emoji", text: ticket.Emoji})
	}
	members = append(members, goMember{name: "status", text: string(ticket.Status)})
	if ticket.Description != "" {
		members = append(members, goMember{name: "description", text: ticket.Description})
	}
	if ticket.Summary != "" {
		members = append(members, goMember{name: "summary", text: ticket.Summary})
	}
	if ticket.Management != nil {
		nested := make([]goMember, 0, 1)
		if ticket.Management.Issue != "" {
			nested = append(nested, goMember{name: "issue", text: ticket.Management.Issue})
		}
		members = append(members, goMember{name: "github", nested: nested, kind: 1})
	}
	if ticket.Goal != "" {
		members = append(members, goMember{name: "goal", text: ticket.Goal})
	}
	if ticket.Parent != "" { members = append(members, goMember{name: "parent", text: ticket.Parent}) }
	if ticket.Plan != nil {
		nested := make([]goMember, 0, 4)
		if ticket.Plan.Client != "" {
			nested = append(nested, goMember{name: "client", text: ticket.Plan.Client})
		}
		if ticket.Plan.ID != "" {
			nested = append(nested, goMember{name: "id", text: ticket.Plan.ID})
		}
		if ticket.Plan.Source != "" {
			nested = append(nested, goMember{name: "source", text: ticket.Plan.Source})
		}
		if ticket.Plan.Local != "" {
			nested = append(nested, goMember{name: "local", text: ticket.Plan.Local})
		}
		members = append(members, goMember{name: "plan", nested: nested, kind: 1})
	}
	if len(ticket.Sessions) > 0 {
		members = append(members, goMember{name: "sessions", strings: ticket.Sessions, kind: 2})
	}
	if len(ticket.Interactions) > 0 {
		objects := make([][]goMember, 0, len(ticket.Interactions))
		for _, interaction := range ticket.Interactions { objects = append(objects, interactionMembers(interaction)) }
		members = append(members, goMember{name: "interactions", objects: objects, kind: 3})
	}
	if len(ticket.Agents) > 0 {
		objects := make([][]goMember, 0, len(ticket.Agents))
		for _, agent := range ticket.Agents { objects = append(objects, agentMembers(agent)) }
		members = append(members, goMember{name: "agents", objects: objects, kind: 3})
	}
	return renderGoMembers(members, 0)
}

// 🧾️ Encodes durable interaction records in the canonical model field order.
func interactionMembers(interaction model.Interaction) []goMember {
	members := []goMember{{name: "kind", text: interaction.Kind}, {name: "date", text: interaction.Date}, {name: "author", text: interaction.Author}, {name: "system", text: interaction.System}, {name: "client", text: interaction.Client}, {name: "checkpoint", text: interaction.Checkpoint}}
	for _, field := range []goMember{{name: "prompt", text: interaction.Prompt}, {name: "summary", text: interaction.Summary}, {name: "llm", text: interaction.LLM}, {name: "effort", text: interaction.Effort}} { if field.text != "" { members = append(members, field) } }
	if len(interaction.Files) > 0 {
		objects := make([][]goMember, 0, len(interaction.Files))
		for _, file := range interaction.Files {
			fields := []goMember{{name: "path", text: file.Path}}
			if file.ID != "" { fields = append(fields, goMember{name: "id", text: file.ID}) }
			if file.URI != "" { fields = append(fields, goMember{name: "uri", text: file.URI}) }
			objects = append(objects, fields)
		}
		members = append(members, goMember{name: "files", objects: objects, kind: 3})
	}
	return members
}

// 🤖️agentMembers preserves ordered agent metadata and nested plan state.
func agentMembers(agent model.TicketAgent) []goMember {
 members:=[]goMember{{name:"session",text:agent.Session}}
 for _,field:=range []goMember{{name:"contributor",text:agent.Contributor},{name:"system",text:agent.System},{name:"client",text:agent.Client},{name:"llm",text:agent.LLM},{name:"effort",text:agent.Effort},{name:"transcript",text:agent.Transcript}} {if field.text!="" {members=append(members,field)}}
 if agent.Plan!=nil {
  fields:=make([]goMember,0,1)
  if len(agent.Plan.Steps)>0 {
   steps:=make([][]goMember,0,len(agent.Plan.Steps))
   for _,step:=range agent.Plan.Steps {
    row:=[]goMember{{name:"id",text:step.ID},{name:"name",text:step.Name}}
    for _,field:=range []goMember{{name:"description",text:step.Description},{name:"status",text:step.Status},{name:"ideated",text:step.Ideated},{name:"started",text:step.Started},{name:"completed",text:step.Completed},{name:"abandoned",text:step.Abandoned}} {if field.text!="" {row=append(row,field)}}
    steps=append(steps,row)
   }
   fields=append(fields,goMember{name:"steps",objects:steps,kind:3})
  }
  members=append(members,goMember{name:"plan",nested:fields,kind:1})
 }
 return members
}

// ➕️AppendSessionID appends a session id once, ignoring blanks and duplicates.
func AppendSessionID(ticket *model.Ticket, sessionID string) {
	model.AppendTicketSessionID(ticket, sessionID)
}

// 🔍️IsTicketInteractionKind reports whether a kind names an operation, with or without its suffix.
func IsTicketInteractionKind(kind, expected string) bool {
	kind = strings.TrimSpace(kind)
	expected = strings.TrimSpace(expected)
	if kind == expected {
		return true
	}
	return strings.HasSuffix(kind, ".ended") && strings.TrimSuffix(kind, ".ended") == expected
}

// #endregion 📄️Codec
