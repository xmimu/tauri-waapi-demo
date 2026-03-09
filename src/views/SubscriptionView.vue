<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface SelectionPayload {
  args?: unknown;
  kwargs?: { objects?: Record<string, unknown>[] };
}

const subscribed = ref(false);
const subError = ref("");
const subLoading = ref(false);
const returnStr = ref("id name type");
const objectList = ref<Record<string, unknown>[]>([]);
let unlisten: (() => void) | null = null;

const returnList = computed(() => objectList.value);
const totalCount = computed(() => returnList.value.length);

const pageSize = ref(20);
const currentPage = ref(1);

const paginatedList = computed(() => {
  const list = returnList.value;
  const size = pageSize.value;
  const start = (currentPage.value - 1) * size;
  return list.slice(start, start + size);
});

const columns = computed(() => {
  const list = returnList.value;
  if (!list.length) return [];
  const keys = new Set<string>();
  for (const row of list) {
    for (const k of Object.keys(row)) keys.add(k);
  }
  return Array.from(keys);
});

function onPageChange(page: number) {
  currentPage.value = page;
}

function onSizeChange(size: number) {
  pageSize.value = size;
  currentPage.value = 1;
}

function cellValue(row: Record<string, unknown>, key: string): string {
  const v = row[key];
  if (v == null) return "—";
  if (typeof v === "object") {
    const o = v as Record<string, unknown>;
    if (o && typeof o.name === "string") return o.name;
    return JSON.stringify(v);
  }
  return String(v);
}

async function toggleSubscription() {
  subLoading.value = true;
  subError.value = "";
  const wantOn = subscribed.value;
  try {
    if (wantOn) {
      await invoke("subscribe_selection_start", {
        returnStr: returnStr.value.trim() || undefined,
      });
    } else {
      await invoke("subscribe_selection_stop");
    }
  } catch (e) {
    subError.value = e instanceof Error ? e.message : String(e);
    subscribed.value = !wantOn;
  } finally {
    subLoading.value = false;
  }
}

onMounted(async () => {
  unlisten = await listen<SelectionPayload>("waapi-selection-changed", (ev) => {
    const objs = (ev.payload?.kwargs?.objects ?? []) as Record<string, unknown>[];
    objectList.value = objs;
    currentPage.value = 1;
  });
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
});
</script>

<template>
  <div class="min-h-full p-6 box-border">
    <el-card class="w-full" shadow="hover">
      <template #header>
        <span>订阅 ak.wwise.ui.selectionChanged</span>
      </template>

      <div class="flex flex-col gap-4 w-full">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            return 属性（空格分隔，留空则使用 id name type notes）
          </label>
          <p class="text-xs text-gray-500 dark:text-gray-400 mb-1">
            不同对象类型具备不同属性，如 WorkUnit 无 volume，Sound 等可播放对象才有 volume
          </p>
          <el-input
            v-model="returnStr"
            placeholder="id name type path"
            class="w-full"
            :disabled="subscribed"
          />
        </div>
        <div class="flex items-center gap-4">
          <el-switch
            v-model="subscribed"
            :loading="subLoading"
            :disabled="subLoading"
            size="large"
            @change="toggleSubscription"
          />
          <span class="text-sm text-gray-600 dark:text-gray-400">
            {{ subscribed ? "订阅中：Wwise 选择变化时将推送到此处" : "关闭" }}
          </span>
        </div>

        <el-alert
          v-if="subError"
          type="error"
          :title="subError"
          show-icon
          class="w-full"
        />

        <template v-if="subscribed || objectList.length > 0">
          <h3 class="text-base font-medium mb-2 mt-2">结果 (objects)</h3>
          <p v-if="returnList.length" class="text-sm text-gray-600 dark:text-gray-400 mb-2">
            共 {{ totalCount }} 条
          </p>
          <el-table
            v-if="returnList.length"
            :data="paginatedList"
            border
            stripe
            class="w-full"
            style="width: 100%"
            max-height="400"
          >
            <el-table-column
              v-for="key in columns"
              :key="key"
              :prop="key"
              :label="key"
              min-width="120"
              show-overflow-tooltip
            >
              <template #default="{ row }">
                {{ cellValue(row, key) }}
              </template>
            </el-table-column>
          </el-table>
          <div
            v-if="returnList.length"
            class="flex flex-wrap items-center gap-4 mt-3 w-full"
          >
            <el-pagination
              :total="totalCount"
              :page-size="pageSize"
              :current-page="currentPage"
              :page-sizes="[10, 20, 50, 100]"
              layout="sizes, prev, pager, next, jumper"
              @current-change="onPageChange"
              @size-change="onSizeChange"
            />
          </div>
          <p v-else class="text-gray-500 dark:text-gray-400 text-sm">
            在 Wwise 中切换选中的对象，结果将在此展示
          </p>
        </template>
      </div>
    </el-card>
  </div>
</template>

<style scoped>
.el-card :deep(.el-card__body) {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 0.5rem;
}
</style>
