package utils

import (
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

type Folder struct {
	path string
	name string
}

func DirExists(directory string) (bool, error) {
	dirInfo, err := os.Stat(directory)
	if os.IsNotExist(err) {
		return false, nil
	}

	return dirInfo.IsDir(), nil
}

func GetFolders(directory string, excludeList []string, maxDepth int) (dirList []string, err error) {

	// Check if directory exists
	dirExists, err := DirExists(directory)
	if !dirExists {
		return dirList, nil
	}
	err = filepath.Walk(directory, func(path string, info os.FileInfo, err error) error {
		if regexp.MustCompile(strings.Join(excludeList, "|")).Match([]byte(path)) {
			return nil
		}

		if info.IsDir() {
			relativePath := strings.Replace(path, directory, "", 1)
			if strings.Count(relativePath, "/") <= maxDepth {
				dirList = append(dirList, path)
			}
		}
		return nil
	})
	return dirList, nil
}
