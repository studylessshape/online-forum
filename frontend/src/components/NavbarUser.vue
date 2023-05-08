<script>
import { getUrl, userPath, cookieName } from './ServerConfig.vue'
import { isEmail } from './Utils.vue'
import FileUploadButton from './FileUploadButton.vue';
import { cookieWatch } from './Utils.vue';

export default {
    components: {
        FileUploadButton,
    },
    data() {
        return {
            // sign in
            sign_in_name_feedback: '',
            sign_in_pwd_feedback: '',
            // sign up
            sign_up_email_feedback: '请输入正确的邮箱',
            sign_up_code_feedback: '请输入正确的验证码',
            sign_up_username_feedback: '用户名不可用',
            sign_up_password_feedback: '密码格式不正确：1.长度大于8；2.至少包含大小写字母、数字、特殊字符中的三种',
            sign_up_again_password_feedback: '两次密码不一致',
            sign_up_all_feedback: '',
            // sign up info
            sign_up_email: '',
            sign_up_code: '',
            sign_up_username: '',
            sign_up_password: '',
            sign_up_again_password: '',
            // sign up set info
            sign_up_avatar: '',
            sign_up_nickname: '',
            sign_up_nickname_feedback: '昵称已被使用'
        }
    },
    methods: {
        check_sign_name(id, setFeedbackMessage, message) {
            if (String($("#" + id).val()).length < 4) {
                $("#" + id).addClass("is-invalid");
                setFeedbackMessage(message);
                return false;
            } else {
                $("#" + id).removeClass("is-invalid");
                return true;
            }
        },
        check_password(id, setFeedbackMessage, message) {
            if (String($("#" + id).val()).length < 8) {
                $("#" + id).addClass("is-invalid");
                setFeedbackMessage(message);
                return false;
            } else {
                $("#" + id).removeClass("is-invalid");
                return true;
            }
        },
        set_valid(i_id, is_valid) {
            var id = "#" + i_id;
            if (is_valid) {
                $(id).removeClass("is-invalid");
                $(id).addClass("is-valid");
            } else {
                $(id).removeClass("is-valid");
                $(id).addClass("is-invalid");
            }
        },
        remove_valid(i_id) {
            var id = "#" + i_id;
            $(id).removeClass("is-invalid");
            $(id).removeClass("is-valid");
        },
        sign_in() {
            // 检查输入
            this.check_sign_name("sign-in-email-input", msg => { this.sign_in_name_feedback = msg }, "登录名长度太短");
            if (!this.check_password("sign-in-password-input", msg => { this.sign_in_pwd_feedback = msg }, "密码太短")) {
                return;
            }

            // 点击登录按钮时，禁止再次点击登录按钮，并且禁止点击 Modal 框的关闭按钮
            $("#sign-in-button").empty().attr("disabled", "disabled");
            $("#sign-in-close").attr("disabled", "disabled");

            // 将登录按钮切换成加载动画
            $("#sign-in-button").append(
                '<span class="spinner-border spinner-border-sm"></span>'
            );

            // 隐藏注册按钮
            $("#sign-in-to-up").hide();

            // 获取输入
            var name = String($("#sign-in-email-input").val());
            var pwd = String($("#sign-in-password-input").val());

            var recoveryFn = () => {
                // 恢复按钮
                $("#sign-in-button").empty().removeAttr("disabled");
                $("#sign-in-close").removeAttr("disabled");

                // 将登录按钮切换成加载动画
                $("#sign-in-button").append("登录");

                // 显示注册按钮
                $("#sign-in-to-up").show();
            };

            // 发送数据给后端
            fetch(getUrl(userPath.signIn), {
                method: 'POST',
                mode: 'cors',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ name: name, password: pwd })
            }).then(response => response.json())
                .then(data => {
                    if (data.user) {
                        // 登录成功，通过返回的数据设置 Cookie，并刷新页面
                        cookieWatch.setCookie(cookieName.username, data.user.user_name);
                        cookieWatch.setCookie(cookieName.user_nickname, data.user.user_nickname);
                        cookieWatch.setCookie(cookieName.token, data.user.token);
                        cookieWatch.setCookie(cookieName.avatar, data.user.avatar);
                        location.reload();
                    } else {
                        $("#sign-in-panel").addClass("is-invalid");
                    }
                    recoveryFn();
                }
                ).catch(error => {
                    recoveryFn();
                    console.log(error);
                })
        },
        check_password_valid(password) {
            if (password.length < 8) {
                return false;
            }
            var count = 0;
            var low_word_re = new RegExp("[a-z]");
            var big_word_re = new RegExp("[A-Z]");
            var number_re = new RegExp("[0-9]");
            var special_re = new RegExp("[~`!@#$%^&*(_)+.=?,;:'\"/]");
            if (low_word_re.exec(password)) {
                count++;
            }
            if (big_word_re.exec(password)) {
                count++;
            }
            if (number_re.exec(password)) {
                count++;
            }
            if (special_re.exec(password)) {
                count++;
            }
            if (count < 3) {
                return false;
            }
            return true;
        },
        sign_up() {
            // 检查注册的所有项目
            if (!isEmail(this.sign_up_email)
                || (this.sign_up_code.length != 4)
                || !this.check_password_valid(this.sign_up_password)
                || (this.sign_up_password != this.sign_up_again_password)
                || this.sign_up_username > 20 || this.sign_up_username < 4) {
                return;
            }

            // 隐藏返回登录按钮
            $("#sign-up-to-in").hide();

            // 点击注册按钮时，禁止再次点击注册按钮，并且禁止点击 Modal 框的关闭按钮
            $("#sign-up-button").empty().attr("disabled", "disabled");
            $("#sign-up-close").attr("disabled", "disabled");

            // 将注册按钮切换成加载动画
            $("#sign-up-button").append(
                '<span class="spinner-border spinner-border-sm"></span>'
            );

            var recoveryFn = () => {
                // 恢复按钮
                $("#sign-up-button").empty().removeAttr("disabled");
                $("#sign-up-close").removeAttr("disabled");

                // 将注册按钮切换成加载动画
                $("#sign-up-button").append("注册");

                // 显示返回按钮
                $("#sign-up-to-in").show();
            }

            var send_data = JSON.stringify({
                user_name: this.sign_up_username,
                email: this.sign_up_email,
                code: parseInt(this.sign_up_code),
                password: this.sign_up_password
            });

            this.reset_sign_up_valid();

            // 上传注册数据
            fetch(getUrl(userPath.signUp), {
                method: 'POST',
                mode: 'cors',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: send_data
            }).then(response => {
                return response.json();
            })
                .then(data => {
                    if (data.user) {
                        // 注册成功，通过返回的数据设置 Cookie
                        cookieWatch.setCookie(cookieName.username, data.user.user_name);
                        cookieWatch.setCookie(cookieName.user_nickname, data.user.user_nickname);
                        cookieWatch.setCookie(cookieName.token, data.user.token);
                        cookieWatch.setCookie(cookieName.avatar, data.user.avatar);
                        // 转到完成注册的 Modal
                        this.sign_up_avatar = data.user.avatar;
                        this.sign_up_nickname = data.user.user_nickname;
                        location.reload();
                    } else {
                        data.error.forEach(element => {
                            switch (element) {
                                case 'email':
                                    this.sign_up_email_feedback = "邮箱已被注册";
                                    this.set_valid("sign-up-email-input", false);
                                    break;
                                case 'user_name':
                                    this.sign_up_username_feedback = "用户名重复";
                                    this.set_valid("sign-up-name-input", false);
                                    break;
                                case 'code':
                                    this.sign_up_code_feedback = "验证码错误";
                                    this.set_valid("email-code-input", false);
                                    break;
                                case 'unknow':
                                    this.sign_up_all_feedback = "发生未知错误";
                                    this.set_valid("sign-up-panel", false);
                                    break;
                            }
                        });
                    }
                    recoveryFn();
                }
                ).catch(error => {
                    recoveryFn();
                    console.log(error);
                })
        },
        reset_sign_up_valid() {
            this.remove_valid("sign-up-email-input");
            this.remove_valid("sign-up-name-input");
            this.remove_valid("email-code-input");
            this.remove_valid("sign-up-panel");
        },
        get_email_code() {
            // 检查邮箱输入
            if (!isEmail(this.sign_up_email)) return;
            // 获取 email 的输入
            var email_input = this.sign_up_email;

            $("#email-code-get-btn")
                .empty()
                .append('<span class="spinner-border spinner-border-sm"></span>')
                .attr("disabled", "disabled");

            // 将邮箱信息发送到服务器端
            fetch(getUrl(userPath.emailCodeGet), {
                method: 'POST',
                mode: 'cors',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ email: email_input, check_user_rule: 'no-exist' })
            })
                .then(response => response.json())
                .then(data => {
                    if (data.message) {
                        // 移除按钮的警告状态
                        if ($("#email-code-get-btn").hasClass("btn-outline-danger")) {
                            $("#email-code-get-btn").removeClass("btn-outline-danger");
                        }
                        // 提示发送成功
                        $("#email-code-get-btn").addClass("btn-outline-secondary");
                        $("#email-code-get-btn")
                            .empty()
                            .append("发送成功")
                            .attr("disabled", "disabled");
                        // 设置一段时间后可再次发送
                        setTimeout(() => {
                            $("#email-code-get-btn")
                                .empty()
                                .append("再次发送")
                                .removeAttr("disabled");
                            if ($("#email-code-get-btn").hasClass("btn-outline-secondary")) {
                                $("#email-code-get-btn").removeClass("btn-outline-secondary");
                            }
                            if ($("#email-code-get-btn").hasClass("btn-outline-secondary")) {
                                $("#email-code-get-btn").removeClass("btn-outline-secondary");
                            }
                        }, 30000);
                    } else {
                        // 修改按钮为发送失败的状态
                        if ($("#email-code-get-btn").hasClass("btn-outline-secondary")) {
                            $("#email-code-get-btn").removeClass("btn-outline-secondary");
                        }
                        $("#email-code-get-btn").addClass("btn-outline-danger");
                        $("#email-code-get-btn").empty().append(data.responseText);
                        $("#email-code-get-btn").removeAttr("disabled");
                    }
                })
                .catch(error => {
                    console.log(error);
                })
        },
    },
    watch: {
        sign_up_email(newValue) {
            this.set_valid("sign-up-email-input", isEmail(newValue));
        },
        sign_up_password(newPwd) {
            if (newPwd.length < 8) {
                this.sign_up_password_feedback = "密码长度太短"
                this.set_valid("sign-up-password-input", false);
                return;
            }
            if (!this.check_password_valid(newPwd)) {
                this.sign_up_password_feedback = "密码格式不正确：至少包含大小写字母、数字、特殊字符中的三种";
                this.set_valid("sign-up-password-input", false);
                return;
            }
            this.set_valid("sign-up-password-input", true);
        },
        sign_up_again_password(newValue) {
            this.set_valid("sign-up-password-again-input", newValue == this.sign_up_password);
        },
        sign_up_code(newValue) {
            if (newValue.length != 4) {
                this.set_valid("email-code-input", false);
            } else {
                this.remove_valid("email-code-input");
            }
        },
        sign_up_username(newName) {
            if (newName > 20 || newName < 4) {
                this.sign_up_username_feedback = "用户名太长或太短";
                this.set_valid("sign-up-name-input", false);
            } else {
                this.remove_valid("sign-up-name-input");
            }
        },
        sign_up_nickname(newNickname) {
            if (newNickname.length < 2 || newNickname > 30) {
                this.sign_up_nickname_feedback = "昵称过长或过短"
                this.set_valid("sign-up-set-nickname-input", false);
            } else {
                this.remove_valid("sign-up-set-nickname-input");
            }
        }
    },
    computed: {
        sign_avatar_is_empty() {
            return $.isEmptyObject(this.sign_up_avatar) || !this.sign_up_avatar;
        }
    }
}
</script>

<template>
    <div>
        <!-- 登录框 -->
        <div id="sign-in-modal" class="modal fade" tabindex="-1" data-bs-backdrop="static" data-bs-keyboard="false"
            aria-hidden="true" aria-labelledby="sign-in-title">
            <div class="modal-dialog modal-dialog-centered modal-sm">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="modal-title fs-5 text-center" id="sign-in-title">
                            登录
                        </h1>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"
                            id="sign-in-close"></button>
                    </div>
                    <div class="modal-body">
                        <div id="sign-in-panel" class="" @keyup.enter.native="sign_in">
                            <div class="form-floating mb-3">
                                <input type="email" class="form-control" id="sign-in-email-input"
                                    placeholder="name@example.com" required autocomplete="username" />
                                <label for="sign-in-email-input" class="text-black-50">邮箱/用户名</label>
                                <div class="invalid-feedback">{{ sign_in_name_feedback }}</div>
                            </div>
                            <div class="form-floating mb-3">
                                <input type="password" class="form-control" id="sign-in-password-input"
                                    placeholder="Password" required autocomplete="current-password" />
                                <label for="sign-in-password-input" class="text-black-50">密码</label>
                                <div class="invalid-feedback">{{ sign_in_pwd_feedback }}</div>
                            </div>
                        </div>
                        <div class="invalid-feedback">密码或邮箱错误</div>
                        <a href="/user/forget-password">忘记密码</a>
                    </div>
                    <div class="modal-footer">
                        <button class="btn btn-secondary" data-bs-target="#sign-up-modal" data-bs-toggle="modal"
                            id="sign-in-to-up">
                            注册
                        </button>
                        <button class="btn btn-primary" @click="sign_in" id="sign-in-button">
                            登录
                        </button>
                    </div>
                </div>
            </div>
        </div>
        <!-- 注册框 -->
        <div id="sign-up-modal" class="modal fade" tabindex="-1" data-bs-backdrop="static" data-bs-keyboard="false"
            aria-hidden="true" aria-labelledby="sign-up-title">
            <div class="modal-dialog modal-dialog-centered modal-sm">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="modal-title fs-5 text-center" id="sign-up-title">
                            注册
                        </h1>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"
                            id="sign-up-close"></button>
                    </div>
                    <div class="modal-body">
                        <div id="sign-up-panel" @keyup.enter.native="sign_up">
                            <div class="form-floating mb-3">
                                <input type="email" class="form-control" id="sign-up-email-input"
                                    placeholder="name@example.com" v-model="sign_up_email" required />
                                <label for="sign-up-email-input" class="text-black-50">邮箱</label>
                                <div class="invalid-feedback">{{ sign_up_email_feedback }}</div>
                            </div>
                            <div class="input-group mb-3">
                                <input type="text" class="form-control" placeholder="验证码" id="email-code-input"
                                    v-model="sign_up_code" required />
                                <button class="btn btn-outline-primary" id="email-code-get-btn" @click="get_email_code">
                                    获取验证码
                                </button>
                                <div class="invalid-feedback">{{ sign_up_code_feedback }}</div>
                            </div>
                            <div class="form-floating mb-3">
                                <input type="text" class="form-control" id="sign-up-name-input" placeholder="yourname"
                                    v-model="sign_up_username" required />
                                <label for="sign-up-name-input" class="text-black-50">用户名</label>
                                <div class="invalid-feedback">{{ sign_up_username_feedback }}</div>
                            </div>
                            <div class="form-floating mb-3">
                                <input type="password" class="form-control" id="sign-up-password-input"
                                    placeholder="Password" v-model="sign_up_password" required />
                                <label for="sign-up-password-input" class="text-black-50">密码</label>
                                <div class="invalid-feedback">{{ sign_up_password_feedback }}</div>
                            </div>
                            <div class="form-floating mb-3">
                                <input type="password" class="form-control" id="sign-up-password-again-input"
                                    placeholder="Password" v-model="sign_up_again_password" required />
                                <label for="sign-up-password-again-input" class="text-black-50">再次输入密码</label>
                                <div class="invalid-feedback">{{ sign_up_again_password_feedback }}</div>
                            </div>
                        </div>
                        <div class="invalid-feedback" id="sign-up-feedback">{{ sign_up_all_feedback }}</div>
                    </div>
                    <div class="modal-footer">
                        <button class="btn btn-secondary" data-bs-target="#sign-in-modal" data-bs-toggle="modal"
                            id="sign-up-to-in">
                            返回登录
                        </button>
                        <button class="btn btn-primary" @click="sign_up" id="sign-up-button">
                            注册
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>