// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🖼️source_helpers carries the identifier, emoji and field helpers of the source projections.

// #endregion 🧲️Header

package graphql

import (
	json "encoding/json"
	fmt "fmt"
	strconv "strconv"
	strings "strings"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🖼️Source Helpers

/** 🖼️ Serialises a domain value into the JSON the executor reads fields off. */
func toJSON(value interface{}) interface{} {
	encoded, err := json.Marshal(value)
	if err != nil {
		return nil
	}
	var decoded interface{}
	if json.Unmarshal(encoded, &decoded) != nil {
		return nil
	}
	return decoded
}

/** 🎫️ The ticket record without the domain type's own inference, so a fixture round-trips verbatim. */
type plainTicket model.Ticket

/** 🎯️ The goal record without the domain type's own inference, so a fixture round-trips verbatim. */
type plainGoal model.Goal

/** 🎫️ The object body of one ticket, read off its declared fields alone. */
func ticketBody(ticket *model.Ticket) map[string]interface{} { return body((*plainTicket)(ticket)) }

/** 🎯️ The object body of one goal, read off its declared fields alone. */
func goalBody(goal *model.Goal) map[string]interface{} { return body((*plainGoal)(goal)) }

/** 🖼️ The object body of a serialised domain value, empty when it is not an object. */
func body(value interface{}) map[string]interface{} {
	if fields, ok := toJSON(value).(map[string]interface{}); ok {
		return fields
	}
	return map[string]interface{}{}
}

/** 🏷️ Stamps the concrete type name an interface or union member is discriminated by. */
func tagged(fields map[string]interface{}, typename string) interface{} {
	fields["__typename"] = typename
	return fields
}

/** 🏷️ The emoji one entity kind is registered under, with its presentation selector normalised. */
func entityEmoji(kind string) string {
	switch kind {
	case "technology-user":
		return model.EmojiTechnologyUser
	case "technology-infrastructure":
		return model.EmojiTechnologyInfra
	case "technology-research":
		return model.EmojiTechnologyResearch
	case "technology-mono":
		return model.EmojiTechnologyMono
	case "bundle-library":
		return model.EmojiBundleLibrary
	case "bundle-schema":
		return model.EmojiBundleSchema
	case "bundle-binary":
		return model.EmojiBundleBinary
	case "bundle-ui":
		return model.EmojiBundleUI
	case "bundle-site":
		return model.EmojiBundleSite
	case "bundle-assets":
		return model.EmojiBundleAssets
	case "bundle-repo":
		return model.EmojiBundleRepo
	case "folder-organization":
		return model.EmojiFolderOrg
	case "folder-required":
		return model.EmojiFolderRequired
	case "folder-root":
		return model.EmojiFolderRoot
	case "file-code":
		return model.EmojiFileCode
	case "file-lab":
		return model.EmojiFileLab
	case "file-script":
		return model.EmojiFileScript
	case "file-docs":
		return model.EmojiFileDocs
	case "file-config":
		return model.EmojiFileConfig
	case "file-resource":
		return model.EmojiFileResource
	case "file-template":
		return model.EmojiFileTemplate
	case "file-license":
		return model.EmojiFileLicense
	case "definition-impl":
		return model.EmojiDefinitionImpl
	case "definition-interface":
		return model.EmojiDefinitionInterface
	case "definition-constant":
		return model.EmojiDefinitionConstant
	case "definition-test":
		return model.EmojiDefinitionTest
	case "section":
		return model.EmojiSection
	case "ticket":
		return model.EmojiTicket
	case "goal":
		return model.EmojiGoal
	case "draft":
		return model.EmojiDraft
	case "todo":
		return model.EmojiTodo
	case "policy":
		return model.EmojiPolicy
	case "breach":
		return model.EmojiBreach
	case "contributor":
		return model.EmojiContributor
	case "checkpoint":
		return model.EmojiCheckpoint
	case "statute":
		return model.EmojiStatute
	default:
		return ""
	}
}

/** 📁️ Joins a repository-relative path onto the root, always with forward slashes. */
func fileURI(rootDir string, relative string) string {
	root := strings.ReplaceAll(rootDir, "\\", "/")
	tail := strings.ReplaceAll(relative, "\\", "/")
	if tail == "" {
		return "file://" + root
	}
	return "file://" + strings.TrimRight(root, "/") + "/" + strings.TrimLeft(tail, "/")
}

/** 🔤️ Title-cases one dash separated slug, word by word. */
func titleizeSlug(slug string) string {
	words := strings.Split(slug, "-")
	titled := make([]string, 0, len(words))
	for _, word := range words {
		if word == "" {
			titled = append(titled, "")
			continue
		}
		runes := []rune(word)
		titled = append(titled, strings.ToUpper(string(runes[0]))+strings.ToLower(string(runes[1:])))
	}
	return strings.Join(titled, " ")
}

/** 📜️ The identifier value of a statute path: every segment titleized, joined with `#`. */
func statutePathToIDValue(path string) string {
	segments := strings.Split(path, "/")
	titled := make([]string, 0, len(segments))
	for _, segment := range segments {
		titled = append(titled, titleizeSlug(segment))
	}
	return strings.Join(titled, "#")
}

/** 🏷️ The emoji a technology name derives when the record carries none. */
func technologyEmoji(name string) string {
	if strings.Contains(name, "repo") {
		return entityEmoji("technology-infrastructure")
	}
	if strings.HasPrefix(name, "coda") {
		return entityEmoji("technology-research")
	}
	return ""
}

/** 📜️ The identifier of a technology. */
func technologyID(technology *model.Technology) string {
	emoji := technology.Emoji
	if emoji == "" {
		emoji = technologyEmoji(technology.Name)
	}
	return model.EmojiText(emoji) + workspace.Flat(technology.Name)
}

/** 🔵️ The emoji a bundle kind stands for. */
func bundleKindEmoji(kind model.BundleKind) string {
	switch kind {
	case model.BundleKindLibrary:
		return entityEmoji("bundle-library")
	case model.BundleKindSchema:
		return entityEmoji("bundle-schema")
	case model.BundleKindBinary:
		return entityEmoji("bundle-binary")
	case model.BundleKindUI:
		return entityEmoji("bundle-ui")
	case model.BundleKindSite:
		return entityEmoji("bundle-site")
	case model.BundleKindAssets:
		return entityEmoji("bundle-assets")
	case model.BundleKindRepo:
		return entityEmoji("bundle-repo")
	default:
		return ""
	}
}

/** 🔵️ The identifier of a bundle: technology emoji and code, then bundle emoji and code. */
func bundleID(bundle *model.Bundle, technologies []*model.Technology) string {
	emoji := bundle.Emoji
	if emoji == "" {
		emoji = bundleKindEmoji(bundle.Kind)
	}
	technologyCode, bundleCode := bundle.Name, bundle.Name
	if head, tail, found := strings.Cut(bundle.Name, "/"); found {
		technologyCode, bundleCode = head, tail
	}
	technologyEmojiValue := string(model.DeriveTechnologyKind(technologyCode))
	for _, candidate := range technologies {
		if candidate.Name == technologyCode && candidate.Emoji != "" {
			technologyEmojiValue = candidate.Emoji
			break
		}
	}
	return model.EmojiText(technologyEmojiValue) + workspace.Flat(technologyCode) + model.EmojiText(emoji) + workspace.Flat(bundleCode)
}

/** 💗️ The identifier of a section, falling back to the emoji form when it carries none. */
func sectionID(section *model.Section) string {
	if section.ID != "" {
		return section.ID
	}
	emoji := section.Emoji
	if emoji == "" {
		emoji = entityEmoji("section")
	}
	return model.EmojiText(emoji) + workspace.Flat(section.Name)
}

/** 💕️ The emoji a definition kind stands for. */
func definitionKindEmojiOf(kind model.DefinitionKind) string {
	switch kind {
	case model.DefinitionKindImplementation:
		return entityEmoji("definition-impl")
	case model.DefinitionKindInterface:
		return entityEmoji("definition-interface")
	case model.DefinitionKindConstant:
		return entityEmoji("definition-constant")
	case model.DefinitionKindTest:
		return entityEmoji("definition-test")
	default:
		return ""
	}
}

/** 💕️ The identifier of a definition, falling back to the emoji form when it carries none. */
func definitionID(definition *model.Definition) string {
	if definition.ID != "" {
		return definition.ID
	}
	return model.EmojiText(definitionKindEmojiOf(definition.Kind)) + workspace.Flat(definition.Name)
}

/** 🕰️ Normalises a recorded timestamp into the RFC 3339 rendering the `DateTime` scalar emits. */
func toRFC3339(raw string) (string, bool) {
	trimmed := strings.TrimSpace(raw)
	if len(trimmed) < 19 || (trimmed[10] != ' ' && trimmed[10] != 'T') {
		return "", false
	}
	date, rest := trimmed[:10], trimmed[11:]
	clock := rest
	if index := strings.IndexAny(rest, "+Z."); index >= 0 {
		clock = rest[:index]
	}
	if len(clock) != 8 {
		return "", false
	}
	return date + "T" + clock + "Z", true
}

/** 🕰️ Midnight UTC of a year/month/day triple, the zero date a ticket falls back to. */
func midnight(year, month, day int) string {
	return fmt.Sprintf("%04d-%02d-%02dT00:00:00Z", year, month, day)
}

/** 🎫️ Whether an interaction kind is the expected one, tolerating the `.ended` suffix. */
func isTicketInteractionKind(kind string, expected string) bool {
	trimmed := strings.TrimSpace(kind)
	want := strings.TrimSpace(expected)
	return trimmed == want || strings.TrimSuffix(trimmed, ".ended") == want && strings.HasSuffix(trimmed, ".ended")
}

/** 🎫️ The date a ticket started: its open interaction, else its first, else midnight. */
func ticketStarted(ticket *model.Ticket) string {
	for index := range ticket.Interactions {
		if !isTicketInteractionKind(ticket.Interactions[index].Kind, "ticket.open") {
			continue
		}
		if date, ok := toRFC3339(ticket.Interactions[index].Date); ok {
			return date
		}
	}
	if len(ticket.Interactions) > 0 {
		if date, ok := toRFC3339(ticket.Interactions[0].Date); ok {
			return date
		}
	}
	return midnight(ticket.Year, ticket.Month, ticket.Day)
}

/** 🎫️ The date a ticket finished: its last close interaction, or none. */
func ticketFinished(ticket *model.Ticket) interface{} {
	for index := len(ticket.Interactions) - 1; index >= 0; index-- {
		if !isTicketInteractionKind(ticket.Interactions[index].Kind, "ticket.close") {
			continue
		}
		if date, ok := toRFC3339(ticket.Interactions[index].Date); ok {
			return date
		}
		return nil
	}
	return nil
}

/** 🎫️ The last interaction's field, then the last agent's, then blank. */
func ticketLatest(ticket *model.Ticket, pick func(*model.Interaction) string, fallback func(*model.TicketAgent) string) string {
	if len(ticket.Interactions) > 0 {
		return pick(&ticket.Interactions[len(ticket.Interactions)-1])
	}
	if len(ticket.Agents) > 0 {
		return fallback(&ticket.Agents[len(ticket.Agents)-1])
	}
	return ""
}

/** 🎫️ The author of a ticket: its first interaction's, then its first agent's. */
func ticketAuthor(ticket *model.Ticket) string {
	if len(ticket.Interactions) > 0 {
		return ticket.Interactions[0].Author
	}
	if len(ticket.Agents) > 0 {
		return ticket.Agents[0].Contributor
	}
	return ""
}

/** 🎫️ The checkpoint of a ticket: its first interaction's. */
func ticketCheckpoint(ticket *model.Ticket) string {
	if len(ticket.Interactions) > 0 {
		return ticket.Interactions[0].Checkpoint
	}
	return ""
}

/** 🔤️ Wraps a non-empty string as JSON, mapping the empty string to null. */
func optional(value string) interface{} {
	if value == "" {
		return nil
	}
	return value
}

/** 🤝️ Splits `Name <email>` the way the reference implementation does. */
func parseGitAuthor(value string) (string, string) {
	name, rest, found := strings.Cut(value, " <")
	if !found {
		return value, ""
	}
	return strings.TrimSpace(name), strings.TrimSuffix(rest, ">")
}

/** 🟡️ The milestone number a management link carries. */
func parseMilestoneNumber(milestone string) interface{} {
	if number, err := strconv.Atoi(milestone); err == nil {
		return number
	}
	segments := strings.Split(milestone, "/")
	if number, err := strconv.Atoi(segments[len(segments)-1]); err == nil {
		return number
	}
	return nil
}

/** 📃️ Wraps a string slice as a JSON array, never as null. */
func stringArray(values []string) []interface{} {
	rows := make([]interface{}, 0, len(values))
	for _, value := range values {
		rows = append(rows, value)
	}
	return rows
}

// #endregion 🖼️Source Helpers
