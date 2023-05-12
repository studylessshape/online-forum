<script>
import NavbarUser from "./NavbarUser.vue";
import { RouterLink } from "vue-router";
import Cookies from "js-cookie";
import { serverUrl, userPath, cookieName, sectionName, getUrl } from "./ServerConfig.vue";
import { cookieWatch } from "./Utils.vue";

export default {
    components: {
        NavbarUser,
        RouterLink,
    },
    data() {
        return {
            username: '',
            user_nickname: '',
            avatar: '',
            searchWords: ''
        }
    },
    methods: {
        clearUserCookie() {
            Cookies.remove(cookieName.username);
            Cookies.remove(cookieName.user_nickname);
            Cookies.remove(cookieName.token);
            Cookies.remove(cookieName.avatar);
            location.reload();
        },
        setUserDom() {
            this.username = Cookies.get(cookieName.username);
            this.user_nickname = Cookies.get(cookieName.user_nickname);
            this.avatar = Cookies.get(cookieName.avatar);
            this.changeAvatar(this.avatar);
        },
        sign_out() {
            this.clearUserCookie();
            location.reload();
        },
        collapseNav() {
            $("#navbar-content").collapse("hide");
        },
        // check user online status
        userOnlineCheck() {
            var token = Cookies.get("token");
            var username = Cookies.get("username");
            if (!($.isEmptyObject(token)) && !($.isEmptyObject(username))) {
                fetch(serverUrl + userPath.onlineCheck, {
                    method: 'POST',
                    mode: "cors",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify({ user_name: username, token: token })
                }).then(response => {
                    if (!response.ok) {
                        throw new Error(response.statusText);
                    }
                    return response.json();
                })
                    .then(data => {
                        if (data.check_online) {
                            this.setUserDom();
                        } else {
                            this.clearUserCookie();
                        }
                    })
                    .catch(_error => {
                        this.clearUserCookie();
                        location.reload();
                    })
            }
        },
        userProfilesUrl(username) {
            return userPath.userProfiles + "/" + username;
        },
        sectionUrlPath(section) {
            return '/section/' + section;
        },
        changeAvatar(avatar) {
            if (!($.isEmptyObject(avatar)) && avatar != null && avatar != 'null' && avatar != undefined) {
                $("#user-avatar").css("background-image", "url('" + getUrl(avatar) + "')");
            }
        },
        onEnterSearch() {
            this.collapseNav();
            this.$router.push({ name: "search", query: { keywords: this.searchWords } })
            this.searchWords = '';
        }
    },
    computed: {
        isUserSignIn() {
            return !$.isEmptyObject(Cookies.get(cookieName.token));
        },
        displayName() {
            return !($.isEmptyObject(this.user_nickname)) && this.user_nickname != '' && this.user_nickname != 'null' ? this.user_nickname : this.curUsername;
        },
        section() {
            return sectionName;
        },
        isSearchRoute() {
            return this.$route.name == 'search';
        },
        curUsername() {
            return cookieWatch.getCookie(cookieName.username);
        }
    },
    watch: {
        avatar(avatar) {
            this.changeAvatar(avatar);
        },

    },
    mounted() {
        this.userOnlineCheck();
        cookieWatch.addListener(cookieWatch.postEventName.onSetCookie, (cob) => {
            switch (cob.detail.cookieName) {
                case cookieName.avatar:
                    this.avatar = cob.detail.newValue;
                    this.changeAvatar(this.avatar);
                    break;
                case cookieName.user_nickname:
                    this.user_nickname = cob.detail.newValue;
                    break;
            };
        })
    },
};
</script>

<template>
    <div>
        <nav class="navbar navbar-expand-lg navbar-light bg-light fixed-top">
            <div class="container-fluid d-flex">
                <RouterLink class="navbar-brand" to="/" @click="collapseNav">泰格论坛</RouterLink>
                <!-- user guide -->
                <div class="btn-group order-lg-2">
                    <button type="button" class="btn btn-light dropdown-toggle" data-bs-toggle="dropdown"
                        aria-expanded="false">
                        <span class="navbar-toggler-icon user-button rounded-circle" id="user-avatar"></span>
                    </button>
                    <ul class="dropdown-menu dropdown-menu-sm-end dropdown-center">
                        <div id="no-user" v-if="!isUserSignIn">
                            <li>
                                <h6 class="dropdown-header">您还未登录，请选择：</h6>
                            </li>
                            <li>
                                <button class="dropdown-item" data-bs-target="#sign-in-modal" data-bs-toggle="modal">
                                    登录
                                </button>
                            </li>
                            <li>
                                <button class="dropdown-item" data-bs-target="#sign-up-modal" data-bs-toggle="modal">
                                    注册
                                </button>
                            </li>
                            <li>
                                <hr class="dropdown-divider" />
                            </li>
                            <li>
                                <RouterLink to="/user/forget-password" class="dropdown-item" role="button">忘记密码？
                                </RouterLink>
                            </li>
                        </div>
                        <div id="user" v-else>
                            <li>
                                <h6 class="dropdown-header">{{ displayName }}</h6>
                            </li>
                            <li>
                                <RouterLink class="dropdown-item" :to="userProfilesUrl(curUsername)" role="button">个人中心
                                </RouterLink>
                            </li>
                            <li v-if="curUsername == 'root'">
                                <RouterLink class="dropdown-item" to="/manage" role="button">论坛管理
                                </RouterLink>
                            </li>
                            <li>
                                <hr class="dropdown-divider" />
                            </li>
                            <li>
                                <button class="dropdown-item" @click="sign_out">
                                    退出登录
                                </button>
                            </li>
                        </div>
                    </ul>
                </div>
                <!-- 论坛板块 -->
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbar-content"
                    aria-controls="navbar-content" aria-expanded="false" aria-label="展开">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse order-lg-1" id="navbar-content">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        <li class="nav-item">
                            <RouterLink to="/" class="nav-link" id="nav-home" @click="collapseNav">
                                首页
                            </RouterLink>
                        </li>
                        <li class="nav-item">
                            <RouterLink :to="sectionUrlPath(section.notify)" class="nav-link" id="nav-notify"
                                @click="collapseNav">论坛通告
                            </RouterLink>
                        </li>
                        <li class="nav-item">
                            <RouterLink :to="sectionUrlPath(section.techShare)" class="nav-link" id="nav-tech-share"
                                @click="collapseNav">
                                技术分享</RouterLink>
                        </li>
                        <li class="nav-item">
                            <RouterLink :to="sectionUrlPath(section.techQA)" class="nav-link" id="nav-tech-QA"
                                @click="collapseNav">技术问答
                            </RouterLink>
                        </li>
                        <li class="nav-item">
                            <RouterLink :to="sectionUrlPath(section.chat)" class="nav-link" id="nav-chat"
                                @click="collapseNav">灌水闲聊
                            </RouterLink>
                        </li>
                        <li class="nav-item">
                            <RouterLink :to="sectionUrlPath(section.issue)" class="nav-link" id="nav-issue"
                                @click="collapseNav">论坛反馈
                            </RouterLink>
                        </li>
                    </ul>
                    <div class="d-flex" role="search" v-if="$route.name != 'search'">
                        <input class="form-control me-2" type="search" placeholder="搜索" aria-label="Search"
                            v-model="searchWords" @keyup.enter.native="onEnterSearch" autocomplete='off'>
                        <button class="btn btn-outline-success" @click="onEnterSearch">
                            <el-icon>
                                <Search />
                            </el-icon>
                        </button>
                    </div>
                </div>
            </div>
        </nav>
        <NavbarUser v-if="!isUserSignIn"></NavbarUser>
    </div>
</template>