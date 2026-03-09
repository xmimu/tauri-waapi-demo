import { createRouter, createWebHistory } from "vue-router";
import Layout from "../layouts/Layout.vue";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      component: Layout,
      children: [
        { path: "", name: "Home", component: () => import("../views/HomeView.vue"), meta: { title: "首页" } },
        { path: "wwise", name: "Wwise", component: () => import("../views/WwiseView.vue"), meta: { title: "Wwise 信息" } },
        { path: "project", name: "Project", component: () => import("../views/ProjectView.vue"), meta: { title: "工程信息" } },
        { path: "waql", name: "Waql", component: () => import("../views/WaqlView.vue"), meta: { title: "WAQL 查询" } },
        { path: "subscription", name: "Subscription", component: () => import("../views/SubscriptionView.vue"), meta: { title: "订阅" } },
      ],
    },
  ],
});
