package utils

import (
	"github.com/ktr0731/go-fuzzyfinder"
)

func LaunchFuzzyFinder(objects []string) (string, error) {
	idx, err := fuzzyfinder.Find(
		objects,
		func(i int) string {
			return objects[i]
		},
	)

	if err != nil {
		return "", err
	}

	return objects[idx], nil

}
