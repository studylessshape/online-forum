<script>
import { isEmail, checkPasswordValid, cookieWatch } from '../components/Utils.vue';
import { ElMessage, ElLoading } from 'element-plus';
import { getUrl, userPath, cookieName } from '../components/ServerConfig.vue';

export default {
    components: {
    },
    data() {
        return {
            codeButtonIcon: "Send",
            codeButtonTitle: "发送验证码",
            codeButtonClass: ['btn', 'btn-outline-primary', 'd-flex', 'flex-wrap', 'align-content-center', 'justify-content-center'],
            isDisabledCodeButton: false,

            registEmail: '',
            emailCode: '',
            isInVerify: false,
            isVerifyCode: false,

            newPassword: '',
            newPasswordAgain: '',
            isInSetPassword: false,
            isSetPassword: false,
        }
    },
    methods: {
        async onEmailCodeButtonClick() {
            if (!isEmail(this.registEmail)) {
                ElMessage.error("输入的邮箱格式不正确！");
                this.$refs.emailInput.focus();
                return;
            }
            // 切换到加载模式，等待服务端回应
            this.switchCodeButtonStatus('loading');
            await fetch(getUrl(userPath.emailCodeGet), {
                method: "POST",
                mode: "cors",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ email: this.registEmail, check_user_rule: 'is-exist' })
            }).then(response => response.json())
                .then(data => {
                    if (data.message) {
                        ElMessage.success(data.message);
                    } else {
                        ElMessage.error(data.responseText);
                    }
                }).catch(err => ElMessage.error(err));
            // 结束，切换状态
            this.switchCodeButtonStatus('wait');
            // 等待一段时间后可再次发送验证码
            setTimeout(() => this.switchCodeButtonStatus('retry'), 30000);
        },
        async onCodeVerify() {
            if (!isEmail(this.registEmail)) {
                ElMessage.error("输入的邮箱格式不正确！");
                this.$refs.emailInput.focus();
                return;
            }
            if (this.emailCode.length != 4) {
                ElMessage.error("验证码应为 4 位！");
                this.$refs.emailCodeInput.focus();
                return;
            }
            // 禁用面板
            this.isInVerify = true;
            this.$refs.emailInput.blur();
            this.$refs.emailCodeInput.blur();
            this.$refs.emilCodeButton.blur();
            // 显示加载动画
            const loadingInstance = ElLoading.service({ target: this.$refs.verifyPanel, text: "正在验证..." });
            await fetch(getUrl(userPath.emailCodeCheck), {
                method: "post",
                mode: "cors",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ email: this.registEmail, code: parseInt(this.emailCode), delete_code: false })
            }).then(response => response.json())
                .then(data => {
                    if (data.success) {
                        ElMessage.success("验证成功！");
                    } else {
                        ElMessage.error(data.responseText);
                    }
                }).catch(err => ElMessage.error(err))
            // 停止加载动画，切换到设置密码表单
            loadingInstance.close();
            this.isVerifyCode = true;
            // 聚焦到密码输入框
            setTimeout(() => { this.$refs.newPasswrodInput.focus(); }, 1000);
        },
        async onSetNewPassword() {
            if (!checkPasswordValid(this.newPassword)) {
                ElMessage.error("密码格式或长度不正确：至少包含大小写字母、数字、特殊字符中的四种中的三种");
                this.$refs.newPasswrodInput.focus();
                return;
            }
            if (this.newPassword != this.newPasswordAgain) {
                ElMessage.error("两次密码不一致！");
                this.$refs.newPasswrodAgainInput.focus();
                return;
            }
            // 禁用面板
            this.isInSetPassword = true;
            this.$refs.newPasswrodInput.blur();
            this.$refs.newPasswrodAgainInput.blur();
            // 显示加载动画
            const loadingInstance = ElLoading.service({ target: this.$refs.setNewPasswordPanel, text: "正在设置密码..." });
            await fetch(getUrl(userPath.updatePassword), {
                method: "POST",
                mode: "cors",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ email: this.registEmail, code: parseInt(this.emailCode), password: this.newPassword })
            }).then(response => response.json())
                .then(data => {
                    if (data.message) {
                        ElMessage.success(data.message);
                        this.isSetPassword = true;
                    } else {
                        ElMessage.error(data.responseText);
                    }
                }).catch(err => ElMessage.error(err));

            if (this.isInSetPassword) {
                loadingInstance.text = "正在登录...";
                await fetch(getUrl(userPath.signIn), {
                    method: "POST",
                    mode: "cors",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ name: this.registEmail, password: this.newPassword })
                }).then(response => response.json())
                    .then(data => {
                        if (data.user) {
                            ElMessage.success("登录成功！");
                            loadingInstance.text = "登录成功！";
                            // 登录成功，通过返回的数据设置 Cookie，并刷新页面
                            cookieWatch.setCookie(cookieName.username, data.user.user_name);
                            cookieWatch.setCookie(cookieName.user_nickname, data.user.user_nickname);
                            cookieWatch.setCookie(cookieName.token, data.user.token);
                            cookieWatch.setCookie(cookieName.avatar, data.user.avatar);
                            loadingInstance.close();
                        } else {
                            ElMessage.error("登录失败，请尝试手动登录");
                            loadingInstance.close();
                        }
                    });
                // this.$router.push('/');
                location.href = `${location.protocol}//${location.host}/`;
            } else {
                loadingInstance.close();
            }
        },
        /**
         * 
         * @param {'normal' | 'loading' | 'retry' | 'wait'} status 
         */
        switchCodeButtonStatus(status) {
            switch (status) {
                case 'normal':
                    this.isDisabledCodeButton = false;
                    this.codeButtonIcon = "Send";
                    this.codeButtonClass[1] = 'btn-outline-primary';
                    this.codeButtonTitle = "发送验证码";
                    break;
                case 'loading':
                    this.isDisabledCodeButton = true;
                    this.codeButtonIcon = 'Loading';
                    this.codeButtonClass[1] = 'btn-outline-secondary'
                    this.codeButtonTitle = '正在发送验证码';
                    break;
                case 'retry':
                    this.isDisabledCodeButton = false;
                    this.codeButtonIcon = 'Retry';
                    this.codeButtonClass[1] = 'btn-outline-danger';
                    this.codeButtonTitle = '重新发送验证码';
                    break;
                case 'wait':
                    this.isDisabledCodeButton = true;
                    this.codeButtonIcon = 'Wait';
                    this.codeButtonClass[1] = 'btn-outline-secondary';
                    this.codeButtonTitle = '短时间内无法再次发送';
                    break;
            }
        }
    },
    computed: {
    },
    mounted() {

    }
}
</script>

<template>
    <div class="floating-tab d-flex main-body">
        <el-main class="floating-tab-content">
            <div ref="verifyPanel" :class="['anim', { 'to-left-fade': isVerifyCode }, 'bg-body', 'px-3', 'py-3', 'rounded', 'shadow']">
                <fieldset :disabled="isVerifyCode || isInVerify">
                    <div class="row justify-content-center">
                        <h3 class="col text-center">忘记密码</h3>
                    </div>
                    <div class="row py-2">
                        <div class="col input-group">
                            <div class="form-floating">
                                <input type="email" class="form-control" id="regist-email" placeholder="请输入注册时的邮箱"
                                    v-model="registEmail" ref="emailInput" @keyup.enter.native="onCodeVerify" />
                                <label for="regist-email">邮箱</label>
                            </div>
                            <button :class="codeButtonClass" :disabled="isDisabledCodeButton" tabindex="-1"
                                @click="onEmailCodeButtonClick" :title="codeButtonTitle" style="width: 60px;">
                                <el-icon :class="[codeButtonIcon == 'Loading' ? 'is-loading' : '']" size="x-large">
                                    <ArrowRight v-if="codeButtonIcon == 'Send'" />
                                    <Loading v-else-if="codeButtonIcon == 'Loading'" />
                                    <RefreshRight v-else-if="codeButtonIcon == 'Retry'" />
                                    <Close v-else-if="codeButtonIcon == 'Wait'" />
                                </el-icon>
                            </button>
                        </div>
                    </div>
                    <div class="row pt-1 pb-2">
                        <div class="col input-group">
                            <input type="text" class="form-control" ref="emailCodeInput" placeholder="请输入接收到的验证码"
                                maxlength="4" @keyup.enter.native="onCodeVerify" v-model="emailCode" />
                        </div>
                    </div>
                    <div class="row justify-content-center pt-2">
                        <button type="button" class="btn btn-primary" style="width: 100px;" @click="onCodeVerify"
                            ref="emilCodeButton">验证</button>
                    </div>
                </fieldset>
            </div>
            <div :class="['anim', 'hidden', { 'show-new-password': isVerifyCode }, 'bg-body', 'px-3', 'py-3', 'rounded', 'shadow']" ref="setNewPasswordPanel">
                <fieldset :disabled="!isVerifyCode || isInSetPassword">
                    <div class="row justify-content-center">
                        <h3 class="col text-center">设置新密码</h3>
                    </div>
                    <div class="row">
                        <el-form label-width="auto" @submit="onSetNewPassword">
                            <el-form-item label="新密码">
                                <el-input type="password" ref="newPasswrodInput" v-model="newPassword" show-password
                                    @keyup.enter.native="onSetNewPassword" />
                            </el-form-item>
                            <el-form-item label="再次输入">
                                <el-input type="password" ref="newPasswrodAgainInput" v-model="newPasswordAgain"
                                    show-password @keyup.enter.native="onSetNewPassword" />
                            </el-form-item>
                            <el-form-item>
                                <button type="button" class="btn btn-primary" style="margin: auto;"
                                    @click="onSetNewPassword" ref="newPasswordButton">确认</button>
                            </el-form-item>
                        </el-form>
                    </div>
                </fieldset>
            </div>
        </el-main>
    </div>
</template>

<style>
input::-webkit-inner-spin-button {
    -webkit-appearance: none !important;

}

input::-webkit-outer-spin-button {
    -webkit-appearance: none !important;

}

fieldset[disabled] {
    -ms-pointer-events: none;
    pointer-events: none;
}

.floating-tab {
    display: flex !important;
    flex-wrap: wrap;
    flex-direction: column;
    align-content: center;
    position: fixed;
    top: 0;
    right: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: white;
    z-index: 1000;
}

.floating-tab-content {
    padding-top: 10em;
}

.d-none {
    display: none;
}

.hidden {
    visibility: hidden;
}

.anim {
    --duration: 0.8s;
    --fill-mode: forwards;
    --timing-function: ease;
    position: relative;
    top: 0;
    left: 0;
}

.to-left-fade {
    animation-name: toLeft;
    animation-duration: var(--duration);
    animation-fill-mode: var(--fill-mode);
    animation-timing-function: var(--timing-function);
}

.show-new-password {
    animation-name: showNewPassword;
    animation-direction: reverse;
    animation-duration: var(--duration);
    animation-fill-mode: var(--fill-mode);
    animation-timing-function: var(--timing-function);
}

@keyframes toLeft {
    from {
        transform: translateX(0);
        opacity: 1;
        visibility: visible;
    }

    to {
        transform: translateX(-100%);
        opacity: 0;
        visibility: hidden;
    }
}

@keyframes showNewPassword {
    from {
        transform: translateX(0) translateY(-100%);
        opacity: 1;
        visibility: visible;
    }

    to {
        transform: translateX(100%) translateY(-100%);
        opacity: 0;
        visibility: hidden;
    }
}

.main-body {
  background-image: linear-gradient(-20deg, #6e45e2 0%, #88d3ce 100%);
}
</style>