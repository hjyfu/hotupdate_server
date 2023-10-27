package main

import (
	"log"
	"net/http"
	"os"
)

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

	// 启动HTTP服务器
	port := ":8000"
	log.Printf("Serving %s on HTTP port: %s\n", dir, port)
	log.Fatal(http.ListenAndServe(port, nil))
}
