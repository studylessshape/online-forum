<script>
import { RouterLink } from 'vue-router'
export default {
    components: {
        RouterLink,
    }
}
</script>
<template>
    <div class="container card-footer pt-3 footer">
        <footer class="py-3 mt-3 color-auto">
            <ul class="nav justify-content-center border-bottom pb-3 mb-3">
                <li class="nav-item"><RouterLink to="/" class="nav-link px-2 text-muted">首页</RouterLink></li>
                <li class="nav-item"><RouterLink to="/section/notify" class="nav-link px-2 text-muted">论坛通告</RouterLink></li>
                <li class="nav-item"><RouterLink to="/section/tech-share" class="nav-link px-2 text-muted">技术分享</RouterLink></li>
                <li class="nav-item"><RouterLink to="/section/tech-QA" class="nav-link px-2 text-muted">技术问答</RouterLink></li>
                <li class="nav-item"><RouterLink to="/section/chat" class="nav-link px-2 text-muted">灌水闲聊</RouterLink></li>
                <li class="nav-item"><RouterLink to="/section/issue" class="nav-link px-2 text-muted">论坛反馈</RouterLink></li>
            </ul>
            <p class="text-center text-muted">© 2023 薛仕豪毕设作品</p>
        </footer>
    </div>
</template>

<style>
.footer {
    margin-top: auto;
}
.color-auto {
    mix-blend-mode: difference;
}
</style>