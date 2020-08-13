package main


type KVStore interface {

	Set(key, value []byte)

	Get(key []byte) ([]byte, error)

	Delete(key []byte)
}


type Store struct {
	parent KVStore
	prefix []byte
}

func NewStore(parent KVStore, prefix []byte) Store {
	return Store{
		parent: parent,
		prefix: prefix,
	}
}

// Implements KVStore
func (s Store) Set(key, value []byte) {
	AssertValidKey(key)
	AssertValidValue(value)
	s.parent.Set(s.key(key), value)
}

// Implements KVStore
func (s Store) Get(key []byte) ([]byte, bool) {
	res, err := s.parent.Get(s.key(key))
	if err != nil {
		return nil, false
	}
	return res, true
}

// Implements KVStore
func (s Store) Delete(key []byte) {
	s.parent.Delete(s.key(key))
}

func (s Store) key(key []byte) (res []byte) {
	if key == nil {
		panic("nil key on Store")
	}
	res = cloneAppend(s.prefix, key)
	return
}


func cloneAppend(bz []byte, tail []byte) (res []byte) {
	res = make([]byte, len(bz)+len(tail))
	copy(res, bz)
	copy(res[len(bz):], tail)
	return
}


// Check if the key is valid(key is not nil)
func AssertValidKey(key []byte) {
	if key == nil {
		panic("key is nil")
	}
}

// Check if the value is valid(value is not nil)
func AssertValidValue(value []byte) {
	if value == nil {
		panic("value is nil")
	}
}