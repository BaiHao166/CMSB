import { createApp } from "vue";
import { createMemoryHistory, createRouter } from 'vue-router'
import routes from "./js/routes.js";
import App from "./App.vue";
import { invoke } from "@tauri-apps/api/core";

const router = createRouter({
    history: createMemoryHistory(),
    routes,
});

/**
 * 路由全局拦截
 * 检查是否登录或者登录状态是否仍然有效
 */
router.beforeEach(async (to, from) => {
    // 路由到登录页面时不需要验证
    if (to.name === "login") {
        return true;
    }

    await invoke("is_login")
        .then(result => {
            if (!result) {
                // 登录状态失效，跳转到登录页面
                return { name: "login" };
            }
        })

    return true;
})

createApp(App)
    .use(router)
    .mount("#app");
