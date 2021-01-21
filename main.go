package main

/*
#cgo LDFLAGS: -L./lib -lhello
#include "./lib/hello.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	nodeUri := C.CString("https://api.hornet01.alphanet.iota.cafe")
	defer C.free(unsafe.Pointer(nodeUri))

	client := C.client_new(nodeUri)
	res := C.client_get_tips(client)
	fmt.Println(C.GoString(res.tip_1), C.GoString(res.tip_2), res.error)
}
