import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { ImageFile, DriveInfo } from '../types';

// RAW 格式扩展名列表
const RAW_EXTENSIONS = new Set([
  'arw', 'nef', 'cr2', 'cr3', 'dng', 'raf', 'orf', 'pef', 'srw', 'rw2',
  'iiq', '3fr', 'mef', 'mos', 'nrw', 'rwl', 'raw', 'r3d'
]);

export const useImageStore = defineStore('image', () => {
  // 状态
  const currentFolder = ref<string | null>(null);
  const images = ref<ImageFile[]>([]);
  const selectedImages = ref<Set<string>>(new Set());
  const previewImage = ref<ImageFile | null>(null);
  const isPreviewOpen = ref(false);
  const targetFolder = ref<string | null>(null);
  const drives = ref<DriveInfo[]>([]);
  const isLoading = ref(false);
  const statusMessage = ref('');
  const watcherActive = ref(false);

  // 计算属性
  const selectedCount = computed(() => selectedImages.value.size);
  
  const allSelected = computed(() => {
    if (images.value.length === 0) return false;
    return selectedImages.value.size === images.value.length;
  });

  const hasSelection = computed(() => selectedImages.value.size > 0);

  // 方法
  function setCurrentFolder(folder: string | null) {
    currentFolder.value = folder;
    if (!folder) {
      images.value = [];
      selectedImages.value.clear();
    }
  }

  function setImages(newImages: ImageFile[]) {
    images.value = newImages;
    // 清空选择，但保留之前的选择如果路径还存在
    const newSelected = new Set<string>();
    selectedImages.value.forEach(path => {
      if (newImages.some(img => img.path === path)) {
        newSelected.add(path);
      }
    });
    selectedImages.value = newSelected;
  }

  function toggleSelect(imagePath: string) {
    if (selectedImages.value.has(imagePath)) {
      selectedImages.value.delete(imagePath);
    } else {
      selectedImages.value.add(imagePath);
    }
  }

  function selectAll() {
    images.value.forEach(img => selectedImages.value.add(img.path));
  }

  function deselectAll() {
    selectedImages.value.clear();
  }

  function isSelected(imagePath: string): boolean {
    return selectedImages.value.has(imagePath);
  }

  function setPreviewImage(image: ImageFile | null) {
    previewImage.value = image;
    isPreviewOpen.value = !!image;
  }

  function closePreview() {
    isPreviewOpen.value = false;
    previewImage.value = null;
  }

  function setTargetFolder(folder: string | null) {
    targetFolder.value = folder;
  }

  function setDrives(newDrives: DriveInfo[]) {
    drives.value = newDrives;
  }

  function setLoading(loading: boolean) {
    isLoading.value = loading;
  }

  function setStatus(message: string) {
    statusMessage.value = message;
    // 3 秒后自动清除状态消息
    if (message) {
      setTimeout(() => {
        if (statusMessage.value === message) {
          statusMessage.value = '';
        }
      }, 3000);
    }
  }

  function setWatcherActive(active: boolean) {
    watcherActive.value = active;
  }

  function isRawFile(filename: string): boolean {
    const ext = filename.split('.').pop()?.toLowerCase() || '';
    return RAW_EXTENSIONS.has(ext);
  }

  function getSelectedImagePaths(): string[] {
    return Array.from(selectedImages.value);
  }

  function removeImage(imagePath: string) {
    images.value = images.value.filter(img => img.path !== imagePath);
    selectedImages.value.delete(imagePath);
  }

  function addImage(image: ImageFile) {
    // 避免重复
    if (!images.value.some(img => img.path === image.path)) {
      images.value.push(image);
    }
  }

  return {
    // 状态
    currentFolder,
    images,
    selectedImages,
    previewImage,
    isPreviewOpen,
    targetFolder,
    drives,
    isLoading,
    statusMessage,
    watcherActive,
    // 计算属性
    selectedCount,
    allSelected,
    hasSelection,
    // 方法
    setCurrentFolder,
    setImages,
    toggleSelect,
    selectAll,
    deselectAll,
    isSelected,
    setPreviewImage,
    closePreview,
    setTargetFolder,
    setDrives,
    setLoading,
    setStatus,
    setWatcherActive,
    isRawFile,
    getSelectedImagePaths,
    removeImage,
    addImage
  };
});
