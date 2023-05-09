<script>
import MdEditor from "md-editor-v3";
import "md-editor-v3/lib/style.css";
import { setValid, removeValid } from "../components/Utils.vue";
import { postPath, getUrl, filePath } from "../components/ServerConfig.vue";
import Cookies from "js-cookie";
import { cookieName } from "../components/ServerConfig.vue";
import { validFileType, uploadFile, imageFileTypes } from "../components/Utils.vue";

export default {
  components: {
    MdEditor,
  },
  data() {
    return {
      loading: true,
      postId: parseInt(this.$route.params.post_id),
      postTitle: "",
      postTitleFeedback: "标题太长或太短（长度应大于4小于50）！",
      postContent: "",
      postContentFeedback: "正文内容太少！",
      toolbarsExclude: [
        "mermaid",
        "katex",
        "pageFullscreen",
        "fullscreen",
        "github",
        "htmlPreview",
      ],
    }
  },
  methods: {
    uploadPost() {
      if (this.postTitle.length < 4 || this.postTitle.length > 50) {
        setValid("post-title-input", false);
        return;
      } else {
        removeValid("post-title-input");
      }

      if (this.postContent.length < 4) {
        setValid("post-content-input", false);
        return;
      } else {
        removeValid("post-content-input");
      }

      this.loading = true;

      var edit_data = JSON.stringify({
        post_id: this.editMode == "add" ? null : this.postId,
        post_title: this.postTitle,
        post_content: this.postContent,
        section_name: this.currentSection,
        token: Cookies.get(cookieName.token),
      });

      fetch(getUrl(postPath.editPost), {
        method: "POST",
        mode: "cors",
        headers: {
          "Content-Type": "application/json",
        },
        body: edit_data,
      })
        .then((response) => {
          return response.json();
        })
        .then((data) => {
          if (data.post_id) {
            this.loading = false;
            this.$router.push(
              "/section/" + this.currentSection + "/" + data.post_id
            );
          } else {
            console.log(data.responseText);
          }
        })
        .catch((err) => {
          console.log(err);
        });
    },
    async uploadImg(files, callback) {
      if (!files.length) {
        alert("没有选择图片");
        return;
      }
      var imgSrc = new Array();
      for (var i = 0; i < files.length; i++) {
        if (!validFileType(files[i], imageFileTypes)) {
          continue;
        }
        var url = getUrl(filePath.uploadImage) + '/' + files[i].name;
        var formData = new FormData();
        formData.append("file", files[i]);
        await uploadFile(formData, url)
          .then((response) => response.json())
          .then((data) => {
            if (data.src) {
              imgSrc.push(getUrl(data.src));
            } else {
              console.log(data.responseText);
            }
          })
          .catch((err) => {
            console.log(err);
          });
      }
      callback(imgSrc);
    },
    getPost() {
      this.loading = true;

      fetch(getUrl(postPath.getPost) + "?post_id=" + this.postId, {
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
            this.loading = false;
            this.postTitle = data.post.post_title;
            this.postContent = data.post.post_content;
          } else {
            console.log(data.responseText);
          }
        })
        .catch((err) => {
          console.log(err);
        });
    },
  },
  watch: {
    postTitle(newTitle) {
      if (newTitle.length < 4 || newTitle.length > 50) {
        this.postTitleFeedback = "标题太长或太短（长度应大于4小于50）！";
        setValid("post-title-input", false);
      } else {
        removeValid("post-title-input");
      }
    },
    postContent(newContent) {
      if (newContent.length < 4) {
        this.postContentFeedback = "正文内容太少！";
        setValid("post-content-input", false);
      } else {
        removeValid("post-content-input");
      }
    },
  },
  computed: {
    currentSection() {
      return this.$route.params.section_name;
    },
    editMode() {
      if (this.postId) {
        return "modify";
      } else {
        return "add";
      }
    },
  },
  beforeMount() {
    if (!Cookies.get(cookieName.token)) {
      alert("您还未登录！");
      this.$router.push("/");
    }
    if (this.editMode == "modify") {
      this.getPost();
    } else {
      this.loading = false;
    }
  },
};
</script>

<template>
  <div class="flex-column mt-3">
    <el-container v-loading="loading">
      <el-main class="p-5 m-5 mt-3 rounded shadow bg-body">
        <div>
          <h4 class="text-center">
            帖子{{ editMode == "add" ? "添加" : "编辑" }}
          </h4>
        </div>
        <div class="input-group my-3">
          <span class="input-group-text" id="post-title-label">帖子标题</span>
          <input type="text" class="form-control rounded-end" placeholder="标题" aria-label="标题"
            aria-describedby="post-title-label" v-model="postTitle" id="post-title-input" />
          <div class="invalid-feedback">{{ postTitleFeedback }}</div>
        </div>
        <div>
          <h5 class="text-center">正文编辑</h5>
        </div>
        <div>
          <MdEditor id="post-content-input" v-model="postContent" :toolbars-exclude="toolbarsExclude" :preview="true"
            @on-upload-img="uploadImg" placeholder="正文编辑">
          </MdEditor>
          <div class="invalid-feedback">{{ postContentFeedback }}</div>
        </div>
        <!-- 提交按钮 -->
        <div class="d-flex justify-content-center mt-3">
          <button type="button" title="提交" class="btn btn-outline-primary w-100" style="max-width: 600px;" @click="uploadPost">
            提交
          </button>
        </div>
      </el-main>
    </el-container>
  </div>
</template>
