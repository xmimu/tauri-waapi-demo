import { computed, type Ref } from "vue";
import type { WaapiObject } from "../features/waapi/types";

export function useObjectColumns(rows: Ref<WaapiObject[]>) {
  const columns = computed(() => {
    if (!rows.value.length) return [];
    const keys = new Set<string>();
    for (const row of rows.value) {
      for (const key of Object.keys(row)) {
        keys.add(key);
      }
    }
    return Array.from(keys);
  });

  function cellValue(row: WaapiObject, key: string): string {
    const value = row[key];
    if (value == null) return "-";
    if (typeof value === "object") {
      const maybeName = (value as WaapiObject).name;
      if (typeof maybeName === "string") {
        return maybeName;
      }
      return JSON.stringify(value);
    }
    return String(value);
  }

  return {
    columns,
    cellValue,
  };
}
