<script>
import { sectionName, sectionNameZh, getUrl, postPath } from '../components/ServerConfig.vue';
import { RouterLink } from 'vue-router';
import { ElMessage } from 'element-plus';
import { marked } from 'marked';
import { convert } from 'html-to-text';
import { displayDate } from '../components/Utils.vue';
import { ref } from 'vue';


export default {
    components: {
        RouterLink,
    },
    data() {
        return {
            keywords: this.$route.query.keywords,
            searchCollapseName: '',
            searchPostParts: ['title', 'content'],
            checkedPostParts: ['title', 'content'],
            searchSections: [sectionName.notify, sectionName.techShare, sectionName.techQA, sectionName.chat, sectionName.issue],
            checkedSections: [sectionName.notify, sectionName.techShare, sectionName.techQA, sectionName.chat, sectionName.issue],
            isInSearching: false,
            lastSearchWords: '',
            searchData: [],
            userDataFilter: [],
            postDateTimeFilter: [],
            postSectionFilter: [],
        }
    },
    methods: {
        sectionNameZh,
        displayDate,
        getUrl,
        async onSearch() {
            if (this.keywords == '') {
                ElMessage.warning("搜索内容不能为空！");
                return;
            }
            this.clearFilter();
            this.isInSearching = true;
            this.searchCollapseName = '';
            this.lastSearchWords = this.keywords;
            await fetch(getUrl(postPath.search), {
                method: "POST",
                mode: "cors",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    keywords: this.keywords,
                    title: this.checkedPostParts.find(elem => elem == 'title') != undefined,
                    content: this.checkedPostParts.find(elem => elem == 'content') != undefined,
                    section_name: this.checkedSections,
                })
            }).then(response => response.json())
                .then(data => {
                    if (data.posts) {
                        this.searchData = data.posts.map(item => {
                            item.post_time = new Date(item.post_time);
                            if (item.post_update_time) {
                                item.post_update_time = new Date(item.post_update_time);
                            }
                            return item;
                        });
                        // 创建过滤器，以供筛选
                        this.searchData.forEach(post => {
                            // 用户过滤器
                            if (this.userDataFilter.findIndex(user => user.value == post.author.user_name) == -1)
                                this.userDataFilter.push({ text: post.author.user_nickname, value: post.author.user_name });
                            // 发布日期过滤器
                            if (this.postDateTimeFilter.findIndex(postTime => postTime.value == post.post_time) == -1)
                                this.postDateTimeFilter.push({ text: displayDate(post.post_time), value: post.post_time });
                            // 论坛板块过滤器
                            if (this.postSectionFilter.findIndex(section => section.value == post.section_name) == -1)
                                this.postSectionFilter.push({ text: post.section_name_zh, value: post.section_name });
                        });
                    } else {
                        ElMessage.error(data.responseText);
                    }
                });

            this.isInSearching = false;
            document.title = `\"${this.keywords}\" 搜索结果 - 泰格论坛`;
        },
        postSummary(postSummary) {
            if (postSummary == null || postSummary == 0)
                return '';
            return convert(marked.parse(postSummary));
        },
        filterHandler(value, row, col) {
            const property = col['property'];
            return row[property] == value;
        },
        filterUserHandler(value, row) {
            return row.author.user_name == value;
        },
        clearFilter() {
            this.$refs.searchResultTable.clearFilter();
        },
    },
    created() {
        this.keywords = this.$route.query.keywords;
        const path = this.$route.path;
        this.$router.push(path);
    },
    mounted() {
        if (this.keywords != undefined && this.keywords != '') {
            this.onSearch();
        }
        document.title = `\"${this.keywords}\" 搜索结果 - 泰格论坛`;
    }
}
</script>

<template>
    <div class="container pt-4 px-md-5">
        <div>
            <el-input v-model="keywords" :disabled="isInSearching" clearable prefix-icon="Search" size="large"
                @keyup.enter.native="onSearch" name="search-field" autocomplete='off' />
        </div>
        <div>
            <el-collapse v-model="searchCollapseName">
                <el-collapse-item :disabled="isInSearching" name="searchOptions">
                    <template #title>
                        <strong>搜索条件选择</strong>
                    </template>
                    <div>
                        <div>搜索部分选择：</div>
                        <el-checkbox-group v-model="checkedPostParts" :min="1">
                            <el-checkbox v-for="part in searchPostParts" :label="part" :key="part" border>
                                {{ part == 'title' ? '帖子标题' : '帖子内容' }}
                            </el-checkbox>
                        </el-checkbox-group>
                    </div>
                    <div>
                        <div>论坛板块选择：</div>
                        <el-checkbox-group v-model="checkedSections" :min="1">
                            <el-checkbox v-for="section in searchSections" :label="section" :key="section" border>
                                {{ sectionNameZh(section) }}
                            </el-checkbox>
                        </el-checkbox-group>
                    </div>
                </el-collapse-item>
            </el-collapse>
        </div>
        <div class="mt-3">
            <div class="pb-1">搜索结果：</div>
            <div>
                <el-button @click="clearFilter" plain>清除筛选项</el-button>
            </div>
            <el-table :data="searchData" max-height="360" stripe border v-loading="isInSearching" ref="searchResultTable">
                <el-table-column label="标题" prop="post_title" width="200" sortable>
                    <template #default="scope">
                        <RouterLink :to="`/section/${scope.row.section_name}/${scope.row.post_id}`" class="time-display">
                            <el-link type="primary">{{ scope.row.post_title }}</el-link>
                        </RouterLink>
                    </template>
                </el-table-column>
                <el-table-column prop="author" label="用户" width="150" sortable :filters="userDataFilter"
                    :filter-method="filterUserHandler" column-key="user">
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
                <el-table-column prop="section_name" label="发布板块" width="120" sortable :filters="postSectionFilter"
                    column-key="section" :filter-method="filterHandler">
                    <template #default="scope">
                        <RouterLink :to="`/section/${scope.row.section_name}`" class="time-display">
                            <el-link type="primary">{{ scope.row.section_name_zh }}</el-link>
                        </RouterLink>
                    </template>
                </el-table-column>
                <el-table-column prop="post_time" label="发布时间" width="210" sortable :filters="postDateTimeFilter"
                    column-key="post-time" :filter-method="filterHandler">
                    <template #default="scope">
                        <div class="time-display">{{ displayDate(scope.row.post_time) }}</div>
                    </template>
                </el-table-column>
                <el-table-column prop="post_update_time" label="修改时间" width="210" sortable column-key="post-update-time">
                    <template #default="scope">
                        <div class="time-display">
                            {{
                                scope.row.post_update_time
                                ? displayDate(scope.row.post_update_time)
                                : ''
                            }}
                        </div>
                    </template>
                </el-table-column>
            </el-table>
        </div>
    </div>
</template>