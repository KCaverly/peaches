package utils

import (
	"fmt"
	"os"
	"os/exec"
	"strings"
	"syscall"
)

func InTmux() bool {
	_, isSet := os.LookupEnv("TMUX")
	return isSet
}

func TmuxActive() (bool, error) {

	if InTmux() {
		return true, nil
	}

	cmd := exec.Command("tmux", "info", "-t", "/dev/pts/1")
	res, _ := cmd.CombinedOutput()

	if strings.Contains(string(res), "no server") {
		return false, nil
	} else {
		return true, nil
	}

	return false, nil
}

func TmuxSessionExists(sessionName string) (bool, error) {

	// If Tmux is Not Active, Then the Session Can't Exist
	active, err := TmuxActive()
	if err != nil {
		return false, err
	}

	if active {

		cmd := exec.Command("tmux", "has-session", "-t", sessionName)
		res, err := cmd.CombinedOutput()

		if err != nil {
			return false, nil
		}

		if strings.Contains(string(res), "can't find session") {
			return false, nil
		} else {
			return true, nil
		}

	}

	return false, nil
}

func TmuxWindowExists(sessionName string, windowName string) (bool, error) {

	// In Order for The Window to Exist, the Session Must Aswell
	sessionExists, err := TmuxSessionExists(sessionName)
	if err != nil {
		return false, nil
	}

	if sessionExists {

		cmd := exec.Command("tmux", "has-session", "-t", sessionName+":"+windowName)
		res, _ := cmd.CombinedOutput()
		if err != nil {
			return false, nil
		}

		if strings.Contains(string(res), "can't find window") {
			return false, nil
		} else {
			return true, nil
		}

	}
	return false, nil
}

func TmuxCreateSession(sessionName string) (bool, error) {

	// Do not recreate the session if one exists
	sessionExists, err := TmuxSessionExists(sessionName)
	if err != nil {

		return false, err
	}

	if sessionExists {
		return true, nil

	} else {
		// if session does not exist, create Session
		cmd := exec.Command("tmux", "new-session", "-d", "-s", sessionName)
		_, err := cmd.CombinedOutput()

		if err != nil {
			return false, err
		}
	}
	return true, nil
}

func TmuxCreateWindow(sessionName string, windowName string) (bool, error) {

	// If Window Exists, do not create window
	windowExists, err := TmuxWindowExists(sessionName, windowName)
	if err != nil {
		return false, err
	}

	// If Session Exists, do not recreate Session
	sessionExists, err := TmuxSessionExists(sessionName)
	if err != nil {
		return false, err
	}

	if windowExists {
		return true, nil
	} else if sessionExists {
		cmd := exec.Command("tmux", "new-window", "-t", sessionName, "-n", windowName)
		_, err := cmd.CombinedOutput()
		if err != nil {
			return false, err
		}

	} else {
		cmd := exec.Command("tmux", "new-session", "-d", "-s", sessionName, "-n", windowName)
		_, err := cmd.CombinedOutput()

		if err != nil {
			return false, err
		}

		return true, nil
	}

	return false, nil

}

func TmuxAttachOrSelectWindow(sessionName string, windowName string) (bool, error) {

	windowExists, err := TmuxCreateWindow(sessionName, windowName)

	if err != nil {
		return false, err
	}

	if windowExists && InTmux() {
		tmux, _ := exec.LookPath("tmux")
		args := []string{"tmux", "switch", "-t", sessionName, ":", windowName}
		err := syscall.Exec(tmux, args, os.Environ())
		if err != nil {
			return false, err
		}

		return true, nil
	} else if windowExists && !InTmux() {
		tmux, _ := exec.LookPath("tmux")
		args := []string{"tmux", "attach", "-t", sessionName}
		err := syscall.Exec(tmux, args, os.Environ())
		if err != nil {
			return false, err
		}
		return true, nil
	}

	return false, nil
}

func TmuxSendKeys(sessionName, windowName string, sendKeys string) (bool, error) {

	cmd := exec.Command("tmux", "send-keys", "-t", sessionName+":"+windowName, sendKeys, "C-m")
	_, err := cmd.CombinedOutput()

	if err != nil {
		return false, err
	} else {
		return true, nil
	}

	return false, nil

}

func TmuxListWindows() (bool, error) {

	cmd := exec.Command("tmux", "list-windows")
	output, err := cmd.CombinedOutput()

	fmt.Println(string(output))

	windows := string.split(output, "\n")

  for window := range(windows) {

  }

	if err != nil {
		return false, err
	} else {
		return true, nil
	}

	return false, nil

}

func TmuxSplitWindow(sessionName string, windowName string) (bool, error) {

	cmd := exec.Command("tmux", "split-window")
	_, err := cmd.CombinedOutput()

	if err != nil {
		return false, err
	} else {
		return true, nil
	}

	return false, nil

}
