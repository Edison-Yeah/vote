package main

import (
	"fmt"
	"sync"
)


type MemDB struct {
	mtx sync.Mutex
	db  map[string][]byte
}

func NewMemDB() *MemDB {
	database := &MemDB{
		db: make(map[string][]byte),
	}
	return database
}


// Implements DB.
func (db *MemDB) Get(key []byte) ([]byte, error) {
	db.mtx.Lock()
	defer db.mtx.Unlock()
	key = nonNilBytes(key)

	value := db.db[string(key)]
	return value, nil
}

// Implements DB.
func (db *MemDB) Set(key []byte, value []byte) {
	db.mtx.Lock()
	defer db.mtx.Unlock()

	db.SetNoLock(key, value)
}

// Implements atomicSetDeleter.
func (db *MemDB) SetNoLock(key []byte, value []byte) {
	db.SetNoLockSync(key, value)
}

// Implements atomicSetDeleter.
func (db *MemDB) SetNoLockSync(key []byte, value []byte) {
	key = nonNilBytes(key)
	value = nonNilBytes(value)

	fmt.Println("final write value:")
	fmt.Println(value)

	db.db[string(key)] = value
}

// Implements DB.
func (db *MemDB) Delete(key []byte) {
	db.mtx.Lock()
	defer db.mtx.Unlock()

	db.DeleteNoLock(key)
}

// Implements atomicSetDeleter.
func (db *MemDB) DeleteNoLock(key []byte) {
	db.DeleteNoLockSync(key)
}

// Implements atomicSetDeleter.
func (db *MemDB) DeleteNoLockSync(key []byte) {
	key = nonNilBytes(key)

	delete(db.db, string(key))
}

func nonNilBytes(bz []byte) []byte {
	if bz == nil {
		return []byte{}
	}
	return bz
}