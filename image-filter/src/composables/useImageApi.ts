import { invoke } from '@tauri-apps/api/core';
import type { ImageFile, DriveInfo } from '../types';

/**
 * 选择源文件夹
 */
export async function selectFolder(): Promise<string | null> {
  try {
    const folder = await invoke<string>('select_folder');
    return folder;
  } catch (error) {
    console.error('Failed to select folder:', error);
    throw error;
  }
}

/**
 * 选择目标文件夹
 */
export async function selectTargetFolder(): Promise<string | null> {
  try {
    const folder = await invoke<string>('select_target_folder');
    return folder;
  } catch (error) {
    console.error('Failed to select target folder:', error);
    throw error;
  }
}

/**
 * 加载指定文件夹的图片
 */
export async function loadImages(folderPath: string): Promise<ImageFile[]> {
  try {
    const images = await invoke<ImageFile[]>('list_images_in_folder', { folderPath });
    return images;
  } catch (error) {
    console.error('Failed to load images:', error);
    throw error;
  }
}

/**
 * 获取图片预览（base64）
 */
export async function getImagePreview(imagePath: string): Promise<string> {
  try {
    const preview = await invoke<string>('get_image_preview', { path: imagePath });
    return preview;
  } catch (error) {
    console.error('Failed to get image preview:', error);
    throw error;
  }
}

/**
 * 获取外接驱动器列表
 */
export async function getDrives(): Promise<string[]> {
  try {
    const drives = await invoke<string[]>('detect_external_drives');
    return drives;
  } catch (error) {
    console.error('Failed to get drives:', error);
    throw error;
  }
}

/**
 * 复制选中的图片到目标文件夹
 */
export async function copySelectedImages(
  sourcePaths: string[],
  targetFolder: string
): Promise<{ success: number; failed: number; errors: string[] }> {
  try {
    const result = await invoke<{ success: number; failed: number; errors: string[] }>(
      'copy_selected_images',
      { sourcePaths, destFolder: targetFolder }
    );
    return result;
  } catch (error) {
    console.error('Failed to copy images:', error);
    throw error;
  }
}

/**
 * 开始监听文件夹变化
 */
export async function startWatching(folderPath: string): Promise<void> {
  try {
    await invoke('start_watching_folder', { folderPath });
  } catch (error) {
    console.error('Failed to start watching:', error);
    throw error;
  }
}

/**
 * 停止监听文件夹变化 - 目前通过重新选择文件夹来切换
 */
export async function stopWatching(): Promise<void> {
  // 当前实现中，watcher 会在选择新文件夹时自动替换
  // 如需显式停止，可以在 Rust 端添加 stop_watching 命令
  console.log('Watching will stop when selecting a new folder');
}
