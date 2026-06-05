<template>
  <div class="toolbar">
    <!-- 左侧操作区 -->
    <div class="toolbar-left">
      <el-button type="primary" :icon="Folder" @click="handleSelectFolder">
        选择文件夹
      </el-button>
      
      <el-button :icon="Refresh" @click="handleRefresh" :disabled="!store.currentFolder">
        刷新
      </el-button>
      
      <el-button 
        :icon="CopyDocument" 
        @click="handleCopy" 
        :disabled="!store.hasSelection || !store.targetFolder"
        type="success"
      >
        复制选中 ({{ store.selectedCount }})
      </el-button>
    </div>

    <!-- 中间路径显示 -->
    <div class="toolbar-center">
      <el-input
        v-model="currentPath"
        placeholder="当前文件夹路径"
        readonly
        clearable
        @clear="handleClearPath"
      >
        <template #prefix>
          <el-icon><FolderOpened /></el-icon>
        </template>
      </el-input>
    </div>

    <!-- 右侧工具区 -->
    <div class="toolbar-right">
      <!-- 全选/取消全选 -->
      <el-button 
        :icon="store.allSelected ? 'Check' : 'Checked'" 
        @click="handleToggleSelectAll"
        :disabled="!store.images.length"
      >
        {{ store.allSelected ? '取消全选' : '全选' }}
      </el-button>
      
      <!-- 目标文件夹选择 -->
      <el-button :icon="Destination" @click="handleSelectTarget">
        目标文件夹
      </el-button>
      
      <!-- 驱动器状态 -->
      <el-dropdown trigger="click" v-if="store.drives.length > 0">
        <el-button :icon="Monitor">
          外接驱动器 ({{ store.drives.length }})
        </el-button>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item 
              v-for="drive in store.drives" 
              :key="drive.path"
              @click="handleDriveClick(drive)"
            >
              <div class="drive-item">
                <span>{{ drive.name }}</span>
                <span class="drive-path">{{ drive.path }}</span>
                <el-tag size="small" :type="drive.isRemovable ? 'warning' : 'info'">
                  {{ drive.isRemovable ? '可移动' : '固定' }}
                </el-tag>
              </div>
            </el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
      
      <!-- 监听状态 -->
      <el-tooltip :content="store.watcherActive ? '正在监听文件夹变化' : '未监听'">
        <el-button :icon="store.watcherActive ? 'VideoPause' : 'VideoCamera'">
          {{ store.watcherActive ? '监听中' : '监听' }}
        </el-button>
      </el-tooltip>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { 
  Folder, 
  FolderOpened, 
  Refresh, 
  CopyDocument,
  Destination,
  Monitor
} from '@element-plus/icons-vue';
import { useImageStore } from '../../stores/image';
import { selectFolder, selectTargetFolder, loadImages, startWatching } from '../../composables/useImageApi';
import type { DriveInfo } from '../../types';
import { ElMessage, ElMessageBox } from 'element-plus';

const store = useImageStore();

const currentPath = computed({
  get: () => store.currentFolder || '',
  set: (val) => {
    // 只读，不处理 set
  }
});

/**
 * 选择源文件夹
 */
async function handleSelectFolder() {
  try {
    store.setLoading(true);
    const folder = await selectFolder();
    
    if (folder) {
      store.setCurrentFolder(folder);
      const images = await loadImages(folder);
      store.setImages(images);
      
      // 开始监听文件夹变化
      await startWatching(folder);
      
      ElMessage.success(`已加载 ${images.length} 张图片`);
    }
  } catch (error) {
    console.error('Failed to select folder:', error);
    ElMessage.error('选择文件夹失败');
  } finally {
    store.setLoading(false);
  }
}

/**
 * 刷新图片列表
 */
async function handleRefresh() {
  if (!store.currentFolder) return;
  
  try {
    store.setLoading(true);
    const images = await loadImages(store.currentFolder);
    store.setImages(images);
    ElMessage.success(`已刷新 ${images.length} 张图片`);
  } catch (error) {
    console.error('Failed to refresh:', error);
    ElMessage.error('刷新失败');
  } finally {
    store.setLoading(false);
  }
}

/**
 * 清空路径
 */
function handleClearPath() {
  store.setCurrentFolder(null);
  store.setImages([]);
}

/**
 * 切换全选状态
 */
function handleToggleSelectAll() {
  if (store.allSelected) {
    store.deselectAll();
  } else {
    store.selectAll();
  }
}

/**
 * 选择目标文件夹
 */
async function handleSelectTarget() {
  try {
    const folder = await selectTargetFolder();
    
    if (folder) {
      store.setTargetFolder(folder);
      ElMessage.success(`目标文件夹：${folder}`);
    }
  } catch (error) {
    console.error('Failed to select target folder:', error);
    ElMessage.error('选择目标文件夹失败');
  }
}

/**
 * 复制选中的图片
 */
async function handleCopy() {
  if (!store.hasSelection || !store.targetFolder) {
    ElMessage.warning('请选择要复制的图片和目标文件夹');
    return;
  }

  try {
    store.setLoading(true);
    
    const { copySelectedImages } = await import('../../composables/useImageApi');
    const result = await copySelectedImages(
      store.getSelectedImagePaths(),
      store.targetFolder!
    );
    
    if (result.success) {
      ElMessage.success(`成功复制 ${result.copied} 张图片`);
      if (result.failed > 0) {
        ElMessage.warning(`有 ${result.failed} 张图片复制失败`);
      }
    } else {
      ElMessage.error('复制失败');
    }
  } catch (error) {
    console.error('Failed to copy images:', error);
    ElMessage.error('复制图片失败');
  } finally {
    store.setLoading(false);
  }
}

/**
 * 点击驱动器项
 */
function handleDriveClick(drive: DriveInfo) {
  ElMessageBox.confirm(
    `是否要打开驱动器 "${drive.name}" (${drive.path})？`,
    '打开驱动器',
    {
      confirmButtonText: '打开',
      cancelButtonText: '取消',
      type: 'info'
    }
  ).then(async () => {
    try {
      store.setLoading(true);
      store.setCurrentFolder(drive.path);
      const images = await loadImages(drive.path);
      store.setImages(images);
      await startWatching(drive.path);
      ElMessage.success(`已加载 ${images.length} 张图片`);
    } catch (error) {
      console.error('Failed to load drive:', error);
      ElMessage.error('无法加载驱动器内容');
    } finally {
      store.setLoading(false);
    }
  }).catch(() => {
    // 用户取消
  });
}
</script>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px;
  background: #fff;
  border-bottom: 1px solid #e4e7ed;
  flex-wrap: wrap;
}

.toolbar-left,
.toolbar-right {
  display: flex;
  gap: 8px;
  align-items: center;
}

.toolbar-center {
  flex: 1;
  min-width: 300px;
}

.drive-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.drive-path {
  font-size: 12px;
  color: #909399;
}
</style>
