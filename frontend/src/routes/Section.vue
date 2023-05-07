<script>
import { RouterLink } from 'vue-router';
import { getUrl, sectionPath, sectionNameZh, cookieName, sectionName } from '../components/ServerConfig.vue';
import Pagination from '../components/Pagination.vue';
import { marked } from 'marked';
import { convert } from 'html-to-text';
import { displaySqlDatetime } from '../components/Utils.vue';
import Cookies from 'js-cookie';
import { ElMessage } from 'element-plus';

export default {
  components: {
    RouterLink,
    Pagination,
  },
  data() {
    return {
      perPagePostCount: 10,
      currentPage: 1,
      totalPage: 1,
      firstPostDateTime: null,
      postCount: 0,
      error_message: '',
      posts: [],
      marked,
    }
  },
  methods: {
    getUrl,
    // 获取论坛板块总帖子数
    getTotalCount() {
      fetch(getUrl(sectionPath.getTotalCount) + "?section_name=" + this.currentSection, {
        method: 'get',
        mode: 'cors',
        headers: {
          "Content-Type": "application/json"
        },
      })
        .then(response => {
          return response.json();
        })
        .then(data => {
          if (data.section_name) {
            this.postCount = data.count;
            this.totalPage = parseInt(data.count / this.perPagePostCount);
            this.getPosts(this.currentPage);
          } else {
            console.log(data.responseText);
          }
        })
        .catch(err => {
          console.log(err);
        });
    },
    getPosts(page) {
      this.currentPage = page;
      var body = JSON.stringify({
        section_name: this.currentSection,
        current_page: this.currentPage,
        per_page_count: this.perPagePostCount,
        first_post_datetime: this.firstPostDateTime,
      });
      fetch(getUrl(sectionPath.getPerPagePost), {
        method: "post",
        mode: "cors",
        headers: {
          "Content-Type": "application/json",
        },
        body: body,
      }).then(response => {
        return response.json();
      })
        .then(data => {
          if (data.section_name) {
            this.posts = data.posts;
          } else {
            ElMessage.error(data.responseText);
          }
        })
        .catch(err => {
          console.log(err);
        })
    },
    resetSectionData() {
      this.currentPage = 1;
      this.totalPage = 1;
      this.firstPostDateTime = null;
      this.post_count = 0;
      this.error_message = '';
      this.posts = [];
    },
    displaySqlDatetime,
    post_summary(post) {
      return convert(marked.parse(post.post_summary));
    }
  },
  computed: {
    currentSection() {
      return this.$route.params.section_name;
    },
    currentSectionName() {
      return sectionNameZh(this.currentSection);
    },
    currentUserName() {
      return Cookies.get(cookieName.username);
    },
    sectionName() {
      return sectionName;
    }
  },
  watch: {
    currentSection() {
      // reset data
      this.resetSectionData();
      // get new data
      this.getTotalCount();
      if (this.postCount)
        this.getPosts(this.currentPage);
    },
    $route() {
      document.title = `${this.currentSectionName} - 泰格论坛`;
    }
  },
  beforeMount() {
    this.getTotalCount();
  }
}
</script>

<template>
  <main class="container pt-3">
    <!-- nav -->
    <nav aria-label="breadcrumb" class="my-2 ps-2">
      <ol class="breadcrumb">
        <li class="breadcrumb-item">
          <RouterLink to="/">首页</RouterLink>
        </li>
        <li class="breadcrumb-item active" aria-current="page" id="current-section">{{ currentSectionName }}</li>
      </ol>
    </nav>
    <!-- posts view -->
    <div class="my-2 p-3 rounded bg-body shadow-sm">
      <div class="d-flex justify-content-between border-bottom pb-2 mb-0">
        <h6 class="">帖子</h6>
        <RouterLink :to="'/post/post-add/' + currentSection" title="添加帖子"
          v-if="currentUserName != undefined && ((currentSection == sectionName.notify && currentUserName == 'root') || currentSection != sectionName.notify)">
          <img src="/img/plus-circle-fill.svg" class="img-fluid w-100" alt="">
        </RouterLink>
      </div>
      <!-- posts summary -->
      <div id="posts">
        <!-- no post -->
        <div v-if="postCount == 0">
          <div class="d-flex text-muted pt-3 my-3">暂无帖子</div>
        </div>
        <!-- has post -->
        <div v-else>
          <div class="border-bottom" v-for="post in posts">
            <el-row>
              <el-col :span="18" :sm="20">
                <RouterLink :to="'/section/' + currentSection + '/' + post.post_id"
                  class="link border-end pt-2 ps-2 pb-2">
                  <el-row>
                    <div class="tw-bold">{{ post.post_title }}</div>
                  </el-row>
                  <el-row><small class="post-summary">{{ post_summary(post) }}</small></el-row>
                  <el-row class="justify-content-end">
                    <p class="pe-sm-2 mb-0 text-nowrap post-summary-time" style="color: darkgrey;">{{
                      displaySqlDatetime(post.post_time) }}</p>
                  </el-row>
                </RouterLink>
              </el-col>
              <el-col :span="3" :sm="2" class="pt-3">
                <RouterLink :to="'/user/user-profiles/' + post.author.user_name" class="link">
                  <el-row class="justify-content-center">
                    <img src="/img/avatar.svg" alt="" class="bd-placeholder-img flex-shrink-0 rounded" width="28"
                      height="28" v-if="!post.author.avatar" />
                    <img :src="getUrl(post.author.avatar)" alt=""
                      class="bd-placeholder-img flex-shrink-0 rounded section-avatar" width="28" height="28" v-else />
                  </el-row>
                  <el-row class="justify-content-center">{{ post.author.user_nickname }}</el-row>
                </RouterLink>
              </el-col>
              <el-col :span="3" :sm="2" class="pt-3">
                <el-row class="justify-content-center">
                  <img src="/img/comment.svg" alt="" class="bd-placeholder-img flex-shrink-0 rounded" width="28"
                    height="28" />
                </el-row>
                <el-row class="justify-content-center">
                  <div id="comment-count" class="text-center" v-if="post.comment_count">{{ post.comment_count }}</div>
                  <div v-else id="comment-count" class="text-center">0</div>
                </el-row>
              </el-col>
            </el-row>
          </div>
        </div>
        <div id="error-message" v-if="error_message != ''" class="alert alert-danger" role="alert">{{ error_message }}
        </div>
      </div>
    </div>
    <!-- pagination -->
    <Pagination :current-page="currentPage" :page-count="totalPage" @on-change-page="getPosts"></Pagination>
  </main>
</template>