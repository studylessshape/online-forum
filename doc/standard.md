# 标准/规则/规范条目
## 前端
前端使用 `vue` 框架开发，`vite` 构建，构建的项目可使用 nginx 负载运行。

## 后端
后端使用 `rust` 开发，使用的框架主要为 `actix-web`，构建工具为 `cargo`。通过交叉编译或本地编译，直接运行项目即可。

## 前后端交互
### 前端发送请求给后端的路径
1. 后端解析的主要为路径，匹配路径的内容和类型；
2. 前端请求数据必须使用方法 `GET`，发送数据必须使用 `POST`；
3. 请求路径格式为：
```
protocol//{server-host}/{object-name}/{action}({'/' or '?'})[{detail-info}]

// 示例，用户 id 为 114514 的用户详细数据获取请求，请求方法为 GET
http://localhost:8080/user/user-detail-profiles?user_id=114514

// 示例，用户注册，请求方法为 POST
http://localhost:8080/user/user-sign-up
```
4. 

### 前端发送数据
1. 前端发送的数据，不应以 `data` 将所有数据包裹，而应指定正确的数据对象名称，或者匿名，如用户注册时，将邮箱发送给后端的数据：
```json
{
    "email":"email@example.com"
}
```
2. 由于后端序列化数据非常严格，所以发送给后端的数据需要和后端定义的结构一致（或后端定义的数据结构要和前端发送的数据结构一致）。如下数据结构示例（上部分为后端数据结构，下部分为前端数据结构）：
```rust
// in backend, rust
struct UserSignStatus {
    user_name: String,
    user_nickname: String,
    avatar: Option<String>,
    token: Uuid,
}
```

```js
// in frontend, vue
userSignStatus {
    user_name: "",
    user_nickname: "",
    avatar: "", // null or "" is none on rust
    token: ""
}
```
3. 

### 后端发送数据
1. 后端发送给前端的数据，如果只有一个字符串（包括错误信息），统一使用 `json` 格式封装，并且指定字段名为 `responseText`，如下示例：
```json
{
    "responseText":"This is the message from backend",
}
```
2. 当后端涉及的 `json` 数据格式过多时，应当将每个功能分到不同的模块文件中；
3. 发送的响应代码，如无必要，应使用 `HttpResponse::Ok()`（200），来确保回应的数据传达到前端；