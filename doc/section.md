# 论坛板块设计
## 数据库表
### 论坛板块表
|名称|类型|描述|
|-|-|-|
|`section_id`|`int`|板块在数据库中唯一的 ID|
|`section_name`|`varchar(100)`|板块名称，英文|
|`section_name_zh`|`varchar(100)`|板块名称，中文|

### 帖子表
|名称|类型|描述|
|-|-|-|
|`post_id`|`int`|帖子在数据库中唯一的 ID|
|`user_id`|`int unsigned`|发帖的用户 ID|
|`section_id`|`int`|发帖的对应板块|
|`post_titlte`|`varchar(100)`|标题|
|`post_time`|`datetime`|发帖时间|
|`post_update_time`|`datetime`|帖子更新时间|
|`post_content`|`text`|帖子内容|

### 评论表
|名称|类型|描述|
|-|-|-|
|`post_id`|`int`|帖子在数据库中唯一的 ID|
|`user_id`|`int unsigned`|发布评论的用户 ID|
|`comment_time`|`datetime`|发布评论的时间|
|`comment_content`|`text`|评论的内容|

## 行为表
### 论坛板块行为表
|行为|路径|方法|前端发送数据|后台返回数据|数据项说明|
|---|---|---|---|---|---|
|获取论坛板块状态|`/section/get-status`|`POST`|`{"sections":[]}`|`{"<section-name>":{"last_update_time":String, "today_post_count":Number, "total_post_count":Number},...}`|前端发送的数据为需要获取状态的板块的英文名，后台返回的数据为数组，对应每个板块的需求信息|
|获取当前论坛板块的总帖子数|`/section/get-total-post-count`|`get`|`?section_name=:section_name`|`{"section_name":String,"count":Number}`|前端需要获取的信息在带参的链接，返回的数据为板块名称和总帖子数|
|获取论坛板块当前页的帖子|`/section/get-per-page-post`|`post`|`{section_name:String,currunt_page:Number,per_page_count:Number,first_post_datetime:String}`|`{"section_name":String,"posts"[{"post_id":Number,"post_title":String, "post_summary":String, "post_time":String, "post_update_time":String, "author":{"id":Number, "nickname":String, "avatar":String}, "comment_count":Number}]}`|前端需要获取的信息在带参的链接，返回的数据为板块名称和总帖子数|
|获取当前论坛板块的最新帖子|`/section/get-lastest-post`|`get`|`?section_name=`|`{"post":{"post_id":Number,"post_title":String, "post_summary":String, "post_time":String, "post_update_time":String, "author":{"id":Number, "nickname":String, "avatar":String}}}`|前端需要获取的信息在带参的链接，返回的数据为板块名称和总帖子数|

### 帖子行为表
|行为|路径|方法|前端发送数据|后台返回数据|数据项说明|
|---|---|---|---|---|---|
|获取帖子内容|`/post/get-post`|`get`|`?post_id=:post_id`|`{"post":{"post_title": String, "post_time": String, "post_update_time": String | null, "post_content": String, "author": { "user_id": Number, "user_nickname": String | null, "avatar": String | null,}}, "is_user_author": Bool}`||
|上传帖子|`/post/edit-post`|`post`|`{"post_id":Option<i32>, "post_title":String, "post_content":String, "section_name":String, "token":String}`|`{"post_id":Number}`||
|获取帖子评论|`/post/get-per-page-comment`|`post`|`{"post_id": i32, "page": i32, "per_count":i32}`|`{"comments":[{"comment_id":Number, "user": {"user_name":String, "user_nickname":String | null, "avatar":String | null}, "comment_time":String,"comment_content":String, reply_content: null | {"comment_id":Number,"user":{"user_name":String, "user_nickname":String, "comment_content":String}}}]}`|后台返回的 `reply_content` 有可能为 `null`|
|发表帖子评论|`/post/put-post-comment`|`post`|`{"post_id":i32, "token":String, "comment_content":String, "reply_to":Option<i32>}`|`{"success":boolean, ["responseText":String]}`|在后端添加评论发生错误时，`success` 的值为 `false`|
|删除评论|`/post/delete-post-comment`|`post`|`{"comment_id":i32, "token":String}`|`{"success":boolean, ["responseText":String]}`|在后端发生错误时，`success` 的值为 `false`|
|搜索帖子|`/post/search`|`POST`|`{"keywords":String, "title": bool, "content": bool, "section_name": Vec<String>}`|`{"posts"[{"post_id":Number,"post_title":String, "post_summary":String, "post_time":String, "post_update_time":String, "author":{"id":Number, "nickname":String, "avatar":String}, "comment_count":Number, "section_name":String, "section_name_zh":String}]}`|其中 `text` 为 `String` 类型，代表搜索的内容；`title` 为 `Boolean` 类型，表示是否搜索标题，默认为 `true`；`content` 为 `Boolean` 类型，表示是否搜索内容，默认为 `true`；`section_name` 可以有多个，指定搜索的范围，默认为全站。|

## 获取论坛板块状态
