
### 使用 Rust 开发一个短链接服务


```
web 框架：actix-web 
配置管理 - 环境隔离 - config
数据库访问 - sqlx - mysql
前端页面加载 trea 

```


```

Tiny URL API
1. 根据 URL 生成短链接 code
2. 根据生成短链接 Code 跳转原 URL
3. 查询全部生成的短链接地址

```


### 测试
```
1. curl --request GET 'localhost:8080/links'
2. curl --request POST '127.0.0.1:8080/create' --header 'Content-Type: application/json' --data-raw '{"origin_url":"http://baidu.com"}' 
3. 浏览器请求短链接，重定位
```



### 统计
```
$ cloc ./
      12 text files.
      12 unique files.                              
       3 files ignored.

github.com/AlDanial/cloc v 1.90  T=0.01 s (1339.4 files/s, 47282.0 lines/s)
-------------------------------------------------------------------------------
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                             4             64              1            202
TOML                             3              7              2             29
Markdown                         1             10              0             19
HTML                             1              2              0             10
SQL                              1              0              0              7
-------------------------------------------------------------------------------
SUM:                            10             83              3            267
-------------------------------------------------------------------------------

```