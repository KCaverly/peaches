package utils

import (
	"os/exec"
	"regexp"
	"strings"
)

type SSHHost struct {
	Name string
	User string
}

func GetSSHHostsInConfig() ([]SSHHost, error) {

	cmd := exec.Command("cat", "/home/kcaverly/.ssh/config")
	res, err := cmd.CombinedOutput()

	if err != nil {
		return []SSHHost{}, nil
	}

	hosts := strings.Split(string(res), "Host ")

	sshHosts := []SSHHost{}

	for _, host := range hosts {
		if host != "" {

			hostName := strings.Split(host, "\n")[0]

			r, _ := regexp.Compile("User ([a-zA-Z]+)")
			hostUser := r.FindStringSubmatch(host)
			sshHosts = append(sshHosts, SSHHost{hostName, hostUser[len(hostUser)-1]})

		}
	}

	return sshHosts, nil

}
