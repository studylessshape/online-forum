<script>
</script>

<template>
    <div class="d-flex justify-content-center">
        <el-result icon="error" title="请求的页面不存在！" sub-title="Code：404" class="bg-body w-50 rounded mt-3 shadow">
            <template #extra>
                <el-button type="primary" @click="$router.go(-1)">返回</el-button>
            </template>
        </el-result>
    </div>
</template>