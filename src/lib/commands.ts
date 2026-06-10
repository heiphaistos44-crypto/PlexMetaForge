import { invoke } from '@tauri-apps/api/core';
import type {
  InjectionReport,
  MediaItem,
  MetadataPayload,
  Plugin,
  PlexPaths,
  PluginConfig,
  PluginTemplateMeta,
} from './types';

// Config
export const getPlexPaths = (): Promise<PlexPaths> =>
  invoke('get_plex_paths');

export const setPlexToken = (token: string): Promise<void> =>
  invoke('set_plex_token', { token });

// Scanner
export const listPlugins = (): Promise<Plugin[]> =>
  invoke('list_plugins');

export const togglePlugin = (path: string, enable: boolean): Promise<string> =>
  invoke('toggle_plugin', { path, enable });

export const deletePlugin = (path: string): Promise<void> =>
  invoke('delete_plugin', { path });

// Generator
export const createPlugin = (name: string): Promise<string> =>
  invoke('create_plugin', { name });

export const readPluginCode = (path: string): Promise<string> =>
  invoke('read_plugin_code', { path });

export const writePluginCode = (path: string, content: string): Promise<void> =>
  invoke('write_plugin_code', { path, content });

// Database
export const searchPlexDb = (query: string): Promise<MediaItem[]> =>
  invoke('search_plex_db', { query });

// Generator — templates
export const getPluginTemplates = (): Promise<PluginTemplateMeta[]> =>
  invoke('get_plugin_templates');

export const createPluginFromTemplate = (config: PluginConfig): Promise<string> =>
  invoke('create_plugin_from_template', { config });

// Module C — Moteur Hybride
export const injectMetadata = (payload: MetadataPayload): Promise<InjectionReport> =>
  invoke('inject_metadata', { payload });
