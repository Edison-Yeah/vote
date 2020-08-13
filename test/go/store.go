package main

import (
	"fmt"
	wasm "github.com/wasmerio/go-ext-wasm/wasmer"
	"unsafe"
)


//var store = map[string]string{}

/*func readDB(context unsafe.Pointer, keyPtr, keySize, valuePtr, valueSize, offset int32) int32 {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	realKey := memory[keyPtr: keyPtr + keySize]

	fmt.Printf("read key [%s]\n", string(realKey))

	var size int
	realValue, exist := store[string(realKey)]
	if !exist {
		return -1 // 如果不存在，返回值小于0
	}
	size = len(realValue)
	fmt.Printf("read value [%s]\n", string(realValue))

	if offset >= int32(size) {
		return 0
	}

	index := offset + valueSize
	if index > int32(size) {
		index = int32(size)
	}

	copiedData := []byte(realValue)[offset: index]
	copy(memory[valuePtr:valuePtr+valueSize], copiedData)

	return int32(size)
}

func writeDB(context unsafe.Pointer, keyPtr, keySize, valuePtr, valueSize int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	realKey := memory[keyPtr: keyPtr + keySize]
	realValue := memory[valuePtr: valuePtr + valueSize]

	fmt.Printf("write key [%s], value [%s]\n", string(realKey), string(realValue))

	store[string(realKey)] = string(realValue)
}*/

var store Store
func init() {
	db := NewMemDB()
	store = NewStore(db, []byte("test"))
}

func readDB(context unsafe.Pointer, keyPtr, keySize, valuePtr, valueSize, offset int32) int32 {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	realKey := memory[keyPtr: keyPtr + keySize]

	fmt.Printf("read key [%s]\n", string(realKey))

	var size int
	realValue, exist := store.Get(realKey)
	if !exist {
		fmt.Println("no")
		return -1 // 如果不存在，返回值小于0
	}
	size = len(realValue)
	//fmt.Printf("read value [%s]\n", string(realValue))
	fmt.Println(realValue)

	if offset >= int32(size) {
		return 0
	}

	index := offset + valueSize
	if index > int32(size) {
		index = int32(size)
	}

	copiedData := (realValue)[offset: index]
	copy(memory[valuePtr:valuePtr+valueSize], copiedData)

	return int32(size)
}

func writeDB(context unsafe.Pointer, keyPtr, keySize, valuePtr, valueSize int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	realKey := memory[keyPtr: keyPtr + keySize]
	realValue := memory[valuePtr: valuePtr + valueSize]

	fmt.Printf("write key [%s], value [%s]\n", string(realKey), string(realValue))
	fmt.Println(realValue)

	//store[string(realKey)] = string(realValue)
	store.Set(realKey, realValue)
}

func deleteDB(context unsafe.Pointer, keyPtr, keySize int32) {
	var instanceContext = wasm.IntoInstanceContext(context)
	var memory = instanceContext.Memory().Data()

	realKey := memory[keyPtr: keyPtr + keySize]

	fmt.Printf("delete key [%s]\n", string(realKey))

	//delete(store, string(realKey))
	store.Delete(realKey)
}