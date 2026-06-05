<template>
  <div class="app-container">
    <!-- 顶部工具栏 -->
    <Toolbar />
    
    <!-- 图片网格区域 -->
    <ImageGrid />
    
    <!-- 状态栏 -->
    <div class="status-bar" v-if="store.statusMessage">
      <el-icon><InfoFilled /></el-icon>
      <span>{{ store.statusMessage }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { InfoFilled } from '@element-plus/icons-vue';
import { useImageStore } from './stores/image';
import { useDriveMonitor } from './composables/useDriveMonitor';
import Toolbar from './components/Toolbar.vue';
import ImageGrid from './components/ImageGrid.vue';

const store = useImageStore();

// 使用驱动器监听
useDriveMonitor();

// 组件挂载时刷新驱动器列表
onMounted(() => {
  console.log('Image Filter App mounted');
});
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  width: 100%;
  height: 100%;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.status-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  background: #ecf5ff;
  border-top: 1px solid #d9ecff;
  color: #409eff;
  font-size: 14px;
}

.status-bar .el-icon {
  font-size: 16px;
}
</style>
