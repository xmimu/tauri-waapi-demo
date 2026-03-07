<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRoute, RouterLink } from "vue-router";

const route = useRoute();

const isDark = ref(false);

function toggleTheme() {
  isDark.value = !isDark.value;
  const html = document.documentElement;
  if (isDark.value) {
    html.classList.add("dark");
    html.setAttribute("data-theme", "dark");
  } else {
    html.classList.remove("dark");
    html.setAttribute("data-theme", "light");
  }
  try {
    localStorage.setItem("theme", isDark.value ? "dark" : "light");
  } catch (_) {}
}

onMounted(() => {
  const stored = localStorage.getItem("theme");
  const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  isDark.value = stored === "dark" || (!stored && prefersDark);
  if (isDark.value) {
    document.documentElement.classList.add("dark");
    document.documentElement.setAttribute("data-theme", "dark");
  } else {
    document.documentElement.classList.remove("dark");
    document.documentElement.setAttribute("data-theme", "light");
  }
});
</script>

<template>
  <div class="min-h-screen flex flex-col bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100 transition-colors">
    <!-- 顶栏 -->
    <header class="shrink-0 h-14 px-4 flex items-center justify-between border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 shadow-sm">
      <nav class="flex items-center gap-6">
        <RouterLink
          to="/"
          class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400"
          :class="route.name === 'Home' ? 'text-blue-600 dark:text-blue-400' : 'text-gray-600 dark:text-gray-400'"
        >
          首页
        </RouterLink>
        <RouterLink
          to="/wwise"
          class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400"
          :class="route.name === 'Wwise' ? 'text-blue-600 dark:text-blue-400' : 'text-gray-600 dark:text-gray-400'"
        >
          Wwise 信息
        </RouterLink>
        <RouterLink
          to="/project"
          class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400"
          :class="route.name === 'Project' ? 'text-blue-600 dark:text-blue-400' : 'text-gray-600 dark:text-gray-400'"
        >
          工程信息
        </RouterLink>
        <RouterLink
          to="/waql"
          class="text-sm font-medium transition-colors hover:text-blue-600 dark:hover:text-blue-400"
          :class="route.name === 'Waql' ? 'text-blue-600 dark:text-blue-400' : 'text-gray-600 dark:text-gray-400'"
        >
          WAQL 查询
        </RouterLink>
      </nav>
      <div class="flex items-center">
        <button
          type="button"
          :aria-label="isDark ? '切换到浅色' : '切换到深色'"
          class="inline-flex h-9 w-9 items-center justify-center rounded-lg text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 dark:focus-visible:ring-offset-gray-800"
          @click="toggleTheme"
        >
          <!-- 当前为浅色时显示月亮，点击切深色；当前为深色时显示太阳，点击切浅色 -->
          <svg v-if="!isDark" class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
          </svg>
          <svg v-else class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
          </svg>
        </button>
      </div>
    </header>

    <!-- 主内容区：不同页面（KeepAlive 缓存已加载的页面数据，切换回来时无需重新请求） -->
    <main class="flex-1 overflow-auto">
      <RouterView v-slot="{ Component }">
        <KeepAlive>
          <component :is="Component" />
        </KeepAlive>
      </RouterView>
    </main>
  </div>
</template>
