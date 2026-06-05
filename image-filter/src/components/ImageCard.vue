<template>
  <div class="image-card" :class="{ selected: isSelected, 'is-raw': image.isRaw }">
    <div class="image-thumbnail">
      <img 
        v-if="image.thumbnail" 
        :src="image.thumbnail" 
        :alt="image.name"
        loading="lazy"
        @click="handleClick"
        @dblclick="handleDoubleClick"
      />
      <div v-else class="thumbnail-placeholder" @click="handleClick">
        <el-icon><Picture /></el-icon>
        <span class="loading-text">加载中...</span>
      </div>
      
      <!-- RAW 格式标识 -->
      <div v-if="image.isRaw" class="raw-badge">RAW</div>
      
      <!-- 选择框 -->
      <div class="checkbox-overlay" @click.stop="toggleSelect">
        <el-checkbox 
          :model-value="isSelected" 
          size="large"
          @change="toggleSelect"
        />
      </div>
    </div>
    
    <div class="image-info" @click="handleClick">
      <div class="image-name" :title="image.name">{{ image.name }}</div>
      <div class="image-size">{{ formatSize(image.size) }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { Picture } from '@element-plus/icons-vue';
import { useImageStore } from '../../stores/image';
import type { ImageFile } from '../../types';

const props = defineProps<{
  image: ImageFile;
}>();

const emit = defineEmits<{
  (e: 'preview', image: ImageFile): void;
}>();

const store = useImageStore();

const isSelected = computed(() => store.isSelected(props.image.path));

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
  store.toggleSelect(props.image.path);
}

/**
 * 单击选中
 */
function handleClick() {
  // 如果按住 Ctrl/Cmd 键，切换选择；否则单选
  toggleSelect();
}

/**
 * 双击预览
 */
function handleDoubleClick() {
  emit('preview', props.image);
}
</script>

<style scoped>
.image-card {
  background: #fff;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
  cursor: pointer;
  position: relative;
}

.image-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15);
}

.image-card.selected {
  box-shadow: 0 0 0 3px #409eff;
}

.image-card.is-raw .image-thumbnail::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(135deg, rgba(100, 50, 0, 0.1) 0%, transparent 50%);
  pointer-events: none;
}

.image-thumbnail {
  position: relative;
  aspect-ratio: 4/3;
  overflow: hidden;
  background: #f5f7fa;
}

.image-thumbnail img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.image-card:hover .image-thumbnail img {
  transform: scale(1.05);
}

.thumbnail-placeholder {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #909399;
}

.thumbnail-placeholder .el-icon {
  font-size: 48px;
  margin-bottom: 8px;
}

.loading-text {
  font-size: 12px;
}

.raw-badge {
  position: absolute;
  top: 8px;
  right: 8px;
  background: linear-gradient(135deg, #ff6b6b, #ee5a5a);
  color: white;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 10px;
  font-weight: bold;
  z-index: 2;
}

.checkbox-overlay {
  position: absolute;
  top: 8px;
  left: 8px;
  z-index: 3;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.image-card:hover .checkbox-overlay,
.image-card.selected .checkbox-overlay {
  opacity: 1;
}

.image-info {
  padding: 12px;
}

.image-name {
  font-size: 14px;
  color: #303133;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-bottom: 4px;
}

.image-size {
  font-size: 12px;
  color: #909399;
}
</style>
