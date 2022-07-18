/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (

	"github.com/KCaverly/peaches/utils"
	"github.com/spf13/cobra"
)

// sshCmd represents the ssh command
var sshCmd = &cobra.Command{
	Use:   "ssh",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: func(cmd *cobra.Command, args []string) {
		sshHosts, _ := utils.GetSSHHostsInConfig()

		hostNames := []string{}
		for _, host := range sshHosts {
			hostNames = append(hostNames, host.Name)
		}

		selected, err := utils.LaunchFuzzyFinder(hostNames)

		if err == nil {

			// Get Username
			var user string
			for _, host := range sshHosts {
				if host.Name == selected {
					user = host.User
				}
			}

			utils.TmuxCreateWindow("kc", selected)

			utils.TmuxSendKeys("kc", selected, "ssh "+user+"@"+selected)
			utils.TmuxAttachOrSelectWindow("kc", selected)

		}
	},
}

func init() {
	rootCmd.AddCommand(sshCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// sshCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// sshCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
