<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

const waql = ref("$ from type Event");
const returnStr = ref("id name type");
const result = ref<{ return?: Record<string, unknown>[] } | null>(null);
const waqlError = ref("");
const waqlLoading = ref(false);

const returnList = computed(() => result.value?.return ?? []);
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

async function runWaql() {
  waqlLoading.value = true;
  waqlError.value = "";
  result.value = null;
  try {
    const raw = await invoke<{ return?: Record<string, unknown>[] }>("object_get", {
      waql: waql.value,
      returnStr: returnStr.value.trim() || undefined,
    });
    result.value = raw;
    currentPage.value = 1;
  } catch (e) {
    waqlError.value = e instanceof Error ? e.message : String(e);
  } finally {
    waqlLoading.value = false;
  }
}
</script>

<template>
  <div class="min-h-full p-6 box-border">
    <el-card class="w-full" shadow="hover">
      <template #header>
        <span>WAQL 查询 (ak.wwise.core.object.get)</span>
      </template>

      <div class="flex flex-col gap-4 w-full max-w-3xl">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">WAQL 语句</label>
          <el-input
            v-model="waql"
            type="textarea"
            :rows="2"
            placeholder="$ from type Event"
            class="w-full"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">return 属性（空格分隔，留空则使用 id name type notes）</label>
          <el-input
            v-model="returnStr"
            placeholder="id name type path"
            class="w-full"
          />
        </div>
        <el-button
          type="primary"
          :loading="waqlLoading"
          @click="runWaql"
        >
          执行 WAQL 查询
        </el-button>
      </div>

      <template v-if="result !== null">
        <h3 class="text-base font-medium mb-2 mt-6">结果 (return)</h3>
        <p v-if="returnList.length" class="text-sm text-gray-600 dark:text-gray-400 mb-2">
          共 {{ totalCount }} 条
        </p>
        <el-table
          v-if="returnList.length"
          :data="paginatedList"
          border
          stripe
          class="w-full"
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
        <p v-else class="text-gray-500 dark:text-gray-400 text-sm">无结果</p>
      </template>

      <el-alert
        v-if="waqlError"
        type="error"
        :title="waqlError"
        show-icon
        class="w-full mt-4"
      />
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
