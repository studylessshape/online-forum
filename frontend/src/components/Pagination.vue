<script>

export default {
    props: {
        pageCount: Number,
        currentPage: Number,
    },
    emits: {
        /**
         * @param {Number} page 
         */
        onChangePage(page) { },
    },
    data() {
        return {
            front_page_active: '',
            after_page_active: '',
        }
    },
    methods: {
        pageActiveSet() {
            // 设置底部页数跳转按钮的显示
            if (this.currentPage <= 1) {
                this.front_page_active = "disabled";
            } else {
                this.front_page_active = "";
            }

            if (this.currentPage >= this.pageCount) {
                this.after_page_active = "disabled";
            } else {
                this.after_page_active = "";
            }
        },
        changePage(page) {
            this.pageActiveSet();
            // call event
            this.$emit("onChangePage", page);
        },
        frontPage() {
            if (this.currentPage <= 1)
                return;
            this.currentPage--;
            changePage(this.currentPage);
        },
        afterPage() {
            if (this.currentPage >= this.pageCount)
                return;
            this.currentPage++;
            changePage(this.currentPage);
        }

    },
    mounted() {
        this.pageActiveSet();
    }
}

</script>

<template>
    <nav v-if="pageCount > 0">
        <ul class="pagination justify-content-center">
            <li class="page-item"><a href="#" class="page-link" :class="front_page_active" @click="frontPage">上一页</a></li>
            <li class="page-item" v-for="n in pageCount">
                <a href="#" class="page-link disabled" @click="changePage(n)" v-if="n == currentPage">{{ n
                }}</a>
                <a href="#" class="page-link" @click="changePage(n)" v-else>{{ n }}</a>
            </li>
            <li class="page-item"><a href="#" class="page-link" :class="after_page_active" @click="afterPage">下一页</a></li>
        </ul>
    </nav>
</template>