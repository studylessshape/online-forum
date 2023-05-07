# 使用 Rust 和 Vue 实现的论坛网站
## 禁止商用！禁止拿来盈利！

## 前端
- [vue](https://cn.vuejs.org/)：没什么好说的，强大的前端框架
- [Bootstrap](http://getbootstrap.com/)：很好用的前端布局框架
- [element-plus](https://element-plus.gitee.io/zh-CN/)：vue的ui框架，非常好用
- [md-editor-v3](https://imzbf.github.io/md-editor-v3/)：基于 vue3 的 markdown 编辑器

## 后端
- [rust](https://www.rust-lang.org/zh-CN/)：主要使用了 [actix-web](https://actix.rs/) 框架，其他用到的都在 [cargo.toml](./backend/Cargo.toml) 里

## 前后端交互
使用跨域通信

## 配置文件
### 后端
配置文件解析：

```toml
# 后端启动后，监听的端口
port = 8082
# 后端跨域请求中，允许给后端发送请求的服务器地址，默认为 `[]`，即允许所有外域。
# 此处可不填，也可以填自己前端的服务器和端口
allow_origins = []
# 跨域请求允许的方法，设置为 `[]` 为允许所有请求方法
allow_methods = ["GET", "POST"]

[email_account]
# 用户注册、更改密码、忘记密码时，会用一个邮箱来给用户发送验证码，此处配置的是能够通过 SMTP 服务器发送邮箱的账户和密码
# 如本人用的是微软的 outlook 邮箱来发送验证码邮件，则 smtp_server 处应为
email = ""
password = ""
smtp_server = "smtp.office365.com"
```

参考配置：
```toml
port = 8082
allow_origins = ["http://localhost:5173"]
allow_methods = ["GET", "POST"]

[email_account]
email = "*@outlook.com"
password = "*"
smtp_server = "smtp.office365.com"
```