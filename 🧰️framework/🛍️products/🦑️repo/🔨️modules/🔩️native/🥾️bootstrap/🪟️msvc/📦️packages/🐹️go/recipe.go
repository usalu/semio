package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"path/filepath"
	"regexp"
	"strings"
)

const recipeDigest = "c9e7064fc494abbff8cc2ec4074fd791f2365e16005f68a78484c56f3b2f40b5"
const inventoryDigest = "69cfa0b1437d32de989abad528e77ef5497bd0771e2d5b281d6385786520b6db"
const catalogDigest = "a8e854fa9f3fbaeac27d7304e297f1660d7137401389deb57cf7983f33c16fef"

type recipePayload struct {
	ID            string `json:"id"`
	Name          string `json:"name"`
	DeclaredBytes int64  `json:"declaredBytes"`
	Bytes         int64  `json:"bytes"`
	SHA256        string `json:"sha256"`
}
type recipeDocument struct {
	Version                              int `json:"version"`
	Algorithm, CatalogURL, CatalogSHA256 string
	Payloads                             []recipePayload `json:"payloads"`
	TableHashes                          []string        `json:"tableHashes"`
}
type catalogPayload struct {
	FileName string `json:"fileName"`
	SHA256   string `json:"sha256"`
	Size     int64  `json:"size"`
	URL      string `json:"url"`
}
type catalogPackage struct {
	ID, Language string
	Payloads     []catalogPayload `json:"payloads"`
}
type catalogDocument struct {
	Packages []catalogPackage `json:"packages"`
}
type inputPlan struct {
	archives, msis []archiveInput
	cabinets       []cabinetInput
	tableHashes    []string
	urls           []string
}

var lowerHash = regexp.MustCompile(`^[a-f0-9]{64}$`)

func sha(data []byte) string { sum := sha256.Sum256(data); return hex.EncodeToString(sum[:]) }
func makeInputPlan(recipeBytes, catalogBytes []byte, inputs string) (inputPlan, error) {
	var result inputPlan
	if sha(recipeBytes) != recipeDigest || sha(catalogBytes) != catalogDigest {
		return result, errors.New("untrusted MSVC recipe or catalog")
	}
	var recipe recipeDocument
	var catalog catalogDocument
	if err := json.Unmarshal(recipeBytes, &recipe); err != nil {
		return result, err
	}
	if err := json.Unmarshal(catalogBytes, &catalog); err != nil {
		return result, err
	}
	if recipe.Version != 1 || recipe.Algorithm != "semio-msvc-1" || recipe.CatalogSHA256 != catalogDigest || len(recipe.Payloads) != 47 || len(recipe.TableHashes) != 8 {
		return result, errors.New("invalid MSVC recipe")
	}
	sdk := []catalogPackage{}
	for _, p := range catalog.Packages {
		if p.ID == "Win11SDK_10.0.26100" {
			sdk = append(sdk, p)
		}
	}
	if len(sdk) != 1 {
		return result, errors.New("ambiguous SDK package")
	}
	for index, row := range recipe.Payloads {
		if !lowerHash.MatchString(row.SHA256) || row.Bytes <= 0 || row.Bytes > 60000000 {
			return result, errors.New("invalid payload pin")
		}
		matches := []catalogPayload{}
		if index < 5 {
			for _, p := range catalog.Packages {
				if p.ID == row.ID && (p.Language == "" || p.Language == "en-US") {
					matches = append(matches, p.Payloads...)
				}
			}
		} else {
			for _, p := range sdk[0].Payloads {
				if p.FileName == row.Name {
					matches = append(matches, p)
				}
			}
		}
		if len(matches) != 1 || matches[0].FileName != row.Name || !strings.EqualFold(matches[0].SHA256, row.SHA256) || matches[0].Size != row.DeclaredBytes {
			return result, fmt.Errorf("catalog payload mismatch: %s", row.ID)
		}
		result.urls = append(result.urls, matches[0].URL)
		input := archiveInput{SHA256: row.SHA256, Bytes: row.Bytes}
		switch {
		case index < 5:
			input.Path = filepath.Join(inputs, fmt.Sprintf("msvc-%d.vsix", index))
			result.archives = append(result.archives, input)
		case index < 13:
			if row.ID != fmt.Sprintf("msi-%d", index-5) {
				return result, errors.New("MSI order mismatch")
			}
			input.Path = filepath.Join(inputs, fmt.Sprintf("sdk-%d.msi", index-5))
			result.msis = append(result.msis, input)
		default:
			if !cabinetID.MatchString(row.ID) {
				return result, errors.New("invalid cabinet pin")
			}
			input.Path = filepath.Join(inputs, row.ID)
			result.cabinets = append(result.cabinets, cabinetInput{row.ID, input})
		}
	}
	for _, hash := range recipe.TableHashes {
		if !lowerHash.MatchString(hash) {
			return result, errors.New("invalid MSI table pin")
		}
	}
	result.tableHashes = recipe.TableHashes
	return result, nil
}

func readBounded(path string, limit int64) ([]byte, error) {
	file, err := openLocked(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	info, err := file.Stat()
	if err != nil || info.Size() < 1 || info.Size() > limit {
		return nil, errors.New("bounded metadata required")
	}
	data := make([]byte, info.Size())
	if _, err := file.ReadAt(data, 0); err != nil {
		return nil, err
	}
	return data, nil
}
