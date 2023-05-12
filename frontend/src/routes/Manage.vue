<script>
import { getUrl, managePath, sectionName, sectionNameZh } from '../components/ServerConfig.vue';
import CommentDisabed from '../components/icons/OutLineCommentDisabed.vue';
import { displaySqlDatetime, displayDate } from '../components/Utils.vue';
import { ElMessage } from 'element-plus';
import { marked } from 'marked';
import { convert } from 'html-to-text';

export default {
    components: {
        CommentDisabed,
    },
    data() {
        return {
            sectionName,
            isMenuCollapse: false,
            disableMenu: false,
            curPanel: "users",
            isLoading: false,
            users: [],
            selectUsers: [],
            delDialogVisible: false,
            curSection: null,
            posts: [],
            selectPosts: [],
        }
    },
    methods: {
        getUrl,
        displayDate,
        displaySqlDatetime,
        delCondition() {
            return (this.curPanel == 'users' && this.selectUsers.length > 0) || (this.curPanel == 'posts' && this.selectPosts.length > 0);
        },
        async getUsers() {
            this.curSection = null;
            this.curPanel = 'users';
            if (this.users.length == 0) {
                this.isLoading = true;
                await fetch(getUrl(managePath.userGet), { mode: "cors", })
                    .then(response => response.json())
                    .then(data => {
                        if (data.users) {
                            this.users = data.users;
                        } else {
                            ElMessage.error("加载失败");
                        }
                    }).catch(err => {
                        ElMessage.error("加载失败");
                        console.error(err);
                    });
                this.isLoading = false;
            }
        },
        async getPosts(section) {
            this.curPanel = 'posts';
            this.curSection = section;
            this.isLoading = true;
            await fetch(getUrl(managePath.sectionPostGet) + "?section_name=" + section, { mode: "cors" })
                .then(response => response.json())
                .then(data => {
                    if (data.posts) {
                        this.posts = data.posts;
                    } else {
                        ElMessage.error(data.responseText);
                    }
                }).catch(err => ElMessage.error(err));
            this.isLoading = false;
        },
        /**
         * @param {Number} seconds
         */
        displayBanTime(seconds) {
            var minutes = parseInt(seconds / 60);
            seconds = seconds % 60;
            var hours = parseInt(minutes / 60);
            minutes = minutes % 60;
            var day = parseInt(hours / 24);
            hours = hours % 60;

            var res = '';
            if (day > 0) {
                res += `${day}天`;
            }
            if (hours > 0) {
                res += `${hours}小时`;
            }
            if (minutes > 0) {
                res += `${minutes}分`;
            }
            if (seconds > 0) {
                res += `${seconds}秒`;
            }
            return res;
        },
        postSummary(postSummary) {
            if (postSummary == null || postSummary == 0)
                return '';
            return convert(marked.parse(postSummary));
        },
        onUsersTableSelectionChange(val) {
            this.selectUsers = val.map(item => item.user_id);
        },
        showDelDialog() {
            if (!this.delCondition()) {
                ElMessage.warning(`未选择${this.curPanelTarget}`);
                return;
            }
            this.delDialogVisible = true;
        },
        async delUsers() {
            this.isLoading = true;
            this.disableMenu = true;
            this.delDialogVisible = false;

            await fetch(getUrl(managePath.userDel), {
                method: "POST",
                mode: "cors",
                credentials: "include",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ user_id: this.selectUsers })
            }).then(response => response.json())
                .then(data => {
                    if (data.success) {
                        this.users = this.users.filter(user => {
                            return this.selectUsers.findIndex(selUser => selUser == user.user_id) == -1;
                        });
                        this.selectUsers = [];
                        ElMessage.success("删除成功");
                    } else {
                        ElMessage.error(data.responseText);
                    }
                }).catch(err => console.error(err));

            this.isLoading = false;
            this.disableMenu = false;
        },
        onPostsTableSelectionChange(val) {
            this.selectPosts = val.map(item => item.post_id);
        },
        async delPosts() {
            this.isLoading = true;
            this.disableMenu = true;
            this.delDialogVisible = false;

            await fetch(getUrl(managePath.sectionPostDel), {
                method: "POST",
                mode: "cors",
                credentials: "include",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ post_id: this.selectPosts })
            }).then(response => response.json())
                .then(data => {
                    if (data.success) {
                        this.posts = this.posts.filter(post => {
                            return this.selectPosts.findIndex(selPost => selPost == post.post_id) == -1;
                        });
                        this.selectPosts = [];
                        ElMessage.success("删除成功");
                    } else {
                        ElMessage.error(data.responseText);
                    }
                }).catch(err => console.error(err));

            this.isLoading = false;
            this.disableMenu = false;
        },
    },
    computed: {
        curSectionNameZh() {
            return sectionNameZh(this.curSection);
        },
        curPanelTarget() {
            return this.curPanel == 'users' ? '用户' : '帖子';
        },
    },
    mounted() {
        this.getUsers();
    }
}
</script>

<template>
    <el-main class="w-100 mt-1 rounded">
        <el-container class="bg-body" style="min-height: 450px;">
            <el-aside width="fit-content">
                <el-menu :collapse="isMenuCollapse" default-active="1">
                    <el-menu-item index="0" class="justify-content-end"
                        @click="() => { isMenuCollapse = !isMenuCollapse; }">
                        <template #title>
                            <span v-if="isMenuCollapse">展开</span>
                        </template>
                        <el-icon>
                            <ArrowRightBold v-if="isMenuCollapse" />
                            <ArrowLeftBold v-else />
                        </el-icon>
                    </el-menu-item>
                    <el-menu-item index="1" @click="getUsers" :disabled="disableMenu">
                        <template #title>用户管理</template>
                        <el-icon>
                            <UserFilled />
                        </el-icon>
                    </el-menu-item>
                    <el-sub-menu index="2" :disabled="disableMenu">
                        <template #title>
                            <el-icon>
                                <Document />
                            </el-icon>
                            <span>帖子管理</span>
                        </template>
                        <el-menu-item index="2-1" @click="getPosts(sectionName.notify)">
                            <template #title>论坛通告</template>
                        </el-menu-item>
                        <el-menu-item index="2-2" @click="getPosts(sectionName.techShare)">
                            <template #title>技术分享</template>
                        </el-menu-item>
                        <el-menu-item index="2-3" @click="getPosts(sectionName.techQA)">
                            <template #title>技术问答</template>
                        </el-menu-item>
                        <el-menu-item index="2-4" @click="getPosts(sectionName.chat)">
                            <template #title>灌水闲聊</template>
                        </el-menu-item>
                        <el-menu-item index="2-5" @click="getPosts(sectionName.issue)">
                            <template #title>论坛反馈</template>
                        </el-menu-item>
                    </el-sub-menu>
                </el-menu>
            </el-aside>
            <el-container>
                <el-header class="header-bar">
                    <div class="d-flex w-100 align-items-center justify-content-between">
                        <el-dialog v-model="delDialogVisible">
                            <template #header="{ close, titleId, titleClass }">
                                <div class="my-header">
                                    <h3 :id="titleId" :class="titleClass">
                                        <el-icon>
                                            <WarnTriangleFilled color="red" />
                                        </el-icon>
                                        是否删除所选{{ curPanelTarget }}？
                                    </h3>
                                </div>
                            </template>
                            <div>
                                <p style="color: red; font-weight: bold; font-size: medium; text-indent:2em;">
                                    请谨慎考虑是否删除！删除{{ curPanelTarget }}后，已删除的{{ curPanelTarget }}无法恢复！</p>
                                <p style="color: red; font-weight: bold; font-size: medium; text-indent:2em;"
                                    v-if="curPanel == 'users'">
                                    删除用户后，用户的数据将会同时删除，包括：用户发布的帖子、用户发布的评论</p>
                            </div>
                            <template #footer>
                                <el-button type="primary" plain @click="delDialogVisible = false">取消</el-button>
                                <el-button type="danger" plain
                                    @click="() => curPanel == 'users' ? delUsers() : delPosts()">确认</el-button>
                            </template>
                        </el-dialog>
                        <div>{{ curPanel == 'users' ? '' : curSectionNameZh }}</div>
                        <el-tooltip :content="`删除选中${curPanelTarget}`" :enterable="false">
                            <el-button type="danger" @click="showDelDialog" :disabled="disableMenu">
                                <el-icon>
                                    <Delete />
                                </el-icon>
                            </el-button>
                        </el-tooltip>
                    </div>
                </el-header>
                <el-main class="p-0" v-loading="isLoading">
                    <el-scrollbar v-if="curPanel == 'users'">
                        <el-table :data="users" max-height="450" border empty-text="暂无用户"
                            @selection-change="onUsersTableSelectionChange">
                            <el-table-column type="selection" width="40" fixed />
                            <el-table-column prop="user_name" label="用户" width="150" sortable column-key="user_name">
                                <template #default="scope">
                                    <RouterLink :to="`/user/user-profiles/${scope.row.user_name}`"
                                        class="time-display d-flex button-link link text-darkgray">
                                        <el-avatar size="small"
                                            :src="scope.row.avatar ? getUrl(scope.row.avatar) : '/img/avatar.svg'"
                                            style="background-color: unset;" />
                                        <span class="ms-2">{{ scope.row.user_nickname }}</span>
                                    </RouterLink>
                                </template>
                            </el-table-column>
                            <el-table-column prop="regist_time" label="注册时间" width="150" sortable column-key="regist_time">
                                <template #default="scope">
                                    <div class="time-display">{{ displaySqlDatetime(scope.row.regist_time) }}</div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="lastest_sign" label="上一次登录时间" width="150" sortable
                                column-key="lastest_sign">
                                <template #default="scope">
                                    <div class="time-display">{{ displaySqlDatetime(scope.row.lastest_sign) }}</div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="post_count" label="发帖数" width="100" column-key="post_count">
                                <template #default="scope">
                                    <div class="time-display">{{ scope.row.post_count }}</div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="lastest_post_time" label="最新发帖时间" width="150" sortable
                                column-key="lastest_post_time">
                                <template #default="scope">
                                    <div class="time-display">{{ displaySqlDatetime(scope.row.lastest_post_time) }}</div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="comment_count" label="评论数" width="100" column-key="comment_count">
                                <template #default="scope">
                                    <div class="time-display">{{ scope.row.comment_count }}</div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="lastest_comment_time" label="最新评论时间" width="150" sortable
                                column-key="lastest_comment_time">
                                <template #default="scope">
                                    <div class="time-display">{{ displaySqlDatetime(scope.row.lastest_comment_time) }}</div>
                                </template>
                            </el-table-column>
                        </el-table>
                    </el-scrollbar>
                    <el-scrollbar v-else>
                        <el-table :data="posts" max-height="360" stripe border ref="searchResultTable"
                            @selection-change="onPostsTableSelectionChange">
                            <el-table-column type="selection" width="40" fixed />
                            <el-table-column label="标题" prop="post_title" width="200" sortable>
                                <template #default="scope">
                                    <RouterLink :to="`/section/${scope.row.section_name}/${scope.row.post_id}`"
                                        class="time-display">
                                        <el-link type="primary">{{ scope.row.post_title }}</el-link>
                                    </RouterLink>
                                </template>
                            </el-table-column>
                            <el-table-column prop="author" label="用户" width="150" sortable column-key="user">
                                <template #default="scope">
                                    <RouterLink :to="`/user/user-profiles/${scope.row.author.user_name}`"
                                        class="time-display d-flex button-link link text-darkgray">
                                        <el-avatar size="small"
                                            :src="scope.row.author.avatar ? getUrl(scope.row.author.avatar) : '/img/avatar.svg'"
                                            style="background-color: unset;" />
                                        <span class="ms-2">{{ scope.row.author.user_nickname }}</span>
                                    </RouterLink>
                                </template>
                            </el-table-column>
                            <el-table-column prop="post_summary" label="简要" width="400">
                                <template #default="scope">
                                    <div class="time-display">{{ postSummary(scope.row.post_summary) }}</div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="section_name" label="发布板块" width="120" sortable column-key="section">
                                <template #default="scope">
                                    <RouterLink :to="`/section/${scope.row.section_name}`" class="time-display">
                                        <el-link type="primary">{{ scope.row.section_name_zh }}</el-link>
                                    </RouterLink>
                                </template>
                            </el-table-column>
                            <el-table-column prop="post_time" label="发布时间" width="210" sortable column-key="post-time">
                                <template #default="scope">
                                    <div class="time-display">{{ displaySqlDatetime(scope.row.post_time) }}</div>
                                </template>
                            </el-table-column>
                            <el-table-column prop="post_update_time" label="修改时间" width="210" sortable
                                column-key="post-update-time">
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
                        </el-table>
                    </el-scrollbar>
                </el-main>
            </el-container>
        </el-container>
    </el-main>
</template>

<style>
.header-bar {
    display: flex;
    align-content: center;
    justify-content: end;
    flex-wrap: wrap;
    background-image: linear-gradient(-20deg, #e9defa 0%, #fbfcdb 100%);
}
</style>