<script setup lang="ts">
import { ref } from "vue";
import { getProjectInfo } from "../features/waapi/api";
import type { ProjectInfo } from "../features/waapi/types";

const projectInfo = ref<ProjectInfo | null>(null);
const projectError = ref("");
const projectLoading = ref(false);

async function fetchProjectInfo() {
  projectLoading.value = true;
  projectError.value = "";
  projectInfo.value = null;
  try {
    const raw = await getProjectInfo();
    projectInfo.value = raw;
  } catch (e) {
    projectError.value = e instanceof Error ? e.message : String(e);
  } finally {
    projectLoading.value = false;
  }
}
</script>

<template>
  <div class="min-h-full p-6 box-border">
    <el-card class="w-full" shadow="hover">
      <template #header>
        <span>工程信息 (ak.wwise.core.getProjectInfo)</span>
      </template>
      <el-button
        type="primary"
        :loading="projectLoading"
        @click="fetchProjectInfo"
      >
        获取工程信息
      </el-button>

      <template v-if="projectInfo">
        <el-descriptions :column="1" border class="w-full mb-4">
          <el-descriptions-item label="Project 名称 (name)">
            {{ projectInfo.name ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="标题栏文本 (displayTitle)">
            {{ projectInfo.displayTitle ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="WPROJ 路径 (path)">
            <el-text truncated class="block max-w-full" :title="projectInfo.path">
              {{ projectInfo.path ?? "—" }}
            </el-text>
          </el-descriptions-item>
          <el-descriptions-item label="Project ID (id)">
            {{ projectInfo.id ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="未保存更改 (isDirty)">
            {{ projectInfo.isDirty == null ? "—" : projectInfo.isDirty ? "是" : "否" }}
          </el-descriptions-item>
          <el-descriptions-item label="当前语言 ID (currentLanguageId)">
            {{ projectInfo.currentLanguageId ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="参考语言 ID (referenceLanguageId)">
            {{ projectInfo.referenceLanguageId ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="当前平台 ID (currentPlatformId)">
            {{ projectInfo.currentPlatformId ?? "—" }}
          </el-descriptions-item>
        </el-descriptions>

        <template v-if="projectInfo.defaultConversion">
          <h3 class="text-base font-medium mb-2 mt-4">默认转换设置 (defaultConversion)</h3>
          <el-descriptions :column="1" border class="w-full mb-4">
            <el-descriptions-item label="ID">
              {{ projectInfo.defaultConversion.id ?? "—" }}
            </el-descriptions-item>
            <el-descriptions-item label="名称">
              {{ projectInfo.defaultConversion.name ?? "—" }}
            </el-descriptions-item>
          </el-descriptions>
        </template>

        <template v-if="projectInfo.directories">
          <h3 class="text-base font-medium mb-2 mt-4">目录 (directories)</h3>
          <el-descriptions :column="1" border class="w-full mb-4">
            <el-descriptions-item label="根目录 (root)">
              <el-text truncated class="block max-w-full" :title="projectInfo.directories.root">
                {{ projectInfo.directories.root ?? "—" }}
              </el-text>
            </el-descriptions-item>
            <el-descriptions-item label=".cache (cache)">
              <el-text truncated class="block max-w-full" :title="projectInfo.directories.cache">
                {{ projectInfo.directories.cache ?? "—" }}
              </el-text>
            </el-descriptions-item>
            <el-descriptions-item label="Originals (originals)">
              <el-text truncated class="block max-w-full" :title="projectInfo.directories.originals">
                {{ projectInfo.directories.originals ?? "—" }}
              </el-text>
            </el-descriptions-item>
            <el-descriptions-item label="SoundBank 输出根目录 (soundBankOutputRoot)">
              <el-text truncated class="block max-w-full" :title="projectInfo.directories.soundBankOutputRoot">
                {{ projectInfo.directories.soundBankOutputRoot ?? "—" }}
              </el-text>
            </el-descriptions-item>
            <el-descriptions-item label="Commands (commands)">
              <el-text truncated class="block max-w-full" :title="projectInfo.directories.commands">
                {{ projectInfo.directories.commands ?? "—" }}
              </el-text>
            </el-descriptions-item>
            <el-descriptions-item label="Properties (properties)">
              <el-text truncated class="block max-w-full" :title="projectInfo.directories.properties">
                {{ projectInfo.directories.properties ?? "—" }}
              </el-text>
            </el-descriptions-item>
          </el-descriptions>
        </template>

        <template v-if="projectInfo.languages?.length">
          <h3 class="text-base font-medium mb-2 mt-4">语言 (languages)</h3>
          <el-table :data="projectInfo.languages" border stripe class="w-full mb-4">
            <el-table-column prop="name" label="名称" min-width="120" />
            <el-table-column prop="id" label="ID" min-width="280" show-overflow-tooltip />
            <el-table-column prop="shortId" label="Short ID" width="100" align="right" />
          </el-table>
        </template>

        <template v-if="projectInfo.platforms?.length">
          <h3 class="text-base font-medium mb-2 mt-4">平台 (platforms)</h3>
          <el-table :data="projectInfo.platforms" border stripe class="w-full">
            <el-table-column prop="name" label="名称" min-width="100" />
            <el-table-column prop="baseName" label="baseName" min-width="100" />
            <el-table-column prop="baseDisplayName" label="baseDisplayName" min-width="120" />
            <el-table-column prop="soundBankPath" label="SoundBank 路径" min-width="200" show-overflow-tooltip />
            <el-table-column prop="copiedMediaPath" label="复制媒体路径" min-width="200" show-overflow-tooltip />
          </el-table>
        </template>
      </template>

      <el-alert
        v-if="projectError"
        type="error"
        :title="projectError"
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
