package utils

import (
	"fmt"
	"github.com/BurntSushi/toml"
	"log"
	"os"
)

type TmuxConfig struct {
	Session string
}

type ProjectsConfig struct {
	Paths    []string
	MaxDepth []int
	Exclude  []string
}

type SSHConfig struct {
	Hosts string
}

type tomlConfig struct {
	Title    string
	Tmux     TmuxConfig     `toml:"tmux"`
	Projects ProjectsConfig `toml:"projects"`
	SSH      SSHConfig      `toml:"ssh"`
}

type Config struct {
	Path   string
	Data   string
	Config tomlConfig
}

func LoadConfig() *Config {
	cfg := &Config{Path: "/home/kcaverly/personal/peaches/config.toml"}
	cfg.Load()
	return cfg
}

func (c *Config) GetPath() string {
	return c.Path
}

func (c *Config) Load() {

	data, _ := os.ReadFile(c.Path)
	c.Data = string(data)

	var conf tomlConfig
	if _, err := toml.Decode(c.Data, &conf); err != nil {
		log.Fatal(err)
	}

	c.Config = conf
}

func (c *Config) Show() {
	fmt.Println(c.Data)
}
