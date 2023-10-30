package main

import (
	"bufio"
	"fmt"
	"log"
	"net/http"
	"os"
	"strings"
)

func getPortFromSetting() string {
	// 默认端口
	defaultPort := ":8000"

	// 打开设置文件
	file, err := os.Open("./server/setting.ini")
	if err != nil {
		log.Printf("Error opening setting.ini: %s", err)
		return defaultPort
	}
	defer file.Close()

	// 逐行读取文件内容
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		// 检查是否有 "port=" 前缀
		if strings.HasPrefix(line, "port=") {
			return ":" + strings.TrimSpace(line[5:])
		}
	}

	if err := scanner.Err(); err != nil {
		log.Printf("Error reading setting.ini: %s", err)
	}

	return defaultPort
}

func main() {
	// 定义文件夹路径
	const dir = "./project"

	// 检查该文件夹是否存在
	if _, err := os.Stat(dir); os.IsNotExist(err) {
		log.Fatalf("Directory %s does not exist.", dir)
	}

	// 创建文件服务器处理程序，提供指定的文件夹
	fileServer := http.FileServer(http.Dir(dir))

	// 由于我们希望文件路径与URL路径对应，所以这里不使用StripPrefix
	http.Handle("/", fileServer)

	// 从setting.ini中获取port
	port := getPortFromSetting()
	fmt.Printf("prot:%s", port)

	log.Printf("Serving %s on HTTP port: %s\n", dir, port)
	log.Fatal(http.ListenAndServe(port, nil))
}
