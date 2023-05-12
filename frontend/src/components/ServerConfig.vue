<script>
var serverUrl = "http://localhost:8082";
/**
 * 
 * @param {{protocol:String, host:String, port: Number}} url 
 */
function changeServerUrl(url) {
    if (url.port != undefined && url.port != null) {
        serverUrl = `${url.protocol}://${url.host}:${url.port}`;
    } else {
        serverUrl = `${url.protocol}://${url.host}`;
    }
}

const userPath = {
    signIn: "/user/sign-in",
    onlineCheck: "/user/online-check",
    userProfiles: "/user/user-profiles",
    emailCodeGet: "/user/require-email-code",
    emailCodeCheck: "/user/check-email-code",
    signUp: "/user/sign-up",
    getUserProfiles: "/user/profiles/get",
    updateAvatar: "/user/profiles/update/avatar",
    updateNickname: "/user/profiles/update/nickname",
    updateProfiles: "/user/profiles/update/profiles",
    updatePassword: "/user/profiles/update/password",
    getUserPosts: "/user/profiles/posts/get",
    delUserPosts: "/user/profiles/posts/del",
};
const sectionPath = {
    getStatus: "/section/get-status",
    getTotalCount: "/section/get-total-post-count",
    getPerPagePost: "/section/get-per-page-post",
    getLastestPost: "/section/get-lastest-post",
};
const cookieName = {
    username: "username",
    user_nickname: "usernickname",
    avatar: "avatar",
    token: "token",
};
const sectionName = {
    home: "home",
    notify: "notify",
    techShare: "tech-share",
    techQA: "tech-QA",
    chat: "chat",
    issue: "issue",
};
const postPath = {
    getPost: "/post/get-post",
    editPost: "/post/edit-post",
    delPost: "/post/del-post",
    getComments: "/post/get-per-page-comment",
    putComment: "/post/put-post-comment",
    deleteComment: "/post/delete-post-comment",
    search: "/post/search",
}
const filePath = {
    uploadImage: "/files/image",
    uploadFiles: "/files/upload",
}
const managePath = {
    userGet: "/manage/user/get",
    userDel: "/manage/user/del",
    sectionPostGet: "/manage/post/get",
    sectionPostDel: "/manage/post/del",
}
/**
 * 
 * @param {String} section 
 */
function sectionNameZh(section) {
    switch (section) {
        case sectionName.notify:
            return "论坛通告";
        case sectionName.techShare:
            return "技术分享";
        case sectionName.techQA:
            return "技术问答";
        case sectionName.chat:
            return "灌水闲聊";
        case sectionName.issue:
            return "论坛反馈";
    }
}
/**
 * server url append path
 * @param {String} path
 */
function getUrl(path) {
    return serverUrl + path;
}
/**
 * get section route as path
 * @param {String} name 
 */
function getSectionRoute(name) {
    return '/section/' + name;
}
export {
    serverUrl,
    userPath,
    sectionPath,
    cookieName,
    sectionName,
    filePath,
    postPath,
    managePath,
    getUrl,
    getSectionRoute,
    sectionNameZh,
    changeServerUrl,
}
</script>