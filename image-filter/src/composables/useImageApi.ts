import { invoke } from '@tauri-apps/api/core';
import type { ImageFile, DriveInfo } from '../types';

/**
 * 选择文件夹并获取图片列表
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
 * 加载指定文件夹的图片
 */
export async function loadImages(folderPath: string): Promise<ImageFile[]> {
  try {
    const images = await invoke<ImageFile[]>('load_images', { path: folderPath });
    return images;
  } catch (error) {
    console.error('Failed to load images:', error);
    throw error;
  }
}

/**
 * 生成图片缩略图
 */
export async function generateThumbnail(imagePath: string, width: number = 200): Promise<string> {
  try {
    const thumbnail = await invoke<string>('generate_thumbnail', { path: imagePath, width });
    return thumbnail;
  } catch (error) {
    console.error('Failed to generate thumbnail:', error);
    throw error;
  }
}

/**
 * 获取外接驱动器列表
 */
export async function getDrives(): Promise<DriveInfo[]> {
  try {
    const drives = await invoke<DriveInfo[]>('get_drives');
    return drives;
  } catch (error) {
    console.error('Failed to get drives:', error);
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
 * 复制选中的图片到目标文件夹
 */
export async function copySelectedImages(
  sourcePaths: string[],
  targetFolder: string
): Promise<{ success: boolean; copied: number; failed: number; errors: string[] }> {
  try {
    const result = await invoke<{ success: boolean; copied: number; failed: number; errors: string[] }>(
      'copy_images',
      { sourcePaths, targetFolder }
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
    await invoke('start_watching', { path: folderPath });
  } catch (error) {
    console.error('Failed to start watching:', error);
    throw error;
  }
}

/**
 * 停止监听文件夹变化
 */
export async function stopWatching(): Promise<void> {
  try {
    await invoke('stop_watching');
  } catch (error) {
    console.error('Failed to stop watching:', error);
    throw error;
  }
}

/**
 * 读取图片的元数据（EXIF 等）
 */
export async function getImageMetadata(imagePath: string): Promise<Record<string, any>> {
  try {
    const metadata = await invoke<Record<string, any>>('get_image_metadata', { path: imagePath });
    return metadata;
  } catch (error) {
    console.error('Failed to get image metadata:', error);
    return {};
  }
}
