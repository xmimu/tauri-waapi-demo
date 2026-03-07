<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface Version {
  displayName?: string;
  year?: number;
  major?: number;
  minor?: number;
  build?: number;
  nickname?: string;
  schema?: number;
}

interface Directories {
  install?: string;
  authoring?: string;
  bin?: string;
  help?: string;
  user?: string;
}

interface WwiseInfo {
  sessionId?: string;
  apiVersion?: number;
  displayName?: string;
  branch?: string;
  copyright?: string;
  version?: Version;
  configuration?: string;
  platform?: string;
  isCommandLine?: boolean;
  processId?: number;
  processPath?: string;
  directories?: Directories;
}

const wwiseInfo = ref<WwiseInfo | null>(null);
const wwiseError = ref("");
const wwiseLoading = ref(false);

async function fetchWwiseInfo() {
  wwiseLoading.value = true;
  wwiseError.value = "";
  wwiseInfo.value = null;
  try {
    const raw = await invoke<WwiseInfo>("get_wwise_info");
    wwiseInfo.value = raw;
  } catch (e) {
    wwiseError.value = e instanceof Error ? e.message : String(e);
  } finally {
    wwiseLoading.value = false;
  }
}
</script>

<template>
  <div class="min-h-full p-6 box-border">
    <el-card class="w-full" shadow="hover">
      <template #header>
        <span>Wwise 信息 (ak.wwise.core.getInfo)</span>
      </template>
      <el-button
        type="primary"
        :loading="wwiseLoading"
        @click="fetchWwiseInfo"
      >
        获取 Wwise 信息
      </el-button>

      <template v-if="wwiseInfo">
        <el-descriptions :column="1" border class="w-full mb-4">
          <el-descriptions-item label="会话 ID (sessionId)">
            <el-text truncated class="block max-w-full" :title="wwiseInfo.sessionId">
              {{ wwiseInfo.sessionId ?? "—" }}
            </el-text>
          </el-descriptions-item>
          <el-descriptions-item label="API 版本 (apiVersion)">
            {{ wwiseInfo.apiVersion ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="显示名称 (displayName)">
            {{ wwiseInfo.displayName ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="分支 (branch)">
            {{ wwiseInfo.branch ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="版权 (copyright)">
            {{ wwiseInfo.copyright ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="配置 (configuration)">
            {{ wwiseInfo.configuration ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="平台 (platform)">
            {{ wwiseInfo.platform ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="命令行模式 (isCommandLine)">
            {{ wwiseInfo.isCommandLine == null ? "—" : wwiseInfo.isCommandLine ? "是" : "否" }}
          </el-descriptions-item>
          <el-descriptions-item label="进程 ID (processId)">
            {{ wwiseInfo.processId ?? "—" }}
          </el-descriptions-item>
          <el-descriptions-item label="进程路径 (processPath)">
            <el-text truncated class="block max-w-full" :title="wwiseInfo.processPath">
              {{ wwiseInfo.processPath ?? "—" }}
            </el-text>
          </el-descriptions-item>
        </el-descriptions>

        <template v-if="wwiseInfo.version">
          <h3 class="text-base font-medium mb-2 mt-4">版本 (version)</h3>
          <el-descriptions :column="1" border class="w-full mb-4">
            <el-descriptions-item label="版本名称 (displayName)">
              {{ wwiseInfo.version.displayName ?? "—" }}
            </el-descriptions-item>
            <el-descriptions-item label="年份 (year)">
              {{ wwiseInfo.version.year ?? "—" }}
            </el-descriptions-item>
            <el-descriptions-item label="大版本 (major)">
              {{ wwiseInfo.version.major ?? "—" }}
            </el-descriptions-item>
            <el-descriptions-item label="小版本 (minor)">
              {{ wwiseInfo.version.minor ?? "—" }}
            </el-descriptions-item>
            <el-descriptions-item label="内部版本 (build)">
              {{ wwiseInfo.version.build ?? "—" }}
            </el-descriptions-item>
            <el-descriptions-item label="版本号 (year.major.minor.build)">
              <span v-if="wwiseInfo.version.year != null">
                {{ wwiseInfo.version.year }}.{{ wwiseInfo.version.major ?? 0 }}.{{ wwiseInfo.version.minor ?? 0 }}.{{ wwiseInfo.version.build ?? 0 }}
              </span>
              <span v-else>—</span>
            </el-descriptions-item>
            <el-descriptions-item label="昵称 (nickname)">
              {{ wwiseInfo.version.nickname ?? "—" }}
            </el-descriptions-item>
            <el-descriptions-item label="架构版本 (schema)">
              {{ wwiseInfo.version.schema ?? "—" }}
            </el-descriptions-item>
          </el-descriptions>
        </template>

        <template v-if="wwiseInfo.directories">
          <h3 class="text-base font-medium mb-2 mt-4">目录 (directories)</h3>
          <el-descriptions :column="1" border class="w-full mb-4">
            <el-descriptions-item label="安装目录 (install)">
              <el-text truncated class="block max-w-full" :title="wwiseInfo.directories.install">
                {{ wwiseInfo.directories.install ?? "—" }}
              </el-text>
            </el-descriptions-item>
            <el-descriptions-item label="设计工具根目录 (authoring)">
              <el-text truncated class="block max-w-full" :title="wwiseInfo.directories.authoring">
                {{ wwiseInfo.directories.authoring ?? "—" }}
              </el-text>
            </el-descriptions-item>
            <el-descriptions-item label="bin 目录 (bin)">
              <el-text truncated class="block max-w-full" :title="wwiseInfo.directories.bin">
                {{ wwiseInfo.directories.bin ?? "—" }}
              </el-text>
            </el-descriptions-item>
            <el-descriptions-item label="帮助目录 (help)">
              <el-text truncated class="block max-w-full" :title="wwiseInfo.directories.help">
                {{ wwiseInfo.directories.help ?? "—" }}
              </el-text>
            </el-descriptions-item>
            <el-descriptions-item label="用户数据目录 (user)">
              <el-text truncated class="block max-w-full" :title="wwiseInfo.directories.user">
                {{ wwiseInfo.directories.user ?? "—" }}
              </el-text>
            </el-descriptions-item>
          </el-descriptions>
        </template>
      </template>

      <el-alert
        v-if="wwiseError"
        type="error"
        :title="wwiseError"
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
