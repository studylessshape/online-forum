<script>
import Cookies from "js-cookie";
import { cookieName } from '../components/ServerConfig.vue';
import { getUrl, userPath } from '../components/ServerConfig.vue';
import { displaySqlDatetime, imageFileTypes, validFileType, cookieWatch, isEmail, checkPasswordValid } from '../components/Utils.vue';
import { ElMessage } from 'element-plus';
import { marked } from 'marked';
import { convert } from 'html-to-text';
import { RouterLink } from "vue-router";

export default {
    components: {
        RouterLink,
    },
    data() {
        return {
            tabActive: "user-profile",
            userProfilesView: {
                avatar: null,
                user_nickname: null,
                regist_time: null,
                description: null,
                isLoading: true,
            },
            // relate to profiles update
            newNickname: '',
            newDescription: '',
            editIsLoading: false,
            // relate to password update
            pwdEmail: '',
            pwdEmailCode: '',
            pwdNewPassword: '',
            pwdNewPasswordAgain: '',
            getEmailCodeButtonText: "获取验证码",
            isGettingEmailCode: false,
            isDisableEmailCodeButton: false,
            emailCodeButtonType: 'primary',
            isPasswordInUpdate: false,
            // relate to user posts
            userPosts: [],
            isUserPostsTableLoading: true,
            delPostsDialogVisible: false,
            userPostsTableSelect: [],
        }
    },
    methods: {
        resetAll() {
            Object.assign(this.$data, this.$options.data());
        },
        displaySqlDatetime,
        getUserProfiles() {
            fetch(getUrl(userPath.getUserProfiles) + "?user_name=" + this.userName, {
                mode: "cors",
            }).then(response => response.json())
                .then(data => {
                    if (data.regist_time) {
                        this.userProfilesView.avatar = data.avatar ? getUrl(data.avatar) : "/img/avatar.svg";
                        this.userProfilesView.user_nickname = data.user_nickname;
                        this.userProfilesView.regist_time = data.regist_time;
                        this.userProfilesView.description = data.description;
                        this.userProfilesView.isLoading = false;

                        this.newNickname = this.userProfilesView.user_nickname;
                        this.newDescription = this.userProfilesView.description;
                        document.title = `用户：${this.userProfilesView.user_nickname} - 泰格论坛`;
                    } else {
                        ElMessage.error(data.responseText);
                    }
                })
                .catch(err => ElMessage.error(err))
        },
        uploadAvatarClick() {
            var fileSel = document.getElementById("file-sel");
            fileSel.addEventListener('change', this.uploadAvatarToServer);
            fileSel.click();
        },
        uploadAvatarToServer() {
            var files = document.getElementById("file-sel").files;
            if (!files.length) {
                alert("没有选择图片");
                return;
            }
            this.editIsLoading = true;
            for (var i = 0; i < files.length; i++) {
                if (!validFileType(files[i], imageFileTypes)) {
                    continue;
                }
                var formData = new FormData();
                formData.append("file", files[i]);
                var url = getUrl(userPath.updateAvatar) + '/' + files[i].name;
                fetch(url, {
                    method: "POST",
                    mode: "cors",
                    credentials: "include",
                    body: formData,
                })
                    .then(response => response.json())
                    .then(data => {
                        // 获取到地址后，更新 Cookie 和 当期页面状态
                        if (data.avatar) {
                            cookieWatch.setCookie(cookieName.avatar, data.avatar);
                            this.userProfilesView.avatar = getUrl(data.avatar);
                            this.editIsLoading = false;
                            ElMessage.success("头像修改成功");
                        } else {
                            ElMessage.error(data.responseText);
                        }
                    })
                    .catch(error => ElMessage.error(error))
            }
        },
        resetEdit() {
            this.editIsLoading = true;
            this.newNickname = this.userProfilesView.user_nickname;
            this.newDescription = this.userProfilesView.description;
            ElMessage("已重置");
            this.editIsLoading = false;
        },
        async uploadNewProfiles() {
            this.editIsLoading = true;
            var url = getUrl(userPath.updateProfiles) + `?user_nickname=${this.newNickname}&description=${this.newDescription}`;
            await fetch(url, {
                mode: "cors",
                credentials: "include"
            }).then(response => response.json())
                .then(data => {
                    if (data.user_nickname) {
                        this.userProfilesView.user_nickname = data.user_nickname;
                        this.userProfilesView.description = data.description;
                        cookieWatch.setCookie(cookieName.user_nickname, data.user_nickname);
                        ElMessage.success("保存成功");
                    } else {
                        ElMessage.error(data.responseText);
                    }
                })
                .catch(err => ElMessage.error(err));

            this.editIsLoading = false;
        },
        getEmailCode() {
            if (this.userName == 'root') {
                ElMessage.warning("无法获取");
                return;
            }
            this.isGettingEmailCode = true;
            if (!isEmail(this.pwdEmail)) {
                ElMessage.warning("请输入正确的邮箱");
                this.isGettingEmailCode = false;
                return;
            }
            fetch(getUrl(userPath.emailCodeGet), {
                method: 'POST',
                mode: 'cors',
                credentials: 'include',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ email: this.pwdEmail, check_user_rule: 'is-exist' })
            }).then(response => response.json())
                .then(data => {
                    if (data.message) {
                        this.getEmailCodeButtonText = data.message;
                    } else {
                        this.getEmailCodeButtonText = data.responseText;
                        this.emailCodeButtonType = 'danger';
                    }
                    this.isDisableEmailCodeButton = true;
                    this.isGettingEmailCode = false;
                    setTimeout(() => {
                        this.getEmailCodeButtonText = "再次发送";
                        this.isDisableEmailCodeButton = false;
                    }, 30000);
                })
                .catch(err => {
                    ElMessage.error(err);
                    this.isGettingEmailCode = false;
                    this.isDisableEmailCodeButton = false;
                })
        },
        updatePassword() {
            if (this.userName == 'root') {
                ElMessage.warning("无法修改");
                return;
            }
            if (!isEmail(this.pwdEmail)) {
                ElMessage.warning("请输入正确的邮箱");
                return;
            }
            if (this.pwdEmailCode.length != 4) {
                ElMessage.error("请输入正确的验证码");
                return;
            }
            if (!checkPasswordValid(this.pwdNewPassword)) {
                ElMessage.error("密码格式或长度不正确：至少包含大小写字母、数字、特殊字符中的四种中的三种");
                return;
            }
            if (this.pwdNewPassword != this.pwdNewPasswordAgain) {
                ElMessage.error("两次密码不一致");
                return;
            }
            this.isPasswordInUpdate = true;
            fetch(getUrl(userPath.updatePassword), {
                method: "POST",
                mode: "cors",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ email: this.pwdEmail, code: parseInt(this.pwdEmailCode), password: this.pwdNewPassword }),
            }).then(response => response.json())
                .then(data => {
                    if (data.message) {
                        ElMessage.success(data.message);
                    } else {
                        ElMessage.error(data.responseText);
                    }
                    this.isPasswordInUpdate = false;
                })
                .catch(err => ElMessage.error(err));
        },
        onTabsChange(tabName) {
            if (tabName == "user-posts") {
                this.getUserPosts();
            }
        },
        async getUserPosts() {
            await fetch(getUrl(userPath.getUserPosts) + "?user_name=" + this.userName, { mode: "cors", })
                .then(response => response.json())
                .then(data => {
                    if (data.posts) {
                        this.userPosts = data.posts;
                    } else {
                        ElMessage.error(data.responseText);
                    }
                })
                .catch(err => ElMessage.error(err));

            this.isUserPostsTableLoading = false;
        },
        postSummary(postSummary) {
            if (postSummary == null || postSummary == 0)
                return '';
            return convert(marked.parse(postSummary));
        },
        onPostsTableSelectionChange(val) {
            this.userPostsTableSelect = val.map(item => item.post_id);
        },
        showDelPostsDialog() {
            if (this.userPostsTableSelect.length == 0) {
                ElMessage.warning("未选择任何帖子");
                return;
            }
            this.delPostsDialogVisible = true;
        },
        async delPosts() {
            if (this.userPostsTableSelect.length == 0) {
                ElMessage.warning("未选择任何帖子");
                return;
            }
            this.delPostsDialogVisible = false;
            this.isUserPostsTableLoading = true;
            await fetch(getUrl(userPath.delUserPosts), {
                method: "POST",
                mode: "cors",
                credentials: "include",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ post_id: this.userPostsTableSelect })
            }).then(response => response.json())
                .then(data => {
                    if (data.success) {
                        // 过滤掉被删除的帖子
                        this.userPosts = this.userPosts.filter(item => {
                            return this.userPostsTableSelect.findIndex(elem => elem == item.post_id) == -1;
                        });
                        this.userPostsTableSelect = [];
                        ElMessage.success("删除成功");
                    } else {
                        ElMessage.error(data.responseText);
                    }
                })
            this.isUserPostsTableLoading = false;
        }
    },
    computed: {
        userName() {
            return this.$route.params.user_name;
        },
        isCurUser() {
            return this.userName == Cookies.get(cookieName.username);
        },
    },
    watch: {
        userName() {
            this.resetAll();
            this.getUserProfiles();
        },
    },
    mounted() {
        this.getUserProfiles();
    }
}
</script>

<template>
    <el-main class="w-100 mt-1" style="flex-grow: 1;">
        <el-tabs v-model="tabActive" tab-position="left" @tab-change="onTabsChange"
            class="pt-4 pb-4 ps-0 pe-0 ps-sm-4 pe-sm-4 border border-2 rounded rounded-2 bg-body shadow" type="card">
            <!-- 用户资料 -->
            <el-tab-pane name="user-profile" style="display: flex; height: 100%;">
                <template #label>
                    <el-icon title="用户资料">
                        <User />
                    </el-icon>
                </template>
                <el-main v-loading="userProfilesView.isLoading" class="container">
                    <div class="row">
                        <div class="col-md-3">
                            <div class="row justify-content-center">
                                <el-avatar :src="userProfilesView.avatar" alt="头像" class="p-0"
                                    style="background-color: unset;" :size="180" />
                            </div>
                            <div class="row justify-content-center fw-bold fs-4">{{ userProfilesView.user_nickname }}</div>
                        </div>
                        <div class="col-md-9">
                            <div class="row border-bottom">
                                <div class="form-floating">
                                    <input type="text" readonly class="form-control-plaintext" id="nicknameView"
                                        placeholder="昵称" v-model="userProfilesView.user_nickname">
                                    <label for="nicknameView">昵称：</label>
                                </div>
                            </div>
                            <div class="row border-bottom">
                                <div class="form-floating">
                                    <input type="text" readonly class="form-control-plaintext" id="registTimeView"
                                        placeholder="注册时间" :value="displaySqlDatetime(userProfilesView.regist_time)">
                                    <label for="registTimeView">注册时间：</label>
                                </div>
                            </div>
                            <div class="row">
                                <div class="form-floating">
                                    <textarea class="form-control-plaintext" placeholder="简介" style="resize: none;"
                                        id="descriptionView" readonly disabled
                                        v-model="userProfilesView.description"></textarea>
                                    <label for="descriptionView">简介：</label>
                                </div>
                            </div>
                        </div>
                    </div>
                </el-main>
            </el-tab-pane>
            <!-- 用户资料修改 -->
            <el-tab-pane v-if="isCurUser" name="user-profile-edit">
                <template #label>
                    <el-icon title="用户资料修改">
                        <EditPen />
                    </el-icon>
                </template>
                <el-main class="container" v-loading="editIsLoading">
                    <div class="row align-items-center">
                        <div class="col-md-2 fw-bold">点击修改头像</div>
                        <div class="col-md-4">
                            <a href="#" title="上传新头像" style="width: fit-content;" @click="uploadAvatarClick">
                                <img :src="userProfilesView.avatar" alt="avatar" id="user-avatar-edit"
                                    class="avatar-profiles">
                            </a>
                            <input type="file" id="file-sel" accept=".jpg, .jpeg, .png, .svg, .webp" style="display:none"
                                @onchange="uploadAvatarToServer">
                        </div>
                    </div>
                    <div class="row">
                        <el-divider>
                            <el-icon>
                                <MoreFilled />
                            </el-icon>
                        </el-divider>
                    </div>
                    <div class="row">
                        <el-form label-width="40px" @submit="()=>{}">
                            <el-form-item label="昵称">
                                <el-input v-model="newNickname" placeholder="请输入昵称" minlength="3" maxlength="20"
                                    show-word-limit required />
                            </el-form-item>
                            <el-form-item label="简介">
                                <el-input v-model="newDescription" type="textarea" placeholder="请输入简介" />
                            </el-form-item>
                            <el-form-item>
                                <el-button type="primary" @click="uploadNewProfiles">保存</el-button>
                                <el-button @click="resetEdit">重置</el-button>
                            </el-form-item>
                        </el-form>
                    </div>
                </el-main>
            </el-tab-pane>
            <!-- 用户密码修改 -->
            <el-tab-pane v-if="isCurUser" name="user-pwd-edit">
                <template #label>
                    <el-icon title="密码修改">
                        <Lock />
                    </el-icon>
                </template>
                <el-main class="ps-0">
                    <el-form label-width="80px" v-loading="isPasswordInUpdate" @submit="()=>{}">
                        <input type="email" style="opacity: 0;position:absolute;width:0;height:0;" autocomplete="username">
                        <input type="password" style="opacity: 0;position:absolute;width:0;height:0;"
                            autocomplete="current-password">
                        <el-form-item label="邮箱">
                            <el-input type="email" placeholder="请输入注册时的邮箱" v-model="pwdEmail" :disabled="isGettingEmailCode"
                                autocomplete="off"></el-input>
                        </el-form-item>
                        <el-form-item>
                            <el-button :type="emailCodeButtonType" :loading="isGettingEmailCode" :plain="true"
                                @click="getEmailCode" :disabled="isDisableEmailCodeButton">
                                {{ getEmailCodeButtonText }}
                            </el-button>
                        </el-form-item>
                        <el-form-item label="验证码">
                            <el-input type="number" placeholder="请输入邮箱验证码" maxlength="4" v-model="pwdEmailCode"
                                :controls="false" autocomplete="off"></el-input>
                        </el-form-item>
                        <el-form-item label="新密码">
                            <el-input type="password" placeholder="请输入新密码" v-model="pwdNewPassword" autocomplete='off'
                                show-password></el-input>
                        </el-form-item>
                        <el-form-item label="确认密码">
                            <el-input type="password" placeholder="请再次输入新密码" v-model="pwdNewPasswordAgain"
                                autocomplete='off' show-password></el-input>
                        </el-form-item>
                        <el-form-item>
                            <el-button type="primary" @click="updatePassword">确认</el-button>
                        </el-form-item>
                    </el-form>
                </el-main>
            </el-tab-pane>
            <el-tab-pane v-if="isCurUser" name="user-posts">
                <template #label>
                    <el-icon title="发布的帖子">
                        <Tickets />
                    </el-icon>
                </template>
                <el-table :data="userPosts" v-loading="isUserPostsTableLoading" max-height="360" stripe border empty-text="暂无帖子"
                    @selection-change="onPostsTableSelectionChange">
                    <el-table-column type="selection" width="40" fixed />
                    <el-table-column prop="post_title" label="标题" width="200">
                        <template #default="scope">
                            <div class="time-display">{{ scope.row.post_title }}</div>
                        </template>
                    </el-table-column>
                    <el-table-column label="操作" width="100">
                        <template #default="scope">
                            <RouterLink :to="`/section/${scope.row.section_name}/${scope.row.post_id}`"
                                style="margin-right: 8px;">
                                <el-tooltip class="box-item" content="查看帖子" placement="top-start">
                                    <el-button size="small" circle>
                                        <el-icon>
                                            <View />
                                        </el-icon>
                                    </el-button>
                                </el-tooltip>
                            </RouterLink>
                            <RouterLink :to="`/post/post-edit/${scope.row.section_name}/${scope.row.post_id}`">
                                <el-tooltip class="box-item" content="编辑帖子" placement="top-start">
                                    <el-button size="small" type="primary" circle>
                                        <el-icon>
                                            <Edit />
                                        </el-icon>
                                    </el-button>
                                </el-tooltip>
                            </RouterLink>
                        </template>
                    </el-table-column>
                    <el-table-column prop="post_time" label="发布时间" width="210">
                        <template #default="scope">
                            <div class="time-display">{{ displaySqlDatetime(scope.row.post_time) }}</div>
                        </template>
                    </el-table-column>
                    <el-table-column prop="post_summary" label="简要" width="400">
                        <template #default="scope">
                            <div class="time-display">{{ postSummary(scope.row.post_summary) }}</div>
                        </template>
                    </el-table-column>
                    <el-table-column prop="post_update_time" label="修改时间" width="210">
                        <template #default="scope">
                            <div class="time-display">
                                {{
                                    scope.row.post_update_time
                                    ? displaySqlDatetime(scope.row.post_update_time)
                                    : ''
                                }}
                            </div>
                        </template>
                    </el-table-column>
                    <el-table-column prop="section_name_zh" label="论坛板块" width="120" />
                    <el-table-column prop="comment_count" label="评论数" width="120" />
                </el-table>
                <div class="d-flex justify-content-end">
                    <el-dialog v-model="delPostsDialogVisible">
                        <template #header="{ close, titleId, titleClass }">
                            <div class="my-header">
                                <h3 :id="titleId" :class="titleClass">
                                    <el-icon>
                                        <WarnTriangleFilled color="red" />
                                    </el-icon>
                                    是否删除所选帖子？
                                </h3>
                            </div>
                        </template>
                        <div>
                            <p style="color: red; font-weight: bold; font-size: medium; text-indent:2em;">
                                请谨慎考虑是否删除帖子！删除帖子后，已删除的帖子无法恢复！</p>
                        </div>
                        <template #footer>
                            <el-button type="primary" plain @click="delPostsDialogVisible = false">取消</el-button>
                            <el-button type="danger" plain @click="delPosts">确认</el-button>
                        </template>
                    </el-dialog>
                    <el-button type="danger" plain style="margin-top: 10px;" @click="showDelPostsDialog">删除所选帖子</el-button>
                </div>
            </el-tab-pane>
        </el-tabs>
    </el-main>
</template>

<style>
input::-webkit-inner-spin-button {
    -webkit-appearance: none !important;

}

input::-webkit-outer-spin-button {
    -webkit-appearance: none !important;

}

.color-auto {
    mix-blend-mode: difference;
}
</style>