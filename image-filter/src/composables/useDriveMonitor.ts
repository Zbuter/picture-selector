import { ref, onMounted } from 'vue';
import { useImageStore } from '../stores/image';
import { getDrives } from './useImageApi';
import { listen } from '@tauri-apps/api/event';

/**
 * 监听驱动器插拔的 Composable
 */
export function useDriveMonitor() {
  const store = useImageStore();
  const isMonitoring = ref(false);
  let unlistenDrive: (() => void) | null = null;

  /**
   * 刷新驱动器列表
   */
  async function refreshDrives() {
    try {
      const drives = await getDrives();
      store.setDrives(drives);
    } catch (error) {
      console.error('Failed to refresh drives:', error);
    }
  }

  /**
   * 处理驱动器变化事件
   */
  function handleDriveChange(event: any) {
    console.log('Drive change event:', event);
    // 驱动器发生变化，刷新列表
    refreshDrives();
    
    const driveInfo = event.payload as { action: string; path?: string };
    if (driveInfo.action === 'added' && driveInfo.path) {
      store.setStatus(`检测到新驱动器：${driveInfo.path}`);
      // 自动加载新驱动器的图片（如果是可移动驱动器）
      setTimeout(() => {
        refreshDrives();
      }, 1000);
    } else if (driveInfo.action === 'removed') {
      store.setStatus('驱动器已移除');
    }
  }

  /**
   * 开始监听驱动器插拔
   */
  async function startMonitoring() {
    if (isMonitoring.value) {
      return;
    }

    try {
      // 先刷新一次驱动器列表
      await refreshDrives();

      // 监听驱动器变化事件
      unlistenDrive = await listen<any>('drive-change', (event) => {
        handleDriveChange(event);
      });

      isMonitoring.value = true;
      console.log('Drive monitoring started');
    } catch (error) {
      console.error('Failed to start drive monitoring:', error);
      isMonitoring.value = false;
    }
  }

  /**
   * 停止监听
   */
  function stopMonitoring() {
    if (unlistenDrive) {
      unlistenDrive();
      unlistenDrive = null;
    }
    isMonitoring.value = false;
  }

  // 组件挂载时自动开始监听
  onMounted(() => {
    startMonitoring();
  });

  return {
    drives: () => store.drives,
    isMonitoring,
    refreshDrives,
    startMonitoring,
    stopMonitoring
  };
}
