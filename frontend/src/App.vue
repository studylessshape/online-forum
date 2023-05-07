<script>
import Navbar from './components/Navbar.vue'
import NavFooter from './components/Footer.vue'
import { RouterView, onBeforeRouteUpdate } from 'vue-router'

export default {
  components: {
    Navbar,
    NavFooter,
    RouterView,
  },
  data() {
    return {

    }
  },
  setup() {
    return {

    }
  },
  methods: {

  },
  mounted() {
  },
}
</script>

<template>
  <main>
    <Navbar ref="navbar"></Navbar>
    <router-view></router-view>
    <nav-footer></nav-footer>
  </main>
</template>

<style></style>