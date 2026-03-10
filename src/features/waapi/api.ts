import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  ProjectInfo,
  SelectionPayload,
  WaapiObject,
  WaqlResult,
  WwiseInfo,
} from "./types";

const SELECTION_CHANGED_EVENT = "waapi-selection-changed";

function normalizeReturnStr(value?: string): string | undefined {
  const trimmed = value?.trim();
  return trimmed ? trimmed : undefined;
}

export async function getWwiseInfo(): Promise<WwiseInfo> {
  return invoke<WwiseInfo>("get_wwise_info");
}

export async function getProjectInfo(): Promise<ProjectInfo> {
  return invoke<ProjectInfo>("get_project_info");
}

export async function objectGet(
  waql: string,
  returnStr?: string,
): Promise<WaqlResult> {
  return invoke<WaqlResult>("object_get", {
    waql,
    returnStr: normalizeReturnStr(returnStr),
  });
}

export async function subscribeSelectionStart(
  returnStr?: string,
): Promise<void> {
  await invoke("subscribe_selection_start", {
    returnStr: normalizeReturnStr(returnStr),
  });
}

export async function subscribeSelectionStop(): Promise<void> {
  await invoke("subscribe_selection_stop");
}

export async function listenSelectionChanged(
  onChanged: (objects: WaapiObject[], payload: SelectionPayload) => void,
): Promise<UnlistenFn> {
  return listen<SelectionPayload>(SELECTION_CHANGED_EVENT, (event) => {
    const payload = event.payload ?? {};
    const objects = payload.kwargs?.objects ?? [];
    onChanged(objects, payload);
  });
}
