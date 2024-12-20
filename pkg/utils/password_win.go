//go:build windows
// +build windows

package utils

import (
	"fmt"
	"os"

	"golang.org/x/term"
)

func PromptPassword(provider string) (string, error) {
	fmt.Printf("Please the %s password: \n", provider)
	key, err := term.ReadPassword(int(os.Stdin.Fd()))
	if err != nil {
		return "", err
	}
	return string(key), nil
}
