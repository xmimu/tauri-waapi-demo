export interface WwiseVersion {
  displayName?: string;
  year?: number;
  major?: number;
  minor?: number;
  build?: number;
  nickname?: string;
  schema?: number;
}

export interface WwiseDirectories {
  install?: string;
  authoring?: string;
  bin?: string;
  help?: string;
  user?: string;
}

export interface WwiseInfo {
  sessionId?: string;
  apiVersion?: number;
  displayName?: string;
  branch?: string;
  copyright?: string;
  version?: WwiseVersion;
  configuration?: string;
  platform?: string;
  isCommandLine?: boolean;
  processId?: number;
  processPath?: string;
  directories?: WwiseDirectories;
}

export interface ProjectLanguage {
  id: string;
  name: string;
  shortId: number;
}

export interface ProjectPlatform {
  id: string;
  name: string;
  baseName: string;
  baseDisplayName: string;
  soundBankPath: string;
  copiedMediaPath: string;
}

export interface ProjectDefaultConversion {
  id: string;
  name: string;
}

export interface ProjectDirectories {
  root?: string;
  cache?: string;
  originals?: string;
  soundBankOutputRoot?: string;
  commands?: string;
  properties?: string;
}

export interface ProjectInfo {
  name?: string;
  displayTitle?: string;
  path?: string;
  id?: string;
  isDirty?: boolean;
  currentLanguageId?: string;
  referenceLanguageId?: string;
  currentPlatformId?: string;
  languages?: ProjectLanguage[];
  platforms?: ProjectPlatform[];
  defaultConversion?: ProjectDefaultConversion;
  directories?: ProjectDirectories;
}

export type WaapiObject = Record<string, unknown>;

export interface WaqlResult {
  return?: WaapiObject[];
}

export interface SelectionPayload {
  args?: unknown;
  kwargs?: {
    objects?: WaapiObject[];
  };
}
