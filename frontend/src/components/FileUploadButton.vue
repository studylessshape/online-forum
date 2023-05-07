<script>
import { getUrl, filePath } from './ServerConfig.vue';

export default {
    props: {
        fileType: [String],
        title: String,
    },
    emits: ["after-upload"],
    data() {
        return {
        }
    },
    methods: {
        validFileType(file) {
            // this.fileTypes = this.fileType.split(' ');
            if (this.fileType.length == 0)
                return true;
            return this.fileType.includes(file.type);
        },
        uploadClick() {
            var file_sel = document.getElementById("file-sel");
            file_sel.addEventListener('change', this.uploadToServer);
            file_sel.click();
        },
        uploadToServer() {
            var files = document.getElementById("file-sel").files;
            if (!files.length) {
                alert("没有选择图片");
                return;
            }
            for (var i = 0; i < files.length; i++) {
                if (!this.validFileType(files[i])) {
                    continue;
                }
                var form_data = new FormData();
                form_data.append("file", files[i]);
                fetch(getUrl(filePath.uploadFiles + "/" + files[i].name), {
                    method: 'POST',
                    mode: 'cors',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: form_data
                })
                    .then(response => response.json())
                    .then(data => {
                        console.log(data);
                        if (data.src)
                            this.$emit("after-upload", data.src);
                    })
                    .catch(error => {
                        console.log(error);
                    })
            }
        }
    }
}
</script>

<template>
    <button type="button" :title="title" tabindex="-1" class="p-2 btn btn-outline-light fs-6 border-0 text-black lh-1"
        @click="uploadClick">
        <slot></slot>
        <input type="file" id="file-sel" accept=".jpg, .jpeg, .png, .svg, .webp" style="display:none" @onchange="uploadToServer">
    </button>
</template>