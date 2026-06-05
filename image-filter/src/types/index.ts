// 图片文件类型定义
export interface ImageFile {
  path: string;
  name: string;
  size: number;
  modified: number;
  isRaw: boolean;
  thumbnail?: string; // base64 encoded thumbnail
}

// 驱动器信息
export interface DriveInfo {
  name: string;
  path: string;
  isRemovable: boolean;
  totalSpace: number;
  freeSpace: number;
}

// 应用状态
export interface AppState {
  currentFolder: string | null;
  images: ImageFile[];
  selectedImages: Set<string>; // 使用路径作为 key
  previewImage: ImageFile | null;
  isPreviewOpen: boolean;
  targetFolder: string | null;
  drives: DriveInfo[];
  isLoading: boolean;
  statusMessage: string;
}

// 文件夹监听事件
export interface WatchEvent {
  type: 'create' | 'modify' | 'delete' | 'rename';
  path: string;
}
