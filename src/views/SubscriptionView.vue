<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useObjectColumns } from "../composables/useObjectColumns";
import { usePaginatedTable } from "../composables/usePaginatedTable";
import {
  listenSelectionChanged,
  subscribeSelectionStart,
  subscribeSelectionStop,
} from "../features/waapi/api";
import { DEFAULT_QUERY_RETURN } from "../features/waapi/constants";
import type { WaapiObject } from "../features/waapi/types";

const subscribed = ref(false);
const subError = ref("");
const subLoading = ref(false);
const returnStr = ref(DEFAULT_QUERY_RETURN);
const objectList = ref<WaapiObject[]>([]);
let unlisten: (() => void) | null = null;

const returnList = computed(() => objectList.value);
const {
  pageSize,
  currentPage,
  totalCount,
  paginatedList,
  onPageChange,
  onSizeChange,
  resetPage,
} = usePaginatedTable(returnList);
const { columns, cellValue } = useObjectColumns(returnList);

async function toggleSubscription() {
  subLoading.value = true;
  subError.value = "";
  const wantOn = subscribed.value;

  try {
    if (wantOn) {
      await subscribeSelectionStart(returnStr.value);
    } else {
      await subscribeSelectionStop();
    }
  } catch (e) {
    subError.value = e instanceof Error ? e.message : String(e);
    subscribed.value = !wantOn;
  } finally {
    subLoading.value = false;
  }
}

onMounted(async () => {
  unlisten = await listenSelectionChanged((objects) => {
    objectList.value = objects;
    resetPage();
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
            return 属性（空格分隔）
          </label>
          <p class="text-xs text-gray-500 dark:text-gray-400 mb-1">
            不同对象类型可用字段不同，例如可播放对象通常才有 volume
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
            {{ subscribed ? "订阅中：Wwise 选中变化会推送到此处" : "已关闭" }}
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
            在 Wwise 中切换选中的对象，结果会显示在这里
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
