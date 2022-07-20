/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"github.com/KCaverly/peaches/utils"
	"github.com/spf13/cobra"
	"strings"
)

type Directory struct {
	path     string
	maxDepth int
}

func InitializeProjects() []string {

	dirs := []Directory{
		{"/home/kcaverly/work", 2},
		{"/home/kcaverly/personal", 1},
		{"/home/kcaverly/personal/courses", 1},
		{"/home/kcaverly/.dotfiles", 3},
	}

	var folders []string
	for _, dir := range dirs {
		excludeList := []string{`.git\.*`, `__pycache__\.*`, `tmp\.*`}
		new_folders, _ := utils.GetFolders(dir.path, excludeList, dir.maxDepth)
		folders = append(folders, new_folders...)
	}

	return folders
}

// projectsCmd represents the projects command
var projectsCmd = &cobra.Command{
	Use:   "projects",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		folders := InitializeProjects()
		selected, err := utils.LaunchFuzzyFinder(folders)
		if err == nil {
			parts := strings.Split(selected, "/")
			selectedName := parts[len(parts)-1]

			utils.TmuxCreateWindow("kc", selectedName)

			utils.TmuxSendKeys("kc", selectedName, "cd "+selected)
			utils.TmuxSendKeys("kc", selectedName, "clear")

			utils.TmuxAttachOrSelectWindow("kc", selectedName)
		}

	},
}

func init() {
	rootCmd.AddCommand(projectsCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// projectsCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// projectsCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
