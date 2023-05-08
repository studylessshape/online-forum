<script>
import Navbar from './components/Navbar.vue'
import NavFooter from './components/Footer.vue'
import { RouterView } from 'vue-router'
import { changeServerUrl, serverUrl } from './components/ServerConfig.vue'

export default {
  components: {
    Navbar,
    NavFooter,
    RouterView,
  },
  data() {
    return {
      isConfig: false,
      isConfigError: false,
    }
  },
  methods: {
    changeConfigState(isSuccess) {
      this.isConfig = isSuccess;
      this.isConfigError = !isSuccess;
    }
  },
  mounted() {
    changeServerUrl(window.server);
    fetch(serverUrl, {mode: "cors"})
      .then(response => {
        this.changeConfigState(response.ok);
      }).catch(_err => {
        this.changeConfigState(false);
      })
  },
}
</script>

<template>
  <main ref="appBody">
    <!-- <div v-if="isConfig"> -->
    <el-dialog v-model="isConfigError" title="配置服务错误" width="30%" :show-close="false" :close-on-click-modal="false"
      :close-on-press-escape="false">
      <p><el-icon size="x-large" color="red" class="w-100">
          <Warning />
        </el-icon></p>
      <p>请在配置文件 "config.js" 中配置好服务器地址！并确保服务器能够访问！</p>
    </el-dialog>
    <div v-if="isConfig">
      <Navbar ref="navbar"></Navbar>
      <router-view></router-view>
      <nav-footer></nav-footer>
    </div>
  </main>
</template>