import type { RouteRecordRaw } from "vue-router";

export const appChildRoutes: RouteRecordRaw[] = [
  {
    path: "",
    name: "Home",
    component: () => import("../views/HomeView.vue"),
    meta: { title: "首页", navLabel: "首页", keepAlive: false },
  },
  {
    path: "wwise",
    name: "Wwise",
    component: () => import("../views/WwiseView.vue"),
    meta: { title: "Wwise 信息", navLabel: "Wwise 信息", keepAlive: true },
  },
  {
    path: "project",
    name: "Project",
    component: () => import("../views/ProjectView.vue"),
    meta: { title: "工程信息", navLabel: "工程信息", keepAlive: true },
  },
  {
    path: "waql",
    name: "Waql",
    component: () => import("../views/WaqlView.vue"),
    meta: { title: "WAQL 查询", navLabel: "WAQL 查询", keepAlive: true },
  },
  {
    path: "subscription",
    name: "Subscription",
    component: () => import("../views/SubscriptionView.vue"),
    meta: { title: "订阅", navLabel: "订阅", keepAlive: true },
  },
];

export interface NavItem {
  name: string;
  to: string;
  label: string;
}

export const navItems: NavItem[] = appChildRoutes
  .filter((route) => typeof route.name === "string")
  .map((route) => ({
    name: route.name as string,
    to: route.path ? `/${route.path}` : "/",
    label: String(route.meta?.navLabel ?? route.name),
  }));
