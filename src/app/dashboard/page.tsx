'use client';

import { useEffect, useState, useCallback } from 'react';
import { useRouter } from 'next/navigation';
import type { Plugin, PlexPaths } from '@/lib/types';
import {
  getPlexPaths,
  listPlugins,
  togglePlugin,
  deletePlugin,
} from '@/lib/commands';
import PluginCreator from '@/components/PluginCreator';

type Tab = 'list' | 'create';

export default function DashboardPage() {
  const router = useRouter();
  const [tab, setTab] = useState<Tab>('list');
  const [plugins, setPlugins] = useState<Plugin[]>([]);
  const [paths, setPaths] = useState<PlexPaths | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState('');

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

  useEffect(() => { loadData(); }, [loadData]);

  const handleToggle = async (plugin: Plugin) => {
    try {
      await togglePlugin(plugin.path, !plugin.enabled);
      await loadData();
    } catch (e) { setError(String(e)); }
  };

  const handleDelete = async (plugin: Plugin) => {
    if (!confirm(`Supprimer "${plugin.name}" ? Une sauvegarde sera créée.`)) return;
    try {
      await deletePlugin(plugin.path);
      await loadData();
    } catch (e) { setError(String(e)); }
  };

  const handleEdit = (plugin: Plugin) => {
    router.push(`/code-editor?path=${encodeURIComponent(plugin.path)}`);
  };

  const filtered = plugins.filter((p) =>
    p.name.toLowerCase().includes(filter.toLowerCase())
  );

  return (
    <div className="p-6 space-y-5">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold text-plex-text">Plugins Plex</h1>
        <button
          onClick={loadData}
          className="text-sm px-3 py-1.5 rounded bg-plex-surface border border-plex-border text-plex-muted hover:text-plex-text transition-colors"
        >
          ↻ Actualiser
        </button>
      </div>

      {/* Plex paths status */}
      {paths && (
        <div className="text-xs text-plex-muted bg-plex-surface rounded p-3 border border-plex-border space-y-1">
          <div>
            <span className={paths.plugins_dir_exists ? 'text-green-400' : 'text-red-400'}>●</span>
            {' '}Plug-ins : <span className="font-mono">{paths.plugins_dir}</span>
          </div>
          <div>
            <span className={paths.database_exists ? 'text-green-400' : 'text-red-400'}>●</span>
            {' '}Base SQLite : <span className="font-mono">{paths.database_path}</span>
          </div>
        </div>
      )}

      {error && (
        <div className="text-sm text-red-400 bg-red-900/20 border border-red-800 rounded p-3">
          {error}
        </div>
      )}

      {/* Tabs */}
      <div className="flex gap-1 bg-plex-surface p-1 rounded border border-plex-border">
        <TabBtn label={`Mes plugins (${plugins.length})`} active={tab === 'list'} onClick={() => setTab('list')} />
        <TabBtn label="Créer un plugin" active={tab === 'create'} onClick={() => setTab('create')} accent />
      </div>

      {/* Tab: List */}
      {tab === 'list' && (
        <div className="space-y-3">
          {/* Filtre */}
          <input
            type="text"
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            placeholder="Filtrer les plugins…"
            className="w-full bg-plex-surface border border-plex-border rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-plex-accent"
          />

          {loading ? (
            <div className="text-plex-muted text-sm text-center py-12">Chargement…</div>
          ) : filtered.length === 0 ? (
            <div className="text-plex-muted text-sm text-center py-12">
              {filter ? 'Aucun plugin ne correspond.' : 'Aucun plugin trouvé.'}
              <br />
              <button onClick={() => setTab('create')} className="mt-2 text-plex-accent underline text-xs">
                Créer ton premier plugin
              </button>
            </div>
          ) : (
            <div className="space-y-2">
              {filtered.map((plugin) => (
                <div
                  key={plugin.path}
                  className="flex items-center gap-3 bg-plex-surface border border-plex-border rounded px-4 py-3"
                >
                  <span className={`w-2 h-2 rounded-full flex-shrink-0 ${plugin.enabled ? 'bg-green-400' : 'bg-red-500'}`} />
                  <div className="flex-1 min-w-0">
                    <div className="text-sm font-medium text-plex-text truncate">{plugin.name}</div>
                    <div className="text-xs text-plex-muted mt-0.5">
                      {plugin.enabled ? 'Actif' : 'Désactivé'}
                      {plugin.has_code && ' · Python'}
                    </div>
                  </div>
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
      )}

      {/* Tab: Create */}
      {tab === 'create' && (
        <div className="bg-plex-surface border border-plex-border rounded p-5">
          <PluginCreator
            onCreated={async () => {
              await loadData();
              setTab('list');
            }}
          />
        </div>
      )}
    </div>
  );
}

function TabBtn({
  label,
  active,
  onClick,
  accent,
}: {
  label: string;
  active: boolean;
  onClick: () => void;
  accent?: boolean;
}) {
  return (
    <button
      onClick={onClick}
      className={`flex-1 text-sm py-1.5 rounded transition-colors font-medium ${
        active
          ? accent
            ? 'bg-plex-accent text-black'
            : 'bg-plex-border text-plex-text'
          : 'text-plex-muted hover:text-plex-text'
      }`}
    >
      {label}
    </button>
  );
}
