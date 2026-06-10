'use client';

import { useEffect, useState, useCallback } from 'react';
import { useRouter } from 'next/navigation';
import type { Plugin, PlexPaths } from '@/lib/types';
import {
  getPlexPaths,
  listPlugins,
  togglePlugin,
  deletePlugin,
  createPlugin,
} from '@/lib/commands';

export default function DashboardPage() {
  const router = useRouter();
  const [plugins, setPlugins] = useState<Plugin[]>([]);
  const [paths, setPaths] = useState<PlexPaths | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [newName, setNewName] = useState('');
  const [creating, setCreating] = useState(false);

  const loadData = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [p, list] = await Promise.all([getPlexPaths(), listPlugins()]);
      setPaths(p);
      setPlugins(list);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadData();
  }, [loadData]);

  const handleToggle = async (plugin: Plugin) => {
    try {
      await togglePlugin(plugin.path, !plugin.enabled);
      await loadData();
    } catch (e) {
      setError(String(e));
    }
  };

  const handleDelete = async (plugin: Plugin) => {
    if (!confirm(`Supprimer "${plugin.name}" ? Une sauvegarde sera créée.`)) return;
    try {
      await deletePlugin(plugin.path);
      await loadData();
    } catch (e) {
      setError(String(e));
    }
  };

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newName.trim()) return;
    setCreating(true);
    try {
      await createPlugin(newName.trim());
      setNewName('');
      await loadData();
    } catch (e) {
      setError(String(e));
    } finally {
      setCreating(false);
    }
  };

  const handleEdit = (plugin: Plugin) => {
    router.push(`/code-editor?path=${encodeURIComponent(plugin.path)}`);
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-full text-plex-muted">
        Chargement...
      </div>
    );
  }

  return (
    <div className="p-6 space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold text-plex-text">Plugins Plex</h1>
        <button
          onClick={loadData}
          className="text-sm px-3 py-1.5 rounded bg-plex-surface border border-plex-border text-plex-muted hover:text-plex-text transition-colors"
        >
          Actualiser
        </button>
      </div>

      {/* Paths info */}
      {paths && (
        <div className="text-xs text-plex-muted bg-plex-surface rounded p-3 border border-plex-border">
          <div>
            <span className={paths.plugins_dir_exists ? 'text-green-400' : 'text-red-400'}>●</span>
            {' '}Plug-ins : {paths.plugins_dir}
          </div>
          <div className="mt-1">
            <span className={paths.database_exists ? 'text-green-400' : 'text-red-400'}>●</span>
            {' '}Base : {paths.database_path}
          </div>
        </div>
      )}

      {/* Error */}
      {error && (
        <div className="text-sm text-red-400 bg-red-900/20 border border-red-800 rounded p-3">
          {error}
        </div>
      )}

      {/* Create plugin */}
      <form onSubmit={handleCreate} className="flex gap-2">
        <input
          type="text"
          value={newName}
          onChange={(e) => setNewName(e.target.value)}
          placeholder="Nom du nouveau plugin..."
          className="flex-1 bg-plex-surface border border-plex-border rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-plex-accent"
        />
        <button
          type="submit"
          disabled={creating || !newName.trim()}
          className="px-4 py-2 bg-plex-accent text-black font-semibold text-sm rounded hover:bg-yellow-400 disabled:opacity-50 transition-colors"
        >
          {creating ? '...' : '+ Créer'}
        </button>
      </form>

      {/* Plugin list */}
      {plugins.length === 0 ? (
        <div className="text-plex-muted text-sm text-center py-12">
          Aucun plugin trouvé dans {paths?.plugins_dir ?? 'le dossier Plug-ins'}
        </div>
      ) : (
        <div className="space-y-2">
          {plugins.map((plugin) => (
            <div
              key={plugin.path}
              className="flex items-center gap-3 bg-plex-surface border border-plex-border rounded px-4 py-3"
            >
              {/* Status dot */}
              <span
                className={`w-2 h-2 rounded-full flex-shrink-0 ${
                  plugin.enabled ? 'bg-green-400' : 'bg-red-500'
                }`}
              />

              {/* Name */}
              <div className="flex-1 min-w-0">
                <div className="text-sm font-medium text-plex-text truncate">
                  {plugin.name}
                </div>
                <div className="text-xs text-plex-muted mt-0.5">
                  {plugin.enabled ? 'Actif' : 'Désactivé'}
                  {plugin.has_code && ' · Python'}
                </div>
              </div>

              {/* Actions */}
              <div className="flex gap-2 flex-shrink-0">
                {plugin.has_code && (
                  <button
                    onClick={() => handleEdit(plugin)}
                    className="text-xs px-2 py-1 rounded bg-plex-border text-plex-muted hover:text-plex-text transition-colors"
                  >
                    Éditer
                  </button>
                )}
                <button
                  onClick={() => handleToggle(plugin)}
                  className={`text-xs px-2 py-1 rounded transition-colors ${
                    plugin.enabled
                      ? 'bg-orange-900/40 text-orange-400 hover:bg-orange-900/60'
                      : 'bg-green-900/40 text-green-400 hover:bg-green-900/60'
                  }`}
                >
                  {plugin.enabled ? 'Désactiver' : 'Activer'}
                </button>
                <button
                  onClick={() => handleDelete(plugin)}
                  className="text-xs px-2 py-1 rounded bg-red-900/30 text-red-400 hover:bg-red-900/50 transition-colors"
                >
                  Supprimer
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
