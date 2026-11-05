package launcher

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"unicode/utf16"
)

// 🏷️WorkspaceNamespaceClassification identifies the bootstrap phase without granting dependency readiness or content authority.
type WorkspaceNamespaceClassification struct {
	Kind            string `json:"kind"`
	Authority       bool   `json:"authority"`
	Readiness       bool   `json:"readiness"`
	ContentObserved bool   `json:"contentObserved"`
}

type workspaceManagedContract struct {
	generationRoot, receiptName, rootEndpoint, rootScope, readyPurpose                                                                                string
	prefixUnits, readyVersion, maximumMembers, maximumBindings, maximumReferences, maximumSingletons, maximumSlot, maximumNameUnits, maximumPathUnits int
	maximumReceiptBytes                                                                                                                               int64
}

type workspaceManagedReady struct {
	Version       int    `json:"version"`
	Purpose       string `json:"purpose"`
	Scope         string `json:"scope"`
	Generation    string `json:"generation"`
	OwnerRelation string `json:"ownerRelationSha256"`
	Group         string `json:"groupSha256"`
	Recipe        string `json:"recipeSha256"`
	Members       []struct {
		Path       string `json:"path"`
		Member     string `json:"member"`
		Slot       *int   `json:"slot"`
		Owner      string `json:"owner"`
		Generation string `json:"ownerGeneration"`
	} `json:"members"`
	Bindings []struct {
		Source string `json:"sourceDirectory"`
		Target string `json:"targetDirectory"`
	} `json:"bindings"`
	References []struct {
		Scope      string `json:"scope"`
		Generation string `json:"generation"`
	} `json:"ownerReferences"`
	Singletons []struct {
		Name       string `json:"name"`
		Owner      string `json:"owner"`
		Generation string `json:"ownerGeneration"`
	} `json:"singletons"`
}

func workspaceRelativeMetadata(root, path string, into any) error {
	if path == "" || strings.ContainsAny(path, "\\:\x00\r\n") || strings.HasPrefix(path, "/") {
		return fmt.Errorf("relative declared workspace metadata required")
	}
	for _, part := range strings.Split(path, "/") {
		if part == "" || part == "." || part == ".." {
			return fmt.Errorf("canonical workspace metadata path required")
		}
	}
	return readWorkspaceManifest(filepath.Join(root, filepath.FromSlash(path)), 1<<20, into)
}

func workspaceSchemaNumber(raw json.RawMessage, key string) int {
	var fields map[string]json.RawMessage
	if json.Unmarshal(raw, &fields) != nil {
		return -1
	}
	var value int
	if json.Unmarshal(fields[key], &value) != nil {
		return -1
	}
	return value
}

func workspaceSchemaItemProperty(raw json.RawMessage, key string) json.RawMessage {
	var definition struct {
		Items struct {
			Properties map[string]json.RawMessage `json:"properties"`
		} `json:"items"`
	}
	if json.Unmarshal(raw, &definition) != nil {
		return nil
	}
	return definition.Items.Properties[key]
}

func workspaceAtom(value string) bool {
	return value != "" && value != "." && value != ".." && !strings.ContainsAny(value, "/\\:\x00\r\n")
}

func readWorkspaceManagedContract(root string, policy WorkspaceLinksPolicy) (workspaceManagedContract, error) {
	contract := workspaceManagedContract{}
	var source struct {
		Defs map[string]json.RawMessage `json:"$defs"`
	}
	if err := workspaceRelativeMetadata(policy.metadataRoot, policy.Managed.SourceSchema, &source); err != nil {
		return contract, err
	}
	var protocol struct {
		Const struct {
			Version      int    `json:"version"`
			Prefix       int    `json:"generationPrefixUnits"`
			Receipt      string `json:"receiptName"`
			Endpoint     string `json:"rootEndpoint"`
			Scope        string `json:"rootScope"`
			MaximumBytes int64  `json:"maximumReceiptBytes"`
			Authority    bool   `json:"authority"`
			Readiness    bool   `json:"readiness"`
			Content      bool   `json:"contentObserved"`
		} `json:"const"`
	}
	if err := json.Unmarshal(source.Defs["SourceNamespaceClassificationProtocol"], &protocol); err != nil {
		return contract, err
	}
	p := protocol.Const
	if p.Version != 1 || p.Prefix < 1 || p.Prefix > 64 || !workspaceAtom(p.Receipt) || p.Endpoint != policy.Directory || p.Scope != "." || p.MaximumBytes < 1 || p.MaximumBytes > 32<<20 || p.Authority || p.Readiness || p.Content {
		return contract, fmt.Errorf("non-authoritative managed workspace classification protocol required")
	}
	var ready struct {
		Properties map[string]json.RawMessage `json:"properties"`
	}
	if err := json.Unmarshal(source.Defs["SourceOwnerReady"], &ready); err != nil {
		return contract, err
	}
	var purpose struct {
		Const string `json:"const"`
	}
	if err := json.Unmarshal(ready.Properties["purpose"], &purpose); err != nil {
		return contract, err
	}
	contract.receiptName, contract.rootEndpoint, contract.rootScope, contract.prefixUnits, contract.maximumReceiptBytes = p.Receipt, p.Endpoint, p.Scope, p.Prefix, p.MaximumBytes
	contract.readyVersion = workspaceSchemaNumber(ready.Properties["version"], "const")
	contract.readyPurpose = purpose.Const
	contract.maximumPathUnits = workspaceSchemaNumber(ready.Properties["scope"], "maxLength")
	contract.maximumMembers = workspaceSchemaNumber(ready.Properties["members"], "maxItems")
	contract.maximumBindings = workspaceSchemaNumber(ready.Properties["bindings"], "maxItems")
	contract.maximumReferences = workspaceSchemaNumber(ready.Properties["ownerReferences"], "maxItems")
	contract.maximumSingletons = workspaceSchemaNumber(ready.Properties["singletons"], "maxItems")
	contract.maximumSlot = workspaceSchemaNumber(workspaceSchemaItemProperty(ready.Properties["members"], "slot"), "maximum")
	contract.maximumNameUnits = workspaceSchemaNumber(workspaceSchemaItemProperty(ready.Properties["singletons"], "name"), "maxLength")
	if contract.readyVersion < 1 || contract.readyPurpose == "" || contract.maximumPathUnits != policy.MaximumPathLength || contract.maximumMembers < 1 || contract.maximumMembers > 4096 || contract.maximumBindings < 1 || contract.maximumBindings > 4096 || contract.maximumReferences < 0 || contract.maximumReferences > 4096 || contract.maximumSingletons < 0 || contract.maximumSingletons > 4096 || contract.maximumSlot < 0 || contract.maximumSlot >= contract.maximumMembers || contract.maximumNameUnits < 1 || contract.maximumNameUnits > policy.MaximumPathLength {
		return contract, fmt.Errorf("bounded current managed workspace receipt shape required")
	}
	var selection struct {
		Version   int `json:"version"`
		Selection struct {
			Javascript string `json:"javascript"`
		} `json:"selection"`
	}
	var catalog struct {
		Version  int               `json:"version"`
		Adapters map[string]string `json:"adapters"`
	}
	if err := workspaceRelativeMetadata(policy.metadataRoot, policy.Managed.ManagerDirectory+"/🔣️policy.json", &selection); err != nil {
		return contract, err
	}
	if err := workspaceRelativeMetadata(policy.metadataRoot, policy.Managed.ManagerDirectory+"/🔣️catalog.json", &catalog); err != nil {
		return contract, err
	}
	adapter := catalog.Adapters[selection.Selection.Javascript]
	if selection.Version != 1 || catalog.Version != 1 || !workspaceAtom(adapter) {
		return contract, fmt.Errorf("declared JavaScript manager classification required")
	}
	var manager struct {
		Version             int    `json:"version"`
		ID                  string `json:"id"`
		Ecosystem           string `json:"ecosystem"`
		GenerationDirectory string `json:"generationDirectory"`
		Installed           struct {
			Containers []string `json:"containers"`
		} `json:"installed"`
	}
	if err := workspaceRelativeMetadata(policy.metadataRoot, policy.Managed.ManagerDirectory+"/"+adapter+"/🔣️.json", &manager); err != nil {
		return contract, err
	}
	hasEndpoint := false
	for _, entry := range manager.Installed.Containers {
		hasEndpoint = hasEndpoint || entry == p.Endpoint
	}
	if manager.Version != 1 || manager.ID != selection.Selection.Javascript || manager.Ecosystem != "javascript" || !workspaceAtom(manager.GenerationDirectory) || !hasEndpoint {
		return contract, fmt.Errorf("matching JavaScript manager generation layout required")
	}
	var budget struct {
		Partitions map[string]struct {
			Directory string `json:"directory"`
		} `json:"partitions"`
	}
	var native struct {
		Definitions struct {
			ImageProtocol struct {
				Const struct {
					MountDirectory string `json:"mountDirectory"`
				} `json:"const"`
			} `json:"ImageProtocol"`
		} `json:"definitions"`
	}
	if err := workspaceRelativeMetadata(policy.metadataRoot, policy.Managed.BudgetPolicy, &budget); err != nil {
		return contract, err
	}
	if err := workspaceRelativeMetadata(policy.metadataRoot, policy.Managed.NativeSchema, &native); err != nil {
		return contract, err
	}
	directory, mount := budget.Partitions["dependencies"].Directory, native.Definitions.ImageProtocol.Const.MountDirectory
	if directory == "" || strings.ContainsAny(directory, "\\:\x00\r\n") || strings.HasPrefix(directory, "/") || !workspaceAtom(mount) {
		return contract, fmt.Errorf("declared dependency partition classification required")
	}
	for _, part := range strings.Split(directory, "/") {
		if !workspaceAtom(part) {
			return contract, fmt.Errorf("canonical dependency partition path required")
		}
	}
	contract.generationRoot = filepath.Join(root, filepath.FromSlash(directory), mount, manager.GenerationDirectory)
	return contract, nil
}

func workspacePathEqual(left, right string) bool {
	if runtime.GOOS == "windows" {
		return strings.EqualFold(filepath.Clean(left), filepath.Clean(right))
	}
	return filepath.Clean(left) == filepath.Clean(right)
}

func workspaceCanonicalDirectory(path string) (string, error) {
	facts, err := os.Stat(path)
	if err != nil {
		return "", err
	}
	if !facts.IsDir() {
		return "", fmt.Errorf("physical managed namespace directory required")
	}
	return canonicalRoot(path), nil
}

func workspaceReadyShape(ready workspaceManagedReady, contract workspaceManagedContract) bool {
	text := func(value string, maximum int) bool {
		return value != "" && len(utf16.Encode([]rune(value))) <= maximum
	}
	if ready.Version != contract.readyVersion || ready.Purpose != contract.readyPurpose || ready.Scope != contract.rootScope || !shaPattern.MatchString(ready.Generation) || !shaPattern.MatchString(ready.OwnerRelation) || !shaPattern.MatchString(ready.Group) || !shaPattern.MatchString(ready.Recipe) || ready.Members == nil || len(ready.Members) > contract.maximumMembers || len(ready.Bindings) < 1 || len(ready.Bindings) > contract.maximumBindings || ready.References == nil || len(ready.References) > contract.maximumReferences || ready.Singletons == nil || len(ready.Singletons) > contract.maximumSingletons {
		return false
	}
	for _, member := range ready.Members {
		if !text(member.Path, contract.maximumPathUnits) || !text(member.Member, contract.maximumPathUnits) || member.Slot == nil || *member.Slot < 0 || *member.Slot > contract.maximumSlot || !text(member.Owner, contract.maximumPathUnits) || !shaPattern.MatchString(member.Generation) {
			return false
		}
	}
	for _, binding := range ready.Bindings {
		if !text(binding.Source, contract.maximumPathUnits) || !text(binding.Target, contract.maximumPathUnits) || !filepath.IsAbs(binding.Source) || !filepath.IsAbs(binding.Target) {
			return false
		}
	}
	for _, reference := range ready.References {
		if !text(reference.Scope, contract.maximumPathUnits) || !shaPattern.MatchString(reference.Generation) {
			return false
		}
	}
	for _, singleton := range ready.Singletons {
		if !text(singleton.Name, contract.maximumNameUnits) || !text(singleton.Owner, contract.maximumPathUnits) || !shaPattern.MatchString(singleton.Generation) {
			return false
		}
	}
	return true
}

// 🧊️ClassifyWorkspaceNamespace recognizes an exact managed binding and leaves dependency readiness exclusively to its existing owner.
func ClassifyWorkspaceNamespace(root string, policy WorkspaceLinksPolicy) (WorkspaceNamespaceClassification, error) {
	classification := WorkspaceNamespaceClassification{}
	endpoint := filepath.Join(root, policy.Directory)
	facts, err := os.Lstat(endpoint)
	if os.IsNotExist(err) {
		classification.Kind = "cold"
		return classification, nil
	}
	if err != nil {
		return classification, err
	}
	link := workspaceDirectoryLink(endpoint, facts)
	if facts.IsDir() && !link {
		classification.Kind = "unmanaged"
		return classification, nil
	}
	if !link {
		return classification, fmt.Errorf("workspace package namespace is not a directory")
	}
	contract, err := readWorkspaceManagedContract(root, policy)
	if err != nil {
		return classification, err
	}
	target, err := workspaceCanonicalDirectory(endpoint)
	if err != nil {
		return classification, err
	}
	generation := filepath.Base(filepath.Dir(target))
	store, err := workspaceCanonicalDirectory(contract.generationRoot)
	if err != nil {
		return classification, err
	}
	if filepath.Base(target) != contract.rootEndpoint || len(generation) != contract.prefixUnits || !shaPattern.MatchString(generation+strings.Repeat("0", 64-len(generation))) || !workspacePathEqual(filepath.Dir(filepath.Dir(target)), store) {
		return classification, fmt.Errorf("exact declared managed workspace generation path required")
	}
	if len(utf16.Encode([]rune(target))) > policy.MaximumPathLength {
		return classification, fmt.Errorf("managed workspace target exceeds its path budget")
	}
	var raw json.RawMessage
	if err := readWorkspaceManifest(filepath.Join(filepath.Dir(target), contract.receiptName), contract.maximumReceiptBytes, &raw); err != nil {
		return classification, err
	}
	var ready workspaceManagedReady
	decoder := json.NewDecoder(bytes.NewReader(raw))
	decoder.DisallowUnknownFields()
	if err := decoder.Decode(&ready); err != nil {
		return classification, err
	}
	if !workspaceReadyShape(ready, contract) || !strings.HasPrefix(ready.Generation, generation) {
		return classification, fmt.Errorf("bounded current managed workspace binding declaration required")
	}
	matches := 0
	for _, binding := range ready.Bindings {
		if workspacePathEqual(binding.Source, endpoint) {
			actual, err := workspaceCanonicalDirectory(binding.Target)
			if err != nil {
				return classification, err
			}
			if !workspacePathEqual(actual, target) {
				return classification, fmt.Errorf("exact managed workspace target binding required")
			}
			matches++
		}
	}
	if matches != 1 {
		return classification, fmt.Errorf("exact managed workspace Source binding required")
	}
	classification.Kind = "managed"
	return classification, nil
}
