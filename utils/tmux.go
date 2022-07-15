package utils

import (
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

	cmd := exec.Command("tmux", "info", "2>&1")
	res, err := cmd.CombinedOutput()
	if err != nil {
		return false, nil
	}

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
	session_exists, err := TmuxSessionExists(sessionName)
	if err != nil {
		return false, nil
	}

	if session_exists {

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
	session_exists, err := TmuxSessionExists(sessionName)
	if err != nil {

		return false, err
	}

	if session_exists {
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
	window_exists, err := TmuxWindowExists(sessionName, windowName)
	if err != nil {
		return false, err
	}

	// If Session Exists, do not recreate Session
	session_exists, err := TmuxSessionExists(sessionName)
	if err != nil {
		return false, err
	}

	if window_exists {
		return true, nil
	} else if session_exists {
		cmd := exec.Command("tmux", "new-window", "-t", sessionName, "-n", windowName)
		_, err := cmd.Output()
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

	window_exists, err := TmuxCreateWindow(sessionName, windowName)

	if err != nil {
		return false, err
	}

	if window_exists && InTmux() {
		tmux, _ := exec.LookPath("tmux")
		args := []string{"tmux", "switch", "-t", sessionName, ":", windowName}
		err := syscall.Exec(tmux, args, os.Environ())
		if err != nil {
			return false, err
		}

		return true, nil
	} else if window_exists && !InTmux() {
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
