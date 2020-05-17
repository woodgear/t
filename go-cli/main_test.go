package main

import (
	"testing"
)

func TestPing(t *testing.T) {
	t.Logf("Ping ")
	if Echo("xxx") != "xxx" {
		t.Error("fail")
	}
}

func TestPing2(t *testing.T) {
	t.Logf("Ping ")
	if Echo("111") == "xxx" {
		t.Error("fail")
	}
}
