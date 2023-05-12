# 论坛管理设计
## 行为表
|行为|路径|方法|前端发送数据|后台返回数据|数据项说明|
|---|---|---|---|---|---|
|获取用户|`/manage/user/get`|`get`||`{"users":[{"user_id":i32, "avatar":String, "user_name":String, "user_nickname": String, "regist_time": String, "lastest_sign":String, "lastest_post_time":Option<NaiveDatetime>, "post_count":u32, "lastest_comment_time":Option<NaiveDatetime>, "comment_count":u32}]}`||
|删除用户|`/manage/user/del`|`post`|`{"user_id":Vec<u32>}`|`{"success":bool, ["responseText":String]}`|当后端出现报错信息，会返回到 `responseText` 中|
|获取帖子|`/manage/post/get`|`get`|`?section_name=`|`{"posts":[{"post_id":i32,"post_title":String, "post_summary":String,""}]}`||
|删除帖子|`/manage/post/del`|`del`|`{"post_id":Vec<i32>}`|`{"success":bool, ["responseText":String]}`|当后端出现报错信息，会返回到 `responseText` 中|