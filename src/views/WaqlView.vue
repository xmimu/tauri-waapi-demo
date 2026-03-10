<script setup lang="ts">
import { computed, ref } from "vue";
import { useObjectColumns } from "../composables/useObjectColumns";
import { usePaginatedTable } from "../composables/usePaginatedTable";
import { objectGet } from "../features/waapi/api";
import { DEFAULT_QUERY_RETURN } from "../features/waapi/constants";
import type { WaqlResult } from "../features/waapi/types";

const waql = ref("$ from type Event");
const returnStr = ref(DEFAULT_QUERY_RETURN);
const result = ref<WaqlResult | null>(null);
const waqlError = ref("");
const waqlLoading = ref(false);

const returnList = computed(() => result.value?.return ?? []);
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

async function runWaql() {
  waqlLoading.value = true;
  waqlError.value = "";
  result.value = null;
  try {
    result.value = await objectGet(waql.value, returnStr.value);
    resetPage();
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
            placeholder="$ from type Event"
            class="w-full"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">return 属性（空格分隔）</label>
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
