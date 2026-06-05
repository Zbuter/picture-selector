<template>
  <el-dialog
    v-model="visible"
    :title="image?.name || '图片预览'"
    width="90%"
    :close-on-click-modal="true"
    @closed="handleClose"
  >
    <div class="preview-container">
      <!-- 图片显示区域 -->
      <div class="image-wrapper">
        <img 
          v-if="image && imageUrl" 
          :src="imageUrl" 
          :alt="image.name"
          class="preview-image"
        />
        <div v-else class="loading-preview">
          <el-icon class="is-loading"><Loading /></el-icon>
          <span>加载中...</span>
        </div>
      </div>

      <!-- 导航按钮 -->
      <div class="nav-buttons">
        <el-button 
          circle 
          size="large"
          :disabled="!hasPrevious"
          @click="navigate(-1)"
        >
          <el-icon><ArrowLeft /></el-icon>
        </el-button>
        
        <el-button 
          circle 
          size="large"
          :disabled="!hasNext"
          @click="navigate(1)"
        >
          <el-icon><ArrowRight /></el-icon>
        </el-button>
      </div>

      <!-- 底部信息 -->
      <div class="preview-footer" v-if="image">
        <div class="image-details">
          <span>{{ image.name }}</span>
          <span class="divider">|</span>
          <span>{{ formatSize(image.size) }}</span>
          <span v-if="image.isRaw" class="raw-tag">RAW</span>
          <span class="shortcut-hint">空格：选择并下一张 | ← →：切换图片 | Esc：关闭</span>
        </div>
        
        <div class="action-buttons">
          <el-checkbox 
            :model-value="isSelected" 
            @change="toggleSelect"
          >
            选中此图片
          </el-checkbox>
          
          <el-button type="primary" @click="toggleSelect">
            {{ isSelected ? '取消选择' : '选择此图片' }}
          </el-button>
        </div>
      </div>
    </div>
  </el-dialog>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { Loading, ArrowLeft, ArrowRight } from '@element-plus/icons-vue';
import { useImageStore } from '../../stores/image';
import type { ImageFile } from '../../types';

const store = useImageStore();

const visible = computed({
  get: () => store.isPreviewOpen,
  set: (val) => {
    if (!val) {
      store.closePreview();
    }
  }
});

const image = computed(() => store.previewImage);
const imageUrl = ref<string>('');
const currentIndex = ref(0);

/**
 * 计算当前图片在列表中的索引
 */
watch(image, (newImage) => {
  if (newImage) {
    const index = store.images.findIndex(img => img.path === newImage.path);
    currentIndex.value = index >= 0 ? index : 0;
    loadFullImage(newImage);
  }
}, { immediate: true });

/**
 * 是否有上一张
 */
const hasPrevious = computed(() => currentIndex.value > 0);

/**
 * 是否有下一张
 */
const hasNext = computed(() => currentIndex.value < store.images.length - 1);

/**
 * 当前图片是否被选中
 */
const isSelected = computed(() => {
  if (!image.value) return false;
  return store.isSelected(image.value.path);
});

/**
 * 加载完整尺寸的图片
 */
async function loadFullImage(img: ImageFile) {
  try {
    // 使用 Tauri 读取文件并转换为 base64
    const { readBinaryFile } = await import('@tauri-apps/api/fs');
    const binary = await readBinaryFile(img.path);
    const base64 = arrayBufferToBase64(binary);
    
    // 根据文件扩展名确定 MIME 类型
    const ext = img.name.split('.').pop()?.toLowerCase() || 'jpg';
    const mimeType = getMimeType(ext);
    
    imageUrl.value = `data:${mimeType};base64,${base64}`;
  } catch (error) {
    console.error('Failed to load full image:', error);
    imageUrl.value = '';
  }
}

/**
 * ArrayBuffer 转 Base64
 */
function arrayBufferToBase64(buffer: ArrayBuffer): string {
  const bytes = new Uint8Array(buffer);
  let binary = '';
  for (let i = 0; i < bytes.byteLength; i++) {
    binary += String.fromCharCode(bytes[i]);
  }
  return btoa(binary);
}

/**
 * 获取 MIME 类型
 */
function getMimeType(ext: string): string {
  const mimeTypes: Record<string, string> = {
    jpg: 'image/jpeg',
    jpeg: 'image/jpeg',
    png: 'image/png',
    gif: 'image/gif',
    webp: 'image/webp',
    bmp: 'image/bmp',
    svg: 'image/svg+xml',
    arw: 'image/jpeg', // RAW 文件通常使用嵌入的 JPEG 预览
    nef: 'image/jpeg',
    cr2: 'image/jpeg',
    cr3: 'image/jpeg',
    dng: 'image/jpeg',
    raf: 'image/jpeg',
    orf: 'image/jpeg',
    pef: 'image/jpeg',
    srw: 'image/jpeg',
    rw2: 'image/jpeg'
  };
  return mimeTypes[ext] || 'application/octet-stream';
}

/**
 * 格式化文件大小
 */
function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

/**
 * 切换选择状态
 */
function toggleSelect() {
  if (image.value) {
    store.toggleSelect(image.value.path);
  }
}

/**
 * 导航到上一张/下一张
 */
function navigate(direction: number) {
  const newIndex = currentIndex.value + direction;
  if (newIndex >= 0 && newIndex < store.images.length) {
    currentIndex.value = newIndex;
    const newImage = store.images[newIndex];
    store.setPreviewImage(newImage);
  }
}

/**
 * 关闭预览时清理
 */
function handleClose() {
  imageUrl.value = '';
}

// 键盘导航
function handleKeydown(event: KeyboardEvent) {
  if (!visible.value) return;
  
  // 防止在输入框中触发
  if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
    return;
  }
  
  if (event.key === 'ArrowLeft') {
    event.preventDefault();
    navigate(-1);
  } else if (event.key === 'ArrowRight') {
    event.preventDefault();
    navigate(1);
  } else if (event.key === 'Escape') {
    store.closePreview();
  } else if (event.key === ' ' || event.code === 'Space') {
    // Space 键：选择当前图片并跳到下一张
    event.preventDefault();
    toggleSelect();
    if (hasNext.value) {
      navigate(1);
    }
  }
}

// 监听键盘事件
watch(visible, (newVal) => {
  if (newVal) {
    window.addEventListener('keydown', handleKeydown);
  } else {
    window.removeEventListener('keydown', handleKeydown);
  }
});
</script>

<style scoped>
.preview-container {
  display: flex;
  flex-direction: column;
  height: 80vh;
}

.image-wrapper {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #000;
  border-radius: 8px;
  overflow: hidden;
  margin-bottom: 16px;
}

.preview-image {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}

.loading-preview {
  display: flex;
  flex-direction: column;
  align-items: center;
  color: #fff;
}

.loading-preview .el-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.nav-buttons {
  position: absolute;
  top: 50%;
  left: 0;
  right: 0;
  transform: translateY(-50%);
  display: flex;
  justify-content: space-between;
  padding: 0 24px;
  pointer-events: none;
}

.nav-buttons .el-button {
  pointer-events: auto;
  background: rgba(255, 255, 255, 0.8);
  border: none;
}

.nav-buttons .el-button:hover {
  background: rgba(255, 255, 255, 1);
}

.preview-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  background: #f5f7fa;
  border-radius: 8px;
}

.image-details {
  display: flex;
  align-items: center;
  gap: 12px;
  color: #606266;
  font-size: 14px;
}

.shortcut-hint {
  color: #909399;
  font-size: 12px;
  margin-left: 8px;
  padding: 2px 8px;
  background: #f4f4f5;
  border-radius: 4px;
}

.divider {
  color: #dcdfe6;
}

.raw-tag {
  background: linear-gradient(135deg, #ff6b6b, #ee5a5a);
  color: white;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: bold;
}

.action-buttons {
  display: flex;
  align-items: center;
  gap: 16px;
}
</style>
