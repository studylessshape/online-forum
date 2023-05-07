import { createRouter, createWebHistory, createWebHashHistory } from 'vue-router'
import NotFoundComponent from './404.vue'
import Cookies from 'js-cookie';
import { cookieName } from '../components/ServerConfig.vue';

const routes = [
    { path: '/', component: () => import("@/routes/Home.vue"), name: "home", meta: {title: "泰格论坛"}},
    { path: '/section/:section_name', name: 'section', component: () => import("@/routes/Section.vue")},
    { path: '/section/:section_name/:post_id', component: () => import("@/routes/Post.vue") },
    { path: '/post/post-add/:section_name', component: () => import("@/routes/PostEdit.vue"), meta: {title: "添加帖子 - 泰格论坛"}},
    { path: '/post/post-edit/:section_name/:post_id', component: () => import("@/routes/PostEdit.vue"), meta: {title: "编辑帖子 - 泰格论坛"}},
    { path: '/post/search', name: "search", component: () => import("@/routes/Search.vue"), props: true, name: "search" },
    { path: '/user/user-profiles/:user_name', component: () => import("@/routes/UserProfiles.vue") },
    { path: '/user/forget-password', component: () => import("@/routes/ForgetPassword.vue"), name: "forget-password", meta: {title: "忘记密码 - 泰格论坛"}},
    { path: '/404', name: 'notfound', component: () => import("@/routes/404.vue"), meta: {title: "页面不存在 - 泰格论坛"} },
    // { path: '/:pathMatch(.*)*', redirect: 'notfound'},
    { path: '/:pathMatch(.*)*', component: NotFoundComponent },
];

const router = createRouter({
    // 内部提供了 history 模式的实现。
    // history: createWebHashHistory(),
    history: createWebHistory(),
    routes, // `routes: routes` 的缩写
})

router.beforeEach((to, _from, next) => {
    if (to.meta.title) {
        document.title = to.meta.title;
    }
    if (to.name == 'forget-password' && Cookies.get(cookieName.token) != undefined) {
        next({ name: "home" })
    }
    else next();
})

export default router;