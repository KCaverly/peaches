package utils

import (
	"fmt"
	"github.com/ktr0731/go-fuzzyfinder"
)

func LaunchFuzzyFinder(objects []string) string {
	idx, err := fuzzyfinder.Find(
		objects,
		func(i int) string {
			return objects[i]
		},
	)

	if err != nil {
		fmt.Println(err)
	}

	return objects[idx]

}
