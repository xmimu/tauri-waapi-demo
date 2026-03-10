import { createRouter, createWebHistory } from "vue-router";
import Layout from "../layouts/Layout.vue";
import { appChildRoutes } from "./routes";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      component: Layout,
      children: appChildRoutes,
    },
  ],
});
