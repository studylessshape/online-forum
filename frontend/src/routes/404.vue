<script>
</script>

<template>
    <el-result icon="error" title="请求的页面不存在！" sub-title="Code：404">
        <template #extra>
            <el-button type="primary" @click="$router.go(-1)">返回</el-button>
        </template>
    </el-result>
</template>