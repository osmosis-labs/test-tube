package result

import (
	"C"
	"encoding/base64"
)
import "fmt"

var (
	Ok           byte = 0
	QueryError   byte = 1
	ExecuteError byte = 2
)

func markError(code byte, data []byte) []byte {
	return append([]byte{code}, data...)
}

func markOk(data []byte) []byte {
	return append([]byte{Ok}, data...)
}

func EncodeResultFromError(code byte, err error) string {
	fmt.Println("EncodeResultFromError")
	marked := markError(code, []byte(err.Error()))
	return base64.StdEncoding.EncodeToString(marked)
}

func EncodeResultFromOk(data []byte) string {
	fmt.Println("EncodeResultFromOk")

	marked := markOk(data)
	return base64.StdEncoding.EncodeToString(marked)
}
