// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Mermaid diagram renderers and command.

// #endregion 🧲️Header

package client

import (
	"context"
	"fmt"
	command "github.com/usalu/semio/repo/client/internal/command"
	"path/filepath"
	"sort"
	"strings"
)

func mermaidEscapeLabel(s string) string {
	return strings.ReplaceAll(s, "\"", "'")
}

func mermaidTechnologyEmoji(kind TechnologyKind) string {
	switch kind {
	case TechnologyKindInfrastructure:
		return EmojiTechnologyInfra
	case TechnologyKindResearch:
		return EmojiTechnologyResearch
	default:
		return EmojiTechnologyUser
	}
}

func mermaidBundleEmoji(kind BundleKind) string {
	switch kind {
	case BundleKindSchema:
		return EmojiBundleSchema
	case BundleKindBinary:
		return EmojiBundleBinary
	case BundleKindUI:
		return EmojiBundleUI
	case BundleKindSite:
		return EmojiBundleSite
	case BundleKindAssets:
		return EmojiBundleAssets
	default:
		return EmojiBundleLibrary
	}
}

func mermaidFileEmoji(kind string) string {
	switch kind {
	case FileKindLab:
		return EmojiFileLab
	case FileKindScript:
		return EmojiFileScript
	case FileKindDocs:
		return EmojiFileDocs
	case FileKindConfig:
		return EmojiFileConfig
	case FileKindResource:
		return EmojiFileResource
	case FileKindTemplate:
		return EmojiFileTemplate
	case FileKindLicense:
		return EmojiFileLicense
	default:
		return EmojiFileCode
	}
}

func MermaidLocByTechnologiesBundlesFoldersFiles() string {
	technologies := LoadTechnologies()
	ctx := context.Background()
	fileCh := make(chan File)
	go func() { StreamFiles(ctx, "", fileCh) }()
	var allFiles []File
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
		Kind    BundleKind
		Folders map[string]*folderEntry
	}
	type technologyEntry struct {
		Name    string
		Kind    TechnologyKind
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
		bundle := GetBundleByPath(f.Path)
		if bundle == nil {
			continue
		}
		absPath := filepath.Join(GetRootDir(), f.Path)
		loc := CountLinesInFile(absPath)
		if loc == 0 {
			continue
		}
		bundleRoot := NormalizePath(bundle.Root)
		relPath := strings.TrimPrefix(NormalizePath(f.Path), bundleRoot+"/")
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
		sb.WriteString(fmt.Sprintf("    \"%s%s\"\n", emoji, mermaidEscapeLabel(Flat(pe.Name))))
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
			sb.WriteString(fmt.Sprintf("        \"%s%s\"\n", bEmoji, mermaidEscapeLabel(Flat(bundleLabel))))
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
					sb.WriteString(fmt.Sprintf("            \"%s%s\"\n", EmojiFolderOrg, mermaidEscapeLabel(filepath.Base(fName))))
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
	bundles := GetTechnologies()
	files, _ := ScopeToFiles(Scope{Kind: ScopeRepo}, bundles)
	authorLines := make(map[string]int)
	for _, file := range files {
		absPath := filepath.Join(GetRootDir(), file)
		stdout, _, exitCode := ExecCommand("git", []string{"blame", "--line-porcelain", absPath}, GetRootDir())
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
	fileCh := make(chan File)
	go func() { StreamFiles(ctx, "", fileCh) }()
	langLines := make(map[string]int)
	for f := range fileCh {
		if f.Ignored || f.Generated {
			continue
		}
		lang := GetLanguage(f.Path)
		if lang == nil {
			continue
		}
		absPath := filepath.Join(GetRootDir(), f.Path)
		loc := CountLinesInFile(absPath)
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

func mermaidCommand(factory EngineFactory, config *Config) *command.Command {
	root := &command.Command{
		Use:   "mermaid <visualization>",
		Short: "Generate mermaid diagram strings",
	}
	root.AddCommand(&command.Command{
		Use:   "loc-by-technologies-bundles-folders-files",
		Short: "LOC treemap grouped by technology, bundle, folder, file",
		Args:  command.NoArgs,
		RunE: func(cmd *command.Command, args []string) error {
			fmt.Fprint(cmd.OutOrStdout(), MermaidLocByTechnologiesBundlesFoldersFiles())
			return nil
		},
	})
	root.AddCommand(&command.Command{
		Use:   "loc-by-contributors",
		Short: "LOC treemap grouped by contributor (via git blame)",
		Args:  command.NoArgs,
		RunE: func(cmd *command.Command, args []string) error {
			fmt.Fprint(cmd.OutOrStdout(), MermaidLocByContributors())
			return nil
		},
	})
	root.AddCommand(&command.Command{
		Use:   "loc-by-language",
		Short: "LOC treemap grouped by programming language",
		Args:  command.NoArgs,
		RunE: func(cmd *command.Command, args []string) error {
			fmt.Fprint(cmd.OutOrStdout(), MermaidLocByLanguage())
			return nil
		},
	})
	return root
}
