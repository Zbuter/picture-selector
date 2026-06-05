<template>
  <div class="image-grid">
    <!-- 空状态 -->
    <el-empty 
      v-if="!store.images.length" 
      :description="store.currentFolder ? '该文件夹中没有图片' : '请选择一个文件夹开始'"
    >
      <template #extra>
        <el-button type="primary" @click="handleSelectFolder">
          选择文件夹
        </el-button>
      </template>
    </el-empty>

    <!-- 图片网格 -->
    <div v-else class="grid-container">
      <ImageCard
        v-for="image in store.images"
        :key="image.path"
        :image="image"
        @preview="handlePreview"
      />
    </div>

    <!-- 加载状态 -->
    <div v-if="store.isLoading" class="loading-overlay">
      <el-icon class="is-loading"><Loading /></el-icon>
      <span>加载中...</span>
    </div>

    <!-- 预览对话框 -->
    <ImagePreview />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { Loading } from '@element-plus/icons-vue';
import { useImageStore } from '../../stores/image';
import ImageCard from './ImageCard.vue';
import ImagePreview from './ImagePreview.vue';
import { selectFolder, loadImages, generateThumbnail, startWatching } from '../../composables/useImageApi';
import { ElMessage } from 'element-plus';
import type { ImageFile } from '../../types';

const store = useImageStore();

/**
 * 选择文件夹
 */
async function handleSelectFolder() {
  try {
    store.setLoading(true);
    const folder = await selectFolder();
    
    if (folder) {
      store.setCurrentFolder(folder);
      await loadAndProcessImages(folder);
    }
  } catch (error) {
    console.error('Failed to select folder:', error);
    ElMessage.error('选择文件夹失败');
  } finally {
    store.setLoading(false);
  }
}

/**
 * 加载并处理图片
 */
async function loadAndProcessImages(folder: string) {
  try {
    const images = await loadImages(folder);
    store.setImages(images);
    
    // 异步生成缩略图
    generateThumbnails(images);
    
    // 开始监听文件夹变化
    await startWatching(folder);
    
    ElMessage.success(`已加载 ${images.length} 张图片`);
  } catch (error) {
    console.error('Failed to load images:', error);
    ElMessage.error('加载图片失败');
  }
}

/**
 * 批量生成缩略图
 */
async function generateThumbnails(images: ImageFile[]) {
  // 限制并发数量，避免阻塞
  const batchSize = 5;
  
  for (let i = 0; i < images.length; i += batchSize) {
    const batch = images.slice(i, i + batchSize);
    
    await Promise.all(
      batch.map(async (image) => {
        try {
          const thumbnail = await generateThumbnail(image.path, 400);
          // 找到对应的图片并更新缩略图
          const index = store.images.findIndex(img => img.path === image.path);
          if (index >= 0) {
            store.images[index].thumbnail = thumbnail;
          }
        } catch (error) {
          console.error(`Failed to generate thumbnail for ${image.name}:`, error);
        }
      })
    );
  }
}

/**
 * 处理预览事件
 */
function handlePreview(image: ImageFile) {
  store.setPreviewImage(image);
}

// 组件挂载时，如果有当前文件夹则重新加载
onMounted(() => {
  if (store.currentFolder && !store.images.length) {
    loadAndProcessImages(store.currentFolder);
  }
});
</script>

<style scoped>
.image-grid {
  flex: 1;
  padding: 24px;
  background: #f5f7fa;
  overflow-y: auto;
  position: relative;
}

.grid-container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 20px;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.9);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  z-index: 10;
}

.loading-overlay .el-icon {
  font-size: 48px;
  color: #409eff;
  margin-bottom: 16px;
}

.loading-overlay span {
  color: #606266;
  font-size: 16px;
}
</style>
