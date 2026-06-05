import { onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { useImageStore } from '../stores/image';
import type { WatchEvent } from '../types';

/**
 * 监听文件系统变化的 Composable
 */
export function useFileWatcher() {
  const store = useImageStore();
  let unlisten: (() => void) | null = null;

  /**
   * 处理文件变化事件
   */
  function handleWatchEvent(event: WatchEvent) {
    console.log('File system event:', event);

    switch (event.type) {
      case 'create':
        // 新文件创建，需要重新加载或单独添加
        store.setStatus(`新文件创建：${event.path}`);
        break;
      case 'modify':
        // 文件修改，可能需要刷新缩略图
        store.setStatus(`文件修改：${event.path}`);
        break;
      case 'delete':
        // 文件删除，从列表中移除
        store.removeImage(event.path);
        store.setStatus(`文件已删除：${event.path}`);
        break;
      case 'rename':
        // 文件重命名
        store.setStatus(`文件重命名：${event.path}`);
        break;
    }
  }

  /**
   * 开始监听
   */
  async function startWatching() {
    if (!store.currentFolder || store.watcherActive) {
      return;
    }

    try {
      unlisten = await listen<WatchEvent>('file-change', (event) => {
        handleWatchEvent(event.payload);
      });
      store.setWatcherActive(true);
      store.setStatus('已开始监听文件夹变化');
    } catch (error) {
      console.error('Failed to start file watcher:', error);
      store.setStatus('无法监听文件夹变化');
    }
  }

  /**
   * 停止监听
   */
  async function stopWatching() {
    if (unlisten) {
      unlisten();
      unlisten = null;
    }
    store.setWatcherActive(false);
    store.setStatus('已停止监听');
  }

  // 组件挂载时自动开始监听（如果有当前文件夹）
  onMounted(() => {
    if (store.currentFolder) {
      startWatching();
    }
  });

  // 组件卸载时停止监听
  onUnmounted(() => {
    stopWatching();
  });

  return {
    startWatching,
    stopWatching,
    isWatching: () => store.watcherActive
  };
}
