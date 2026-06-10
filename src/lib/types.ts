export interface Plugin {
  name: string;
  path: string;
  enabled: boolean;
  has_code: boolean;
}

export interface MediaItem {
  id: number;
  title: string;
  year?: number;
  summary?: string;
  thumb?: string;
}

export interface MetadataPayload {
  title: string;
  year?: number;
  plot?: string;
  poster_url?: string;
  fanart_url?: string;
  tmdb_id?: string;
  imdb_id?: string;
  media_path: string;
}

export interface InjectionReport {
  nfo_written: boolean;
  poster_saved: boolean;
  fanart_saved: boolean;
  plex_api_refreshed: boolean;
  sqlite_updated: boolean;
  errors: string[];
}

export type MetadataStatus = 'complete' | 'incomplete' | 'missing';

export interface MediaNode {
  path: string;
  name: string;
  status: MetadataStatus;
  media_item?: MediaItem;
}

export type PluginTemplateId =
  | 'blank'
  | 'cinema'
  | 'series'
  | 'musique'
  | 'anime'
  | 'universal';

export interface PluginTemplateMeta {
  id: PluginTemplateId;
  label: string;
  icon: string;
  description: string;
  requires_tmdb: boolean;
  requires_lastfm: boolean;
}

export interface PluginConfig {
  name: string;
  template: PluginTemplateId;
  tmdb_api_key?: string;
  lastfm_api_key?: string;
}

export interface PlexPaths {
  plugins_dir: string;
  database_path: string;
  plugins_dir_exists: boolean;
  database_exists: boolean;
}
