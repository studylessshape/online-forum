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
如果运行时没用配置文件，则会自动创建一个配置文件，并且会提示配置好邮件发送服务的账户密码和服务器，如下所示。
```
[2023-05-08T08:05:15Z INFO  backend] Read config file......
[2023-05-08T08:05:15Z WARN  backend] Please config your email and password in file `server.conf.toml`
```

所以如果不清楚配置文件内容，可以尝试运行一次，再对配置文件进行配置。

配置文件解析：

```toml
# 后端启动后，监听的端口
port = 8082
# 后端跨域请求中，允许给后端发送请求的服务器地址，默认为 `[]`，即允许所有外域。
# 建议添加指定的外域地址，否则将无法接收来自请求的 `Credentials` 信息
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

**注意：**
1. `allow_origins` 处建议添加指定的外域地址，否则将无法接收来自请求的 `Credentials` 信息；
2. 添加的外域地址，尽量不要在尾部加上 `/`，否则可能会导致拒绝外域服务器的请求。

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

### 前端配置
前端的配置文件为 `config.js`，在开发环境中，位于 `public` 文件夹下。打包后，应和 `index.html` 位于同一文件夹。配置的内容应和后端服务器部署的地址一致。

配置文件内容即参考配置：
```js
window.server = {
    // 服务器使用的协议
    protocol: "http",
    // 服务器地址或域名
    host: "192.168.187.24",
    // 服务器端口，可以为 null
    port: 8082,
};
```