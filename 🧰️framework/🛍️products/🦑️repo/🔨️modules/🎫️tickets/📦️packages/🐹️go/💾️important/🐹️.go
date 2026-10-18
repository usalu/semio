package tickets

import (
	fmt "fmt"
)

// #region 💾️Important

// 📓️JournalStep is one recorded step of an important-document transaction.
type JournalStep struct {
	Op      string `json:"op"`
	Path    string `json:"path"`
	Outcome string `json:"outcome"`
}

// 📓️TransactionJournal is the ordered record of every step a transaction attempted.
type TransactionJournal struct {
	Steps []JournalStep `json:"steps"`
}

// ➕️Record records one step.
func (journal *TransactionJournal) Record(op, path, outcome string) {
	journal.Steps = append(journal.Steps, JournalStep{Op: op, Path: path, Outcome: outcome})
}

// 🔤️Trace returns the step sequence as op and outcome pairs.
func (journal TransactionJournal) Trace() []string {
	trace := make([]string, 0, len(journal.Steps))
	for _, step := range journal.Steps {
		trace = append(trace, step.Op+":"+step.Outcome)
	}
	return trace
}

// 🈳️IsEmpty reports whether nothing was recorded.
func (journal TransactionJournal) IsEmpty() bool { return len(journal.Steps) == 0 }

// 🖼️ImportantPreimage is the exact empty-document bundle a close must find and may restore.
type ImportantPreimage struct {
	Path string `json:"path"`
	Dir  string `json:"dir"`
	Mode uint32 `json:"mode"`
}

// ✨️ImportantCreation is what a reopen created, so a failed save removes exactly that.
type ImportantCreation struct {
	Path        string `json:"path"`
	Dir         string `json:"dir"`
	FileCreated bool   `json:"fileCreated"`
	DirCreated  bool   `json:"dirCreated"`
}

// 🔎️InspectImportantDocument requires the canonical important document to be an exact bundle.
func InspectImportantDocument(store TicketStore, path string, journal *TransactionJournal) (ImportantPreimage, error) {
	dir := DirName(path)
	metadata, found := store.Metadata(path)
	if !found {
		journal.Record("inspect", path, "failed")
		return ImportantPreimage{}, errTicketNotFound(fmt.Sprintf("cannot finish ticket: required important document %s is unavailable", path))
	}
	if metadata.Kind != NodeFile {
		journal.Record("inspect", path, "failed")
		return ImportantPreimage{}, errInvalid(fmt.Sprintf("cannot finish ticket: important document %s is not a regular file", path))
	}
	content, err := store.Read(path)
	if err != nil {
		return ImportantPreimage{}, err
	}
	if content != "" {
		journal.Record("inspect", path, "failed")
		return ImportantPreimage{}, errInvalid(fmt.Sprintf("cannot finish ticket: important document %s is not empty", path))
	}
	entries, err := store.Entries(dir)
	if err != nil {
		return ImportantPreimage{}, err
	}
	expected := BaseName(path)
	if len(entries) != 1 || entries[0].Name != expected {
		journal.Record("inspect", dir, "failed")
		return ImportantPreimage{}, errInvalid(fmt.Sprintf("cannot finish ticket: important directory %s must contain only %s", dir, expected))
	}
	journal.Record("inspect", path, "ok")
	return ImportantPreimage{Path: path, Dir: dir, Mode: metadata.Mode}, nil
}

// 🔄️RestoreImportantDocument restores an empty document without overwriting an existing node.
func RestoreImportantDocument(store TicketStore, preimage ImportantPreimage, journal *TransactionJournal) error {
	if err := store.CreateDirectory(preimage.Dir); err != nil {
		return err
	}
	if err := store.CreateExclusive(preimage.Path, preimage.Mode&0o777); err != nil {
		return err
	}
	journal.Record("restore", preimage.Path, "ok")
	return nil
}

// 🗑️RemoveImportantDocument removes the exact bundle, restoring it when the directory will not go.
func RemoveImportantDocument(store TicketStore, preimage ImportantPreimage, journal *TransactionJournal) error {
	if err := store.RemoveFile(preimage.Path); err != nil {
		return err
	}
	journal.Record("remove-file", preimage.Path, "ok")
	if err := store.RemoveDirectory(preimage.Dir); err != nil {
		journal.Record("remove-dir", preimage.Dir, "failed")
		if restore := RestoreImportantDocument(store, preimage, journal); restore != nil {
			return errStore(fmt.Sprintf("remove important directory: %v; restore important document: %v", err, restore))
		}
		return errStore(fmt.Sprintf("remove important directory: %v", err))
	}
	journal.Record("remove-dir", preimage.Dir, "ok")
	return nil
}

// ✨️EnsureImportantDocument preserves an existing regular document or creates an empty one.
func EnsureImportantDocument(store TicketStore, path string, journal *TransactionJournal) (ImportantCreation, error) {
	dir := DirName(path)
	creation := ImportantCreation{Path: path, Dir: dir}
	if metadata, found := store.Metadata(path); found {
		if metadata.Kind != NodeFile {
			journal.Record("ensure", path, "failed")
			return creation, errInvalid(fmt.Sprintf("important document %s is not a regular file", path))
		}
		if _, err := store.Read(path); err != nil {
			return creation, err
		}
		journal.Record("ensure", path, "preserved")
		return creation, nil
	}
	metadata, found := store.Metadata(dir)
	switch {
	case found && metadata.Kind == NodeDirectory:
	case found:
		journal.Record("ensure-dir", dir, "failed")
		return creation, errInvalid(fmt.Sprintf("important directory %s is not a physical directory", dir))
	default:
		if err := store.CreateDirectory(dir); err != nil {
			return creation, err
		}
		creation.DirCreated = true
		journal.Record("ensure-dir", dir, "created")
	}
	if err := store.CreateExclusive(path, 0o644); err != nil {
		if creation.DirCreated {
			_ = store.RemoveDirectory(dir)
		}
		journal.Record("ensure", path, "failed")
		return creation, err
	}
	creation.FileCreated = true
	journal.Record("ensure", path, "created")
	return creation, nil
}

// ↩️RollbackImportantCreation removes only the nodes the current reopen created.
func RollbackImportantCreation(store TicketStore, creation ImportantCreation, journal *TransactionJournal) error {
	if creation.FileCreated && StoreExists(store, creation.Path) {
		if err := store.RemoveFile(creation.Path); err != nil {
			return err
		}
		journal.Record("rollback-file", creation.Path, "ok")
	}
	if creation.DirCreated && StoreExists(store, creation.Dir) {
		if err := store.RemoveDirectory(creation.Dir); err != nil {
			return err
		}
		journal.Record("rollback-dir", creation.Dir, "ok")
	}
	return nil
}

// #endregion 💾️Important
