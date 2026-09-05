// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Package id provides platform-entropy identifiers without external types.

// #endregion 🧲️Header

package id

import (
	"crypto/rand"
	"encoding/hex"
)

// #region 🆔️Identifier

type ID [16]byte

func New() ID {
	var value ID
	if _, err := rand.Read(value[:]); err != nil {
		panic(err)
	}
	value[6] = value[6]&0x0f | 0x40
	value[8] = value[8]&0x3f | 0x80
	return value
}

func (value ID) String() string {
	encoded := make([]byte, 36)
	hex.Encode(encoded[0:8], value[0:4])
	encoded[8] = '-'
	hex.Encode(encoded[9:13], value[4:6])
	encoded[13] = '-'
	hex.Encode(encoded[14:18], value[6:8])
	encoded[18] = '-'
	hex.Encode(encoded[19:23], value[8:10])
	encoded[23] = '-'
	hex.Encode(encoded[24:36], value[10:16])
	return string(encoded)
}

// #endregion 🆔️Identifier
