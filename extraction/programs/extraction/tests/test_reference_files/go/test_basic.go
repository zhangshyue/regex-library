import (
    "other_module",
    "regexp"
)

func main() {
    matched, err := regexp.Match(`foo.*`, []byte(`seafood`))
    fmt.Println(matched, err)
    matched, err = regexp.Match(`bar.*`, []byte(`seafood`))
    fmt.Println(matched, err)
    matched, err = regexp.Match(`a(b`, []byte(`seafood`))
    fmt.Println(matched, err)
}