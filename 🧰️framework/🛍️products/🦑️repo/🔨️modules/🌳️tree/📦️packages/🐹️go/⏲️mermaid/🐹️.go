// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// ⏲️mermaid carries the mermaid treemap and lines-of-code diagram builders.

// #endregion 🧲️Header

package tree

import (
	context "context"
	json "encoding/json"
	fmt "fmt"
	filepath "path/filepath"
	sort "sort"
	strconv "strconv"
	strings "strings"

	codebase "github.com/usalu/semio/repo/codebase"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region ⏲️Mermaid

// 🧜️Mermaid diagram generation for LOC visualizations as treemap-beta strings.
func mermaidEscapeLabel(s string) string {
	return strings.ReplaceAll(s, "\"", "'")
}

func mermaidTechnologyEmoji(kind model.TechnologyKind) string {
	switch kind {
	case model.TechnologyKindInfrastructure:
		return model.EmojiTechnologyInfra
	case model.TechnologyKindResearch:
		return model.EmojiTechnologyResearch
	default:
		return model.EmojiTechnologyUser
	}
}

func mermaidBundleEmoji(kind model.BundleKind) string {
	switch kind {
	case model.BundleKindSchema:
		return model.EmojiBundleSchema
	case model.BundleKindBinary:
		return model.EmojiBundleBinary
	case model.BundleKindUI:
		return model.EmojiBundleUI
	case model.BundleKindSite:
		return model.EmojiBundleSite
	case model.BundleKindAssets:
		return model.EmojiBundleAssets
	default:
		return model.EmojiBundleLibrary
	}
}

func mermaidFileEmoji(kind string) string {
	switch kind {
	case model.FileKindLab:
		return model.EmojiFileLab
	case model.FileKindScript:
		return model.EmojiFileScript
	case model.FileKindDocs:
		return model.EmojiFileDocs
	case model.FileKindConfig:
		return model.EmojiFileConfig
	case model.FileKindResource:
		return model.EmojiFileResource
	case model.FileKindTemplate:
		return model.EmojiFileTemplate
	case model.FileKindLicense:
		return model.EmojiFileLicense
	default:
		return model.EmojiFileCode
	}
}

func MermaidLocByTechnologiesBundlesFoldersFiles() string {
	technologies := codebase.LoadTechnologies()
	ctx := context.Background()
	fileCh := make(chan model.File)
	go func() { ticketspkg.StreamFiles(ctx, "", fileCh) }()
	var allFiles []model.File
	for f := range fileCh {
		if f.Ignored || f.Generated {
			continue
		}
		allFiles = append(allFiles, f)
	}
	type fileEntry struct {
		Name string
		Kind string
		LOC  int
	}
	type folderEntry struct {
		Name  string
		Files []fileEntry
	}
	type bundleEntry struct {
		Name    string
		Kind    model.BundleKind
		Folders map[string]*folderEntry
	}
	type technologyEntry struct {
		Name    string
		Kind    model.TechnologyKind
		Bundles map[string]*bundleEntry
	}
	technologyMap := make(map[string]*technologyEntry)
	for _, p := range technologies {
		pe := &technologyEntry{Name: p.Name, Kind: p.Kind, Bundles: make(map[string]*bundleEntry)}
		for _, b := range p.Bundles {
			pe.Bundles[b.Name] = &bundleEntry{Name: b.Name, Kind: b.Kind, Folders: make(map[string]*folderEntry)}
		}
		technologyMap[p.Name] = pe
	}
	for _, f := range allFiles {
		bundle := model.GetBundleByPath(f.Path)
		if bundle == nil {
			continue
		}
		absPath := filepath.Join(workspace.GetRootDir(), f.Path)
		loc := workspace.CountLinesInFile(absPath)
		if loc == 0 {
			continue
		}
		bundleRoot := workspace.NormalizePath(bundle.Root)
		relPath := strings.TrimPrefix(workspace.NormalizePath(f.Path), bundleRoot+"/")
		folderName := filepath.Dir(relPath)
		if folderName == "." {
			folderName = ""
		}
		parts := strings.SplitN(bundle.Name, "/", 2)
		technologyName := parts[0]
		pe, ok := technologyMap[technologyName]
		if !ok {
			continue
		}
		be, ok := pe.Bundles[bundle.Name]
		if !ok {
			continue
		}
		fe, ok := be.Folders[folderName]
		if !ok {
			fe = &folderEntry{Name: folderName}
			be.Folders[folderName] = fe
		}
		fe.Files = append(fe.Files, fileEntry{Name: f.Name, Kind: f.Kind, LOC: loc})
	}
	var sb strings.Builder
	sb.WriteString("treemap-beta\n")
	sb.WriteString("\"Lines of Code\"\n")
	var technologyNames []string
	for name := range technologyMap {
		technologyNames = append(technologyNames, name)
	}
	sort.Strings(technologyNames)
	for _, pName := range technologyNames {
		pe := technologyMap[pName]
		technologyLOC := 0
		for _, be := range pe.Bundles {
			for _, fe := range be.Folders {
				for _, f := range fe.Files {
					technologyLOC += f.LOC
				}
			}
		}
		if technologyLOC == 0 {
			continue
		}
		emoji := mermaidTechnologyEmoji(pe.Kind)
		sb.WriteString(fmt.Sprintf("    \"%s%s\"\n", emoji, mermaidEscapeLabel(workspace.Flat(pe.Name))))
		var bundleNames []string
		for name := range pe.Bundles {
			bundleNames = append(bundleNames, name)
		}
		sort.Strings(bundleNames)
		for _, bName := range bundleNames {
			be := pe.Bundles[bName]
			bundleLOC := 0
			for _, fe := range be.Folders {
				for _, f := range fe.Files {
					bundleLOC += f.LOC
				}
			}
			if bundleLOC == 0 {
				continue
			}
			bParts := strings.SplitN(bName, "/", 2)
			bundleLabel := bParts[0]
			if len(bParts) > 1 {
				bundleLabel = bParts[1]
			}
			bEmoji := mermaidBundleEmoji(be.Kind)
			sb.WriteString(fmt.Sprintf("        \"%s%s\"\n", bEmoji, mermaidEscapeLabel(workspace.Flat(bundleLabel))))
			var folderNames []string
			for name := range be.Folders {
				folderNames = append(folderNames, name)
			}
			sort.Strings(folderNames)
			for _, fName := range folderNames {
				fe := be.Folders[fName]
				folderLOC := 0
				for _, f := range fe.Files {
					folderLOC += f.LOC
				}
				if folderLOC == 0 {
					continue
				}
				if fName == "" {
					for _, f := range fe.Files {
						fEmoji := mermaidFileEmoji(f.Kind)
						sb.WriteString(fmt.Sprintf("            \"%s%s\": %d\n", fEmoji, mermaidEscapeLabel(f.Name), f.LOC))
					}
				} else {
					sb.WriteString(fmt.Sprintf("            \"%s%s\"\n", model.EmojiFolderOrg, mermaidEscapeLabel(filepath.Base(fName))))
					sort.Slice(fe.Files, func(i, j int) bool { return fe.Files[i].Name < fe.Files[j].Name })
					for _, f := range fe.Files {
						fEmoji := mermaidFileEmoji(f.Kind)
						sb.WriteString(fmt.Sprintf("                \"%s%s\": %d\n", fEmoji, mermaidEscapeLabel(f.Name), f.LOC))
					}
				}
			}
		}
	}
	return sb.String()
}

func MermaidLocByContributors() string {
	bundles := codebase.GetTechnologies()
	files, _ := model.ScopeToFiles(workspace.GetRootDir(), workspace.Scope{Kind: workspace.ScopeRepo}, bundles)
	authorLines := make(map[string]int)
	for _, file := range files {
		absPath := filepath.Join(workspace.GetRootDir(), file)
		stdout, _, exitCode := workspace.ExecCommand("git", []string{"blame", "--line-porcelain", absPath}, workspace.GetRootDir())
		if exitCode != 0 {
			continue
		}
		for _, line := range strings.Split(stdout, "\n") {
			if strings.HasPrefix(line, "author ") {
				author := strings.TrimPrefix(line, "author ")
				authorLines[author]++
			}
		}
	}
	var sb strings.Builder
	sb.WriteString("treemap-beta\n")
	sb.WriteString("\"Lines of Code by Contributor\"\n")
	type authorEntry struct {
		Name string
		LOC  int
	}
	var authors []authorEntry
	for name, loc := range authorLines {
		authors = append(authors, authorEntry{Name: name, LOC: loc})
	}
	sort.Slice(authors, func(i, j int) bool { return authors[i].LOC > authors[j].LOC })
	for _, a := range authors {
		sb.WriteString(fmt.Sprintf("    \"%s\": %d\n", mermaidEscapeLabel(a.Name), a.LOC))
	}
	return sb.String()
}

func MermaidLocByLanguage() string {
	ctx := context.Background()
	fileCh := make(chan model.File)
	go func() { ticketspkg.StreamFiles(ctx, "", fileCh) }()
	langLines := make(map[string]int)
	for f := range fileCh {
		if f.Ignored || f.Generated {
			continue
		}
		lang := languages.GetLanguage(f.Path)
		if lang == nil {
			continue
		}
		absPath := filepath.Join(workspace.GetRootDir(), f.Path)
		loc := workspace.CountLinesInFile(absPath)
		if loc == 0 {
			continue
		}
		langLines[lang.Name()] += loc
	}
	var sb strings.Builder
	sb.WriteString("treemap-beta\n")
	sb.WriteString("\"Lines of Code by Language\"\n")
	type langEntry struct {
		Name string
		LOC  int
	}
	var langs []langEntry
	for name, loc := range langLines {
		langs = append(langs, langEntry{Name: name, LOC: loc})
	}
	sort.Slice(langs, func(i, j int) bool { return langs[i].LOC > langs[j].LOC })
	for _, l := range langs {
		sb.WriteString(fmt.Sprintf("    \"%s\": %d\n", mermaidEscapeLabel(l.Name), l.LOC))
	}
	return sb.String()
}

// #endregion ⏲️Mermaid

// #region 🧜️PortedMermaid

// 🧜️MermaidNode is one node of a Mermaid treemap-beta diagram.
type MermaidNode struct {
	Label    string        `json:"label"`
	Value    *int64        `json:"value,omitempty"`
	Children []MermaidNode `json:"children,omitempty"`
}

// 🧜️MermaidTreemap is a Mermaid treemap-beta diagram.
type MermaidTreemap struct {
	Title string        `json:"title"`
	Nodes []MermaidNode `json:"nodes"`
}

// 📥️ParseMermaidTreemap MUST return an error when the document is malformed.
func ParseMermaidTreemap(document []byte) (*MermaidTreemap, error) {
	treemap := &MermaidTreemap{}
	if err := json.Unmarshal(document, treemap); err != nil {
		return nil, err
	}
	return treemap, nil
}

// 🔤️MermaidEscapeLabel MUST turn every double quote into an apostrophe.
func MermaidEscapeLabel(value string) string {
	return strings.ReplaceAll(value, "\"", "'")
}

// 🧜️RenderMermaidTreemap MUST render the header, the quoted title and the indented node forest.
func RenderMermaidTreemap(treemap *MermaidTreemap) string {
	var out strings.Builder
	out.WriteString("treemap-beta\n")
	out.WriteString("\"" + MermaidEscapeLabel(treemap.Title) + "\"\n")
	for index := range treemap.Nodes {
		renderMermaidNode(&out, &treemap.Nodes[index], 1)
	}
	return out.String()
}

// 🧜️renderMermaidNode renders one treemap node at a depth.
func renderMermaidNode(out *strings.Builder, node *MermaidNode, depth int) {
	indent := strings.Repeat(TextBlankIndent, depth)
	if node.Value != nil && len(node.Children) == 0 {
		out.WriteString(indent + "\"" + MermaidEscapeLabel(node.Label) + "\": " + strconv.FormatInt(*node.Value, 10) + "\n")
	} else {
		out.WriteString(indent + "\"" + MermaidEscapeLabel(node.Label) + "\"\n")
	}
	for index := range node.Children {
		renderMermaidNode(out, &node.Children[index], depth+1)
	}
}

// 🌳️MermaidTreemapFromTree MUST project a monorepo tree into a treemap.
func MermaidTreemapFromTree(root *TreeNode, title string, weightKey string) *MermaidTreemap {
	treemap := &MermaidTreemap{Title: title}
	for _, child := range root.Children {
		treemap.Nodes = append(treemap.Nodes, mermaidNodeFromTree(child, weightKey))
	}
	return treemap
}

// 🌿️mermaidNodeFromTree projects one tree node into a treemap node.
func mermaidNodeFromTree(node *TreeNode, weightKey string) MermaidNode {
	out := MermaidNode{Label: node.Label}
	if node.Data != nil {
		switch value := node.Data[weightKey].(type) {
		case float64:
			whole := int64(value)
			out.Value = &whole
		case int:
			whole := int64(value)
			out.Value = &whole
		case int64:
			whole := value
			out.Value = &whole
		case json.Number:
			if whole, err := value.Int64(); err == nil {
				out.Value = &whole
			}
		}
	}
	for _, child := range node.Children {
		out.Children = append(out.Children, mermaidNodeFromTree(child, weightKey))
	}
	return out
}

// #endregion 🧜️PortedMermaid
