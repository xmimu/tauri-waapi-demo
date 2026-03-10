import { createApp } from "vue";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import "element-plus/theme-chalk/dark/css-vars.css";
import "./index.css";
import App from "./App.vue";
import { router } from "./router";

router.afterEach((to) => {
  const pageTitle = to.meta.title;
  document.title = pageTitle ? `${pageTitle} - tauri-waapi-demo` : "tauri-waapi-demo";
});

const app = createApp(App);
app.use(ElementPlus);
app.use(router);
app.mount("#app");
