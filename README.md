### 使用 Rust 开发一个短链接服务


```
web 框架：actix-web 

Tiny URL API
1. 根据 URL 生成短链接 code
2. 根据生成短链接 Code 跳转原 URL
3. 查询全部生成的短链接地址



配置管理 - 环境隔离
数据库访问 - sqlx - mysql

前端页面加载 trea 


curl --request POST '127.0.0.1:8080/create' --header 'Content-Type: application/json' --data-raw '{"origin_url":"http://baidu.com"}' 
```



