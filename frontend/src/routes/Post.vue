<script>
import { sectionNameZh, cookieName } from "../components/ServerConfig.vue";
import { RouterLink } from "vue-router";
import { getUrl, postPath } from "../components/ServerConfig.vue";
import { displaySqlDatetime } from "../components/Utils.vue";
import Cookies from "js-cookie";
import MdEditor from "md-editor-v3";
import "md-editor-v3/lib/style.css";
import Pagination from "../components/Pagination.vue";
import { marked } from 'marked';
import { ElMessage } from 'element-plus';

export default {
  components: {
    RouterLink,
    MdEditor,
    Pagination,
  },
  data() {
    return {
      post: null,
      isLoaded: false,
      isUser: false,
      elLoading: false,
      // about comment
      perPageCount: 10,
      currentPage: 1,
      totalCommentCount: 0,
      comments: [],
      commentEditorToolbars: [
        "bold",
        "underline",
        "italic",
        "-",
        "title",
        "strikeThrough",
        "sub",
        "sup",
        "quote",
        "unorderedList",
        "orderedList",
        "task",
        "-",
        "codeRow",
        "code",
        "link",
        "image",
        "table",
        "-",
        "revoke",
        "next",
      ],
      commentConent: "",
      replyTo: null,
      showDeleteCommentDialog: false,
      deleteCommentId: null,
      inDelete: false,
      inPublishComment: false,
    };
  },
  methods: {
    getUrl,
    async getPost() {
      this.elLoading = true;
      await fetch(getUrl(postPath.getPost) + "?post_id=" + this.currentPostId, {
        method: "get",
        mode: "cors",
        headers: {
          "Content-Type": "application/json",
        },
      })
        .then((response) => {
          return response.json();
        })
        .then((data) => {
          if (data.post) {
            this.post = data.post;
            this.isLoaded = true;
            document.title = `${data.post.post_title} - 泰格论坛`;
          } else {
            ElMessage.error(data.responseText);
          }
        })
        .catch((err) => {
          ElMessage.error(err);
        });
      this.elLoading = false;
    },
    deletePost() {
      $("#post-del-modal").modal("hide");
      this.elLoading = true;
      fetch(getUrl(postPath.delPost) + "?post_id=" + this.currentPostId, {
        method: "get",
        mode: "cors",
        credentials: "include",
        headers: {
          "Content-Type": "application/json",
        },
      })
        .then((response) => {
          return response.json();
        })
        .then((data) => {
          this.elLoading = false;
          if (data.success) {
            this.$router.push(this.currentSectionPath);
          } else {
            console.log(data.responseText);
          }
        })
        .catch((err) => {
          console.log(err);
        });
    },
    displaySqlDatetime,
    /**
     * 
     * @param {Number} page 
     */
    getComment(page) {
      fetch(getUrl(postPath.getComments), {
        method: "POST",
        mode: "cors",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          post_id: this.currentPostId,
          page: page,
          per_count: this.perPageCount,
        }),
      })
        .then((response) => {
          return response.json();
        })
        .then((data) => {
          if (data.comments) {
            this.comments = data.comments;
          } else {
            console.log(data.responseText);
          }
        })
        .catch((err) => console.log(err));
    },
    /**
     * 
     * @param {String} content 
     */
    getHtml(content) {
      return marked.parse(content);
    },
    async publishComment() {
      this.inPublishComment = true;
      await fetch(getUrl(postPath.putComment), {
        method: "POST",
        mode: "cors",
        credentials: "include",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          post_id: this.currentPostId,
          token: Cookies.get(cookieName.token),
          comment_content: this.commentConent,
          reply_to: this.replyTo,
        })
      }).then(response => {
        return response.json();
      }).then(data => {
        if (data.success) {
          location.reload();
        } else {
          console.log(data.responseText);
        }
      }).catch(err => console.log(err));
      this.inPublishComment = false;
    },
    isCurUserComment(user_name) {
      return user_name == Cookies.get(cookieName.username);
    },
    openDeleteCommentDialog(comment_id) {
      this.deleteCommentId = comment_id;
      this.showDeleteCommentDialog = true;
    },
    deleteComment() {
      // 改变按钮状态
      this.inDelete = true;

      fetch(getUrl(postPath.deleteComment), {
        method: "post",
        mode: "cors",
        credentials: "include",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          comment_id: parseInt(this.deleteCommentId),
          token: Cookies.get(cookieName.token),
        })
      }).then(response => {
        this.inDelete = false;
        return response.json();
      }).then(data => {
        if (data.success) {
          location.reload();
        } else {
          console.log(data.responseText);
        }
      }).catch(err => console.log(err));
    }
  },
  computed: {
    currentPostId() {
      return parseInt(this.$route.params.post_id);
    },
    currentSection() {
      return this.$route.params.section_name;
    },
    currentSectionPath() {
      return "/section/" + this.currentSection;
    },
    currentSectionName() {
      return sectionNameZh(this.currentSection);
    },
    isCurUserPosted() {
      return this.post.author.user_name == Cookies.get(cookieName.username);
    },
    canComment() {
      var token = Cookies.get(cookieName.token);

      return token != null && token != undefined;
    },
  },
  mounted() {
    this.getPost();
    this.getComment(this.currentPage);
  },
};
</script>

<template>
  <el-main class="container pt-3" v-loading="elLoading" element-loading-background="rgba(122, 122, 122, 0.8)">
    <div v-if="isLoaded">
      <nav aria-label="breadCrumb" class="my-2 ps-2">
        <ol class="breadcrumb">
          <li class="breadcrumb-item">
            <RouterLink to="/">首页</RouterLink>
          </li>
          <li class="breadcrumb-item">
            <RouterLink :to="currentSectionPath">{{
              currentSectionName
            }}</RouterLink>
          </li>
          <li class="breadcrumb-item active">{{ post.post_title }}</li>
        </ol>
      </nav>
      <!-- 标题 -->
      <div class="d-flex align-items-center p-3 my-3 bg-light rounded shadow-sm justify-content-between">
        <div class="lh-1">
          <h1 class="h6 fw-bold mb-0 lh-1">{{ post.post_title }}</h1>
        </div>
        <div class="lh-1">
          <h1 class="h6 mb-0 lh-1">{{ displaySqlDatetime(post.post_time) }}</h1>
        </div>
      </div>
      <!-- 帖子正文和评论 -->
      <div>
        <!-- 帖子正文 -->
        <div class="my-3 p-3 bg-body rounded shadow-sm">
          <div class="d-flex border-bottom pb-2 mb-0 justify-content-between">
            <h6>正文</h6>
          </div>
          <div class="row">
            <div class="col-lg-2">
              <div class="w-100 container">
                <RouterLink :to="'/user/user-profiles/' + post.author.user_name" class="link">
                  <span class="row pt-3 justify-content-center">
                    <img :src="[
                      post.author.avatar
                        ? getUrl(post.author.avatar)
                        : '/img/avatar.svg',
                    ]" alt="" class="post-avatar" />
                  </span>
                  <span class="row justify-content-lg-center border-bottom pb-3">
                    <span class="col text-center">{{
                      post.author.user_nickname
                    }}</span>
                  </span>
                </RouterLink>
              </div>
            </div>
            <div class="col-lg-10 ps-2 border-start pb-3" id="post-content">
              <MdEditor v-model="post.post_content" preview-only id="post-content"></MdEditor>
            </div>
          </div>
          <div class="border-top pt-2 row justify-content-end">
            <div class="d-flex col" v-if="isCurUserPosted">
              <RouterLink type="button" class="btn btn-light p-0" title="编辑帖子" style="display: flex; align-items: center;"
                :to="'/post/post-edit/' + currentSection + '/' + currentPostId">
                <img src="/img/edit.svg" alt="" srcset="" class="wh-2" />
              </RouterLink>
              <button type="button" class="btn btn-light p-0" title="删除帖子" data-bs-target="#post-del-modal"
                data-bs-toggle="modal">
                <img src="/img/delete.svg" alt="" srcset="" class="wh-2" />
              </button>
              <div id="post-del-modal" class="modal fade" tabindex="-1" data-bs-backdrop="static" data-bs-keyboard="false"
                aria-hidden="true" aria-labelledby="post-del-title">
                <div class="modal-dialog modal-dialog-centered modal-sm">
                  <div class="modal-content">
                    <div class="modal-header">
                      <h1 class="modal-title fs-5 text-center" id="post-del-title">
                        确认删除：
                      </h1>
                      <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"
                        id="post-del-close"></button>
                    </div>
                    <div class="modal-body">
                      删除之后无法恢复，是否确认删除帖子？
                    </div>
                    <div class="modal-footer">
                      <button type="button" class="btn btn-outline-primary" data-bs-dismiss="modal" aria-label="Close"
                        id="post-del-cancel">
                        取消
                      </button>
                      <button class="btn btn-outline-danger" @click="deletePost" id="post-del-button">
                        确定
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <div class="col text-end text-darkgray">
              {{
                post.post_update_time
                ? displaySqlDatetime(post.post_update_time)
                : displaySqlDatetime(post.post_time)
              }}
            </div>
          </div>
        </div>
        <!-- 帖子评论 -->
        <div :key="comments">
          <div v-if="comments.length > 0" v-for="comment in comments">
            <el-row class="border border-2 rounded pt-3 ps-2 pb-2 pe-3 mb-2">
              <el-col :md="2" class="border-bottom border-2">
                <RouterLink :to="'/user/user-profiles/' + comment.user.user_name" class="link">
                  <el-row class="justify-content-center"><img
                      :src="[comment.user.avatar ? getUrl(comment.user.avatar) : '/img/avatar.svg']" alt=""
                      class="wh-2"></el-row>
                  <el-row class="justify-content-center">
                    <div class="text-center">{{ comment.user.user_nickname }}</div>
                  </el-row>
                </RouterLink>
              </el-col>
              <el-col :md="22" class="border-start ps-2">
                <el-row>
                  <div class="" v-html="getHtml(comment.comment_content)"></div>
                </el-row>
                <el-row class="justify-content-end">
                  <el-col :md="1" v-if="isCurUserComment(comment.user.user_name)">
                    <div style="display: flex; justify-content: end; width: 100%;">
                      <button type="button" class="btn btn-light p-0" title="删除评论"
                        @click="openDeleteCommentDialog(comment.comment_id)">
                        <img src="/img/delete.svg" alt="" srcset="" style="width: 24px;" />
                      </button>
                    </div>
                    <!-- 删除评论的 dialog -->
                    <el-dialog title="确认" v-model="showDeleteCommentDialog" :show-close="false"
                      :close-on-click-modal="false" align-center>
                      <span>是否删除？</span>
                      <template #footer>
                        <span class="dialog-footer">
                          <el-button @click="showDeleteCommentDialog = false" :disabled="inDelete">取消</el-button>
                          <el-button type="danger" @click="deleteComment" :loading="inDelete">
                            确认
                          </el-button>
                        </span>
                      </template>
                    </el-dialog>
                  </el-col>
                  <el-col :md="7">
                    <div class="text-end text-darkgray time-display">{{ displaySqlDatetime(comment.comment_time) }}</div>
                  </el-col>
                </el-row>
              </el-col>
            </el-row>
          </div>
          <div v-else>
            <div class="alert alert-secondary" role="alert">暂时没有评论</div>
          </div>
        </div>
        <!-- 帖子翻页 -->
        <Pagination :current-page="currentPage" :page-count="totalCommentCount" @on-change-page="getComment">
        </Pagination>
        <!-- 发布评论区 -->
        <el-main v-if="canComment" class="d-flex justify-content-end flex-wrap" id="comment-editor"
          v-loading="inPublishComment">
          <MdEditor v-model="commentConent" :toolbars="commentEditorToolbars" :preview="false" :html-preview="false"
            placeholder="输入评论内容" style="height: 200px" :footers="['markdownTotal']" id="comment-editor">
          </MdEditor>
          <el-button type="primary" plain @click="publishComment">发布评论</el-button>
        </el-main>
      </div>
    </div>
    <div v-else-if="!isLoaded && !elLoading">
      <el-result icon="error" title="帖子可能被删除" sub-title="Code：404">
        <template #extra>
          <el-button type="primary" @click="$router.back()">返回</el-button>
        </template>
      </el-result>
    </div>
  </el-main>
</template>
