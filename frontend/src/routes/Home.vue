<script>
import { RouterLink } from "vue-router";
import { getSectionRoute, sectionName, getUrl, sectionPath } from "../components/ServerConfig.vue";
import { displaySqlDatetime } from "../components/Utils.vue"
import MdEditor from 'md-editor-v3'
import 'md-editor-v3/lib/style.css';

class SectionStatus {
  constructor(sectionName) {
    this.sectionName = sectionName;
    this.todayPostCount = '';
    this.totalPostCount = '';
    this.lastPostDateTime = '';
  }
}

export default {
  components: {
    RouterLink,
    MdEditor,
  },
  data() {
    return {
      notify_content: null,
      sectionName,
      // status of section
      techShareStatus: new SectionStatus(sectionName.techShare),
      techQAStatus: new SectionStatus(sectionName.techQA),
      chatStatus: new SectionStatus(sectionName.chat),
      issueStatus: new SectionStatus(sectionName.issue),
    };
  },
  methods: {
    getSectionRoute,
    /**
     * 
     * @param {{last_update_time: String, today_post_count: Number,total_post_count:Number}} value 
     * @param {SectionStatus} target 
     */
    updateStatus(value, target) {
      if (value.last_update_time) {
        target.lastPostDateTime = displaySqlDatetime(value.last_update_time);
      }
      target.todayPostCount = value.today_post_count;
      target.totalPostCount = value.total_post_count;
    },
    getSectionStatus() {
      fetch(getUrl(sectionPath.getStatus), {
        method: "post",
        mode: "cors",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          names: [sectionName.techShare, sectionName.techQA, sectionName.chat, sectionName.issue]
        })
      }).then(response => {
        return response.json();
      })
        .then(data => {
          if (data.statuses) {
            data.statuses.forEach(element => {
              switch (element.section_name) {
                case sectionName.techShare:
                  this.updateStatus(element, this.techShareStatus);
                  break;
                case sectionName.techQA:
                  this.updateStatus(element, this.techQAStatus);
                  break;
                case sectionName.chat:
                  this.updateStatus(element, this.chatStatus);
                  break;
                case sectionName.issue:
                  this.updateStatus(element, this.issueStatus);
                  break;
              }
            });
          }
        })
        .catch(err => {
          console.log(err);
        })
    },
    getNotifyLastest() {
      fetch(getUrl(sectionPath.getLastestPost) + "?section_name=notify", {
        method: "get",
        mode: "cors",
      }).then(response => {
        return response.json()
      }).then(data=>{
        if (data.post) {
          this.notify_content = data.post.post_content;
        } else {
          console.log(data.responseText);
        }
      }).catch(err=> {
        console.log(err);
      })
    }
  },
  computed: {

  },
  beforeMount() {
    // get section status
    this.getSectionStatus();
    // get lastest post
    this.getNotifyLastest();
  },
};
</script>

<template>
  <main class="container pt-0">
    <div class="d-flex align-items-center p-3 my-3 bg-light rounded shadow-sm bg-brand">
      <div class="lh-1">
        <h1 class="h6 fw-bold mb-0 lh-1">泰格论坛</h1>
        <small>自由技术分享论坛</small>
      </div>
    </div>
    <div class="my-3 p-3 bg-body rounded shadow-sm">
      <h6 class="border-bottom pb-2 mb-0">最新通告</h6>
      <div class="d-flex text-muted pt-3 flex-column" id="lastest-notify">
        <!-- 如果没有获取到则显示暂无通告 -->
        <div class="pb-3 mb-0 small lh-sm border-bottom" v-if="!notify_content">
          <strong class="d-block text-gray-dark">暂无通告</strong>
        </div>
        <!-- 获取成功则显示内容 -->
        <div class="pb-3 mb-0 pt-1 small lh-sm border-bottom" v-else>
          <MdEditor v-model="notify_content" preview-only></MdEditor>
        </div>
      </div>
      <small class="d-block text-end mt-3">
        <RouterLink :to="getSectionRoute(sectionName.notify)">查看全部通告</RouterLink>
      </small>
    </div>
    <div class="my-3 p-3 bg-body rounded shadow-sm">
      <h6 class="border-bottom pb-2 mb-0">论坛板块</h6>
      <div class="d-flex text-muted pt-3">
        <table class="table table-hover table-bordered">
          <thead>
            <tr>
              <th scope="col"></th>
              <th scope="col">最新发布时间</th>
              <th scope="col">今日发帖量</th>
              <th scope="col">总发帖量</th>
            </tr>
          </thead>
          <tbody class="table-group-divider">
            <tr>
              <td>
                <RouterLink :to="getSectionRoute(sectionName.techShare)" class="link-normal">技术分享</RouterLink>
              </td>
              <td>
                <div>{{ techShareStatus.lastPostDateTime }}</div>
              </td>
              <td>
                <div>{{ techShareStatus.todayPostCount }}</div>
              </td>
              <td>
                <div>{{ techShareStatus.totalPostCount }}</div>
              </td>
            </tr>
            <tr>
              <td>
                <RouterLink :to="getSectionRoute(sectionName.techQA)" class="link-normal">技术问答</RouterLink>
              </td>
              <td>
                <div>{{ techQAStatus.lastPostDateTime }}</div>
              </td>
              <td>
                <div>{{ techQAStatus.todayPostCount }}</div>
              </td>
              <td>
                <div>{{ techQAStatus.totalPostCount }}</div>
              </td>
            </tr>
            <tr>
              <td>
                <RouterLink :to="getSectionRoute(sectionName.chat)" class="link-normal">灌水闲聊</RouterLink>
              </td>
              <td>
                <div>{{ chatStatus.lastPostDateTime }}</div>
              </td>
              <td>
                <div>{{ chatStatus.todayPostCount }}</div>
              </td>
              <td>
                <div>{{ chatStatus.totalPostCount }}</div>
              </td>
            </tr>
            <tr>
              <td>
                <RouterLink :to="getSectionRoute(sectionName.issue)" class="link-normal">论坛反馈</RouterLink>
              </td>
              <td>
                <div>{{ issueStatus.lastPostDateTime }}</div>
              </td>
              <td>
                <div>{{ issueStatus.todayPostCount }}</div>
              </td>
              <td>
                <div>{{ issueStatus.totalPostCount }}</div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </main>
</template>
