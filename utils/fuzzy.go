package utils

import (
	"github.com/ktr0731/go-fuzzyfinder"
)

func LaunchFuzzyFinder(objects []string) string {
	idx, _ := fuzzyfinder.Find(
		objects,
		func(i int) string {
			return objects[i]
		},
	)

	return objects[idx]

}
