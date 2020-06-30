package main
import "_t_main_t_/util"
func main() {
	util.Log().Infof("echo: %v",Echo("hello world"))
}

func Echo(val string) string {
	return val
}

