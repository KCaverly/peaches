package utils

import (
	"os"
	"path/filepath"
	"strings"
)

type Folder struct {
	path string
	name string
}

func GetFolders(directory string, maxDepth int) ([]string, error) {

	var folders []string
	err := filepath.Walk(directory, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			relative_path := strings.Replace(path, directory, "", 1)
			if strings.Count(relative_path, "/") <= maxDepth {
				folders = append(folders, path)
			}
		}
		return nil
	})
	return folders, err

}
