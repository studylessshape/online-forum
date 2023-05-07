import { createApp } from 'vue'
import App from './App.vue'

// add router
import router from './routes/index.js'

// import jquery
import './assets/js/jquery-3.6.3.min.js'

// import bootstrap
import './assets/css/bootstrap.min.css'
import './assets/js/bootstrap.bundle.min.js'

import './assets/css/style.css'
// import element plus
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

const app = createApp(App);

app.use(router);
// import element plus component
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
    app.component(key, component)
}
app.use(ElementPlus);

app.mount('#app');
