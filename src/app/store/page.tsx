'use client';

import { useEffect, useState, useCallback } from 'react';
import type { InstallResult, StoreCategory, StorePlugin } from '@/lib/types';
import { getStoreCatalog, getStoreCategories, installStorePlugin, getInstalledPluginIds } from '@/lib/commands';

type Filter = { category: string; subcategory: string };

const EMOJI: Record<string, string> = {
  // Catégories principales
  'Métadonnées': '🏷️', 'Sous-titres': '💬', 'Outils': '🛠️',
  // Métadonnées sous-catégories
  'Films': '🎬', 'Séries TV': '📺', 'Anime / Manga': '⛩️',
  'Musique': '🎵', 'Concerts & Lives': '🎤', 'Audiobooks': '📚',
  'Podcasts': '🎙️', 'Comics & BD': '📖', 'Jeux Vidéo': '🎮', 'Sports': '⚽',
  // Sous-titres sous-catégories
  'Multi-langues': '🌍', 'Français': '🇫🇷', 'Autre langue': '🗣️',
  // Outils sous-catégories
  'Utilitaires': '🔧', 'Scanners': '🔍', 'IPTV': '📡', 'Sync': '🔄',
};

export default function StorePage() {
  const [plugins, setPlugins] = useState<StorePlugin[]>([]);
  const [categories, setCategories] = useState<StoreCategory[]>([]);
  const [installed, setInstalled] = useState<Set<string>>(new Set());
  const [filter, setFilter] = useState<Filter>({ category: 'Tous', subcategory: '' });
  const [search, setSearch] = useState('');
  const [expanded, setExpanded] = useState<string | null>('Métadonnées');
  const [loading, setLoading] = useState(true);
  const [installing, setInstalling] = useState<string | null>(null);
  const [results, setResults] = useState<Record<string, { ok: boolean; msg: string }>>({});

  const loadData = useCallback(async () => {
    setLoading(true);
    try {
      const [catalog, cats, ids] = await Promise.all([
        getStoreCatalog(),
        getStoreCategories(),
        getInstalledPluginIds(),
      ]);
      setPlugins(catalog);
      setCategories(cats);
      setInstalled(new Set(ids));
    } catch (e) { console.error(e); }
    finally { setLoading(false); }
  }, []);

  useEffect(() => { loadData(); }, [loadData]);

  const handleInstall = async (plugin: StorePlugin) => {
    setInstalling(plugin.id);
    setResults((r) => ({ ...r, [plugin.id]: { ok: false, msg: '' } }));
    try {
      const res: InstallResult = await installStorePlugin(plugin.zip_url, plugin.bundle_name);
      const msg = res.already_existed
        ? `✓ Mis à jour : ${res.bundle_name}`
        : `✓ Installé : ${res.bundle_name}`;
      setResults((r) => ({ ...r, [plugin.id]: { ok: true, msg } }));
      getInstalledPluginIds().then((ids) => setInstalled(new Set(ids)));
    } catch (e) {
      setResults((r) => ({ ...r, [plugin.id]: { ok: false, msg: String(e) } }));
    } finally {
      setInstalling(null);
    }
  };

  const isInstalled = (plugin: StorePlugin) => {
    const base = plugin.bundle_name.toLowerCase()
      .replace('.bundle', '').replace('.disabled', '');
    return [...installed].some((id) => {
      const idBase = id.toLowerCase().replace('.bundle', '').replace('.disabled', '');
      return idBase === base;
    });
  };

  const filtered = plugins.filter((p) => {
    if (filter.category !== 'Tous') {
      if (p.category !== filter.category) return false;
      if (filter.subcategory && p.subcategory !== filter.subcategory) return false;
    }
    if (search) {
      const q = search.toLowerCase();
      return (
        p.name.toLowerCase().includes(q) ||
        p.description.toLowerCase().includes(q) ||
        p.tags.some((t) => t.toLowerCase().includes(q))
      );
    }
    return true;
  });

  const selectFilter = (category: string, subcategory = '') => {
    setFilter({ category, subcategory });
  };

  return (
    <div className="flex h-full">
      {/* Sidebar navigation */}
      <aside className="w-52 flex-shrink-0 bg-plex-surface border-r border-plex-border overflow-y-auto">
        <div className="p-3 space-y-1">
          {/* Tous */}
          <button
            onClick={() => selectFilter('Tous')}
            className={`w-full text-left px-3 py-2 rounded text-sm transition-colors font-medium ${
              filter.category === 'Tous'
                ? 'bg-plex-accent text-black'
                : 'text-plex-muted hover:text-plex-text hover:bg-plex-border'
            }`}
          >
            🌐 Tous ({plugins.length})
          </button>

          {/* Catégories */}
          {categories.map((cat) => {
            const isOpen = expanded === cat.name;
            const catCount = plugins.filter((p) => p.category === cat.name).length;
            return (
              <div key={cat.name}>
                <button
                  onClick={() => {
                    setExpanded(isOpen ? null : cat.name);
                    selectFilter(cat.name);
                  }}
                  className={`w-full text-left px-3 py-2 rounded text-sm transition-colors font-medium flex items-center justify-between ${
                    filter.category === cat.name && !filter.subcategory
                      ? 'bg-plex-border text-plex-text'
                      : 'text-plex-muted hover:text-plex-text hover:bg-plex-border'
                  }`}
                >
                  <span>{EMOJI[cat.name] ?? '📁'} {cat.name}</span>
                  <span className="text-xs opacity-60 flex items-center gap-1">
                    {catCount}
                    <span className={`transition-transform ${isOpen ? 'rotate-90' : ''}`}>›</span>
                  </span>
                </button>

                {/* Sous-catégories */}
                {isOpen && (
                  <div className="ml-3 mt-0.5 space-y-0.5">
                    {cat.subcategories.map((sub) => {
                      const subCount = plugins.filter(
                        (p) => p.category === cat.name && p.subcategory === sub
                      ).length;
                      return (
                        <button
                          key={sub}
                          onClick={() => selectFilter(cat.name, sub)}
                          className={`w-full text-left px-3 py-1.5 rounded text-xs transition-colors flex items-center justify-between ${
                            filter.subcategory === sub
                              ? 'bg-plex-accent/20 text-plex-accent border border-plex-accent/30'
                              : 'text-plex-muted hover:text-plex-text hover:bg-plex-border'
                          }`}
                        >
                          <span>{EMOJI[sub] ?? '·'} {sub}</span>
                          <span className="opacity-60">{subCount}</span>
                        </button>
                      );
                    })}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </aside>

      {/* Main content */}
      <div className="flex-1 overflow-y-auto p-5 space-y-4">
        {/* Header + search */}
        <div className="flex items-center gap-3">
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Rechercher un plugin…"
            className="flex-1 bg-plex-surface border border-plex-border rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-plex-accent"
          />
          <button onClick={loadData}
            className="text-xs px-3 py-2 rounded bg-plex-surface border border-plex-border text-plex-muted hover:text-plex-text transition-colors">
            ↻
          </button>
        </div>

        {/* Breadcrumb */}
        <div className="text-xs text-plex-muted">
          {filter.category === 'Tous' ? 'Tous les plugins' : filter.category}
          {filter.subcategory && <span> › {filter.subcategory}</span>}
          <span className="ml-2 text-plex-accent">{filtered.length} plugin{filtered.length > 1 ? 's' : ''}</span>
        </div>

        {/* Plugin list */}
        {loading ? (
          <div className="text-plex-muted text-sm text-center py-16">Chargement…</div>
        ) : filtered.length === 0 ? (
          <div className="text-plex-muted text-sm text-center py-16">Aucun plugin trouvé.</div>
        ) : (
          <div className="space-y-3">
            {filtered.map((plugin) => {
              const inst = isInstalled(plugin);
              const res = results[plugin.id];
              const busy = installing === plugin.id;
              const disabled = busy || (installing !== null && !busy);

              return (
                <div key={plugin.id}
                  className="bg-plex-surface border border-plex-border rounded p-4 space-y-2.5">
                  {/* Header */}
                  <div className="flex items-start gap-3">
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 flex-wrap">
                        <span className="text-sm font-bold text-plex-text">{plugin.name}</span>
                        {plugin.verified && (
                          <span className="text-xs px-1.5 py-0.5 rounded bg-green-900/30 text-green-400 border border-green-800/40">✓ Vérifié</span>
                        )}
                        {inst && (
                          <span className="text-xs px-1.5 py-0.5 rounded bg-blue-900/30 text-blue-400 border border-blue-800/40">Installé</span>
                        )}
                        <span className="text-xs px-1.5 py-0.5 rounded bg-plex-bg border border-plex-border text-plex-muted">
                          {EMOJI[plugin.subcategory] ?? ''} {plugin.subcategory}
                        </span>
                      </div>
                      <div className="text-xs text-plex-muted mt-0.5">
                        par <span className="text-plex-text">{plugin.author}</span>
                        {' · '}⭐ {plugin.stars} · {plugin.license}
                      </div>
                    </div>

                    {/* Actions */}
                    <div className="flex gap-2 flex-shrink-0">
                      <a href={plugin.github_url} target="_blank" rel="noreferrer"
                        className="text-xs px-2 py-1.5 rounded bg-plex-bg border border-plex-border text-plex-muted hover:text-plex-text transition-colors">
                        GitHub
                      </a>
                      {plugin.verified !== false && (
                        <button
                          onClick={() => handleInstall(plugin)}
                          disabled={disabled}
                          className={`text-xs px-3 py-1.5 rounded font-semibold transition-colors disabled:opacity-40 ${
                            inst
                              ? 'bg-blue-900/40 text-blue-400 hover:bg-blue-900/60'
                              : 'bg-plex-accent text-black hover:bg-yellow-400'
                          }`}>
                          {busy ? '⬇ Installation…' : inst ? '↻ Mettre à jour' : '⬇ Installer'}
                        </button>
                      )}
                    </div>
                  </div>

                  {/* Description — info card styled differently */}
                  {plugin.verified === false ? (
                    <div className="text-xs text-orange-300 bg-orange-900/20 border border-orange-700/40 rounded p-2.5 leading-relaxed">
                      ℹ️ {plugin.description}
                    </div>
                  ) : (
                    <p className="text-xs text-plex-muted leading-relaxed">{plugin.description}</p>
                  )}

                  {/* Tags */}
                  <div className="flex gap-1 flex-wrap">
                    {plugin.tags.map((tag) => (
                      <span key={tag} className="text-xs px-2 py-0.5 rounded-full bg-plex-bg border border-plex-border text-plex-muted">
                        {tag}
                      </span>
                    ))}
                  </div>

                  {/* Résultat */}
                  {res?.msg && (
                    <div className={`text-xs rounded p-2 ${
                      res.ok
                        ? 'text-green-400 bg-green-900/20 border border-green-800/40'
                        : 'text-red-400 bg-red-900/20 border border-red-800/40'
                    }`}>
                      {res.msg}
                    </div>
                  )}

                  <div className="text-xs font-mono text-plex-muted opacity-50">→ {plugin.bundle_name}</div>
                </div>
              );
            })}
          </div>
        )}

        {/* Note sur le plugin Universel */}
        {(filter.subcategory === 'Anime / Manga' || filter.category === 'Tous') && (
          <div className="text-xs text-plex-muted bg-plex-surface border border-plex-border rounded p-3 space-y-1">
            <div className="font-semibold text-plex-text">ℹ️ À propos du plugin Universel (PlexMetaForge)</div>
            <div>Le plugin <strong>Universel</strong> créé dans "Créer un plugin" couvre : films (TMDB), séries TV (TMDB), anime/manga (AniList, sans clé), musique artiste + album (Last.fm).</div>
            <div className="text-orange-400">⚠ Les sous-titres ne sont pas gérés par les agents de métadonnées Plex — ils nécessitent un plugin dédié comme <strong>Sub-Zero</strong> (ci-dessus).</div>
          </div>
        )}
      </div>
    </div>
  );
}
