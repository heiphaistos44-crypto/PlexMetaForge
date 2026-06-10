'use client';

import { useEffect, useState, useCallback } from 'react';
import type { DatabaseStats, MediaItem, PlexSection } from '@/lib/types';
import {
  getDbStats,
  getDbSections,
  getItemsBySection,
  getIncompleteItems,
  dbBatchClearLocks,
  searchPlexDb,
} from '@/lib/commands';

type ViewMode = 'stats' | 'sections' | 'incomplete' | 'search';

export default function DatabasePage() {
  const [mode, setMode] = useState<ViewMode>('stats');
  const [stats, setStats] = useState<DatabaseStats | null>(null);
  const [sections, setSections] = useState<PlexSection[]>([]);
  const [items, setItems] = useState<MediaItem[]>([]);
  const [selectedSection, setSelectedSection] = useState<number | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);

  const loadStats = useCallback(async () => {
    setLoading(true); setError(null);
    try {
      const [s, sec] = await Promise.all([getDbStats(), getDbSections()]);
      setStats(s); setSections(sec);
    } catch (e) { setError(String(e)); }
    finally { setLoading(false); }
  }, []);

  useEffect(() => { loadStats(); }, [loadStats]);

  const loadSection = async (id: number) => {
    setSelectedSection(id); setLoading(true); setError(null);
    try { setItems(await getItemsBySection(id, 200)); }
    catch (e) { setError(String(e)); }
    finally { setLoading(false); }
  };

  const loadIncomplete = async () => {
    setMode('incomplete'); setLoading(true); setError(null);
    try { setItems(await getIncompleteItems(100)); }
    catch (e) { setError(String(e)); }
    finally { setLoading(false); }
  };

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!searchQuery.trim()) return;
    setLoading(true); setError(null);
    try { setItems(await searchPlexDb(searchQuery.trim())); }
    catch (e) { setError(String(e)); }
    finally { setLoading(false); }
  };

  const handleClearLocks = async () => {
    try {
      const r = await dbBatchClearLocks();
      setNotice(`${r.updated} verrou(s) libéré(s).`);
      setTimeout(() => setNotice(null), 3000);
    } catch (e) { setError(String(e)); }
  };

  return (
    <div className="p-6 space-y-5">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold text-plex-text">Base de données Plex</h1>
        <div className="flex gap-2">
          <button onClick={handleClearLocks}
            className="text-xs px-3 py-1.5 rounded bg-orange-900/30 text-orange-400 border border-orange-800/40 hover:bg-orange-900/50 transition-colors">
            Libérer verrous
          </button>
          <button onClick={loadStats}
            className="text-xs px-3 py-1.5 rounded bg-plex-surface border border-plex-border text-plex-muted hover:text-plex-text transition-colors">
            ↻ Actualiser
          </button>
        </div>
      </div>

      {error && <div className="text-sm text-red-400 bg-red-900/20 border border-red-800 rounded p-3">{error}</div>}
      {notice && <div className="text-sm text-green-400 bg-green-900/20 border border-green-800/40 rounded p-3">{notice}</div>}

      {/* Mode tabs */}
      <div className="flex gap-1 bg-plex-surface p-1 rounded border border-plex-border">
        {(['stats','sections','incomplete','search'] as ViewMode[]).map((m) => (
          <button key={m} onClick={() => { setMode(m); if (m === 'incomplete') loadIncomplete(); }}
            className={`flex-1 text-xs py-1.5 rounded transition-colors font-medium capitalize ${
              mode === m ? 'bg-plex-border text-plex-text' : 'text-plex-muted hover:text-plex-text'}`}>
            {{ stats:'Statistiques', sections:'Bibliothèques', incomplete:'Incomplets', search:'Recherche' }[m]}
          </button>
        ))}
      </div>

      {/* Stats */}
      {mode === 'stats' && stats && (
        <div className="grid grid-cols-2 gap-3">
          <StatCard label="Total items" value={stats.total_items} />
          <StatCard label="Films" value={stats.movies} color="blue" />
          <StatCard label="Séries" value={stats.shows} color="purple" />
          <StatCard label="Épisodes" value={stats.episodes} color="purple" />
          <StatCard label="Artistes" value={stats.artists} color="pink" />
          <StatCard label="Albums" value={stats.albums} color="pink" />
          <StatCard label="Pistes" value={stats.tracks} color="pink" />
          <StatCard label="Bibliothèques" value={stats.sections} color="yellow" />
          <StatCard label="Avec poster" value={stats.items_with_thumb} color="green" />
          <StatCard label="Sans synopsis" value={stats.items_without_summary} color="orange" />
        </div>
      )}

      {/* Sections */}
      {mode === 'sections' && (
        <div className="space-y-2">
          {sections.map((s) => (
            <button key={s.id} onClick={() => { setMode('sections'); loadSection(s.id); }}
              className={`w-full text-left px-4 py-3 rounded border transition-colors ${
                selectedSection === s.id
                  ? 'bg-plex-accent/10 border-plex-accent/40 text-plex-text'
                  : 'bg-plex-surface border-plex-border text-plex-muted hover:text-plex-text'}`}>
              <div className="text-sm font-medium">{s.name}</div>
              <div className="text-xs mt-0.5">{s.section_type}{s.location ? ` · ${s.location}` : ''}</div>
            </button>
          ))}
          {selectedSection && items.length > 0 && (
            <div className="mt-3 space-y-1 max-h-96 overflow-y-auto">
              {items.map((item) => <ItemRow key={item.id} item={item} />)}
            </div>
          )}
        </div>
      )}

      {/* Incomplete */}
      {mode === 'incomplete' && (
        <div className="space-y-1 max-h-[60vh] overflow-y-auto">
          {loading && <div className="text-plex-muted text-sm text-center py-8">Chargement…</div>}
          {!loading && items.length === 0 && (
            <div className="text-green-400 text-sm text-center py-8">
              ✓ Tous les items ont une synopsis et un poster.
            </div>
          )}
          {items.map((item) => <ItemRow key={item.id} item={item} showMissing />)}
        </div>
      )}

      {/* Search */}
      {mode === 'search' && (
        <div className="space-y-3">
          <form onSubmit={handleSearch} className="flex gap-2">
            <input type="text" value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Titre du média…"
              className="flex-1 bg-plex-bg border border-plex-border rounded px-3 py-2 text-sm text-plex-text placeholder-plex-muted focus:outline-none focus:border-plex-accent" />
            <button type="submit" disabled={loading || !searchQuery.trim()}
              className="px-4 py-2 bg-plex-accent text-black font-semibold text-sm rounded hover:bg-yellow-400 disabled:opacity-50 transition-colors">
              {loading ? '…' : 'Chercher'}
            </button>
          </form>
          <div className="space-y-1 max-h-[55vh] overflow-y-auto">
            {items.map((item) => <ItemRow key={item.id} item={item} />)}
          </div>
        </div>
      )}
    </div>
  );
}

function StatCard({ label, value, color }: { label: string; value: number; color?: string }) {
  const colors: Record<string, string> = {
    blue: 'text-blue-400', purple: 'text-purple-400', pink: 'text-pink-400',
    yellow: 'text-yellow-400', green: 'text-green-400', orange: 'text-orange-400',
  };
  return (
    <div className="bg-plex-surface border border-plex-border rounded p-3 flex items-center justify-between">
      <span className="text-xs text-plex-muted">{label}</span>
      <span className={`text-lg font-bold ${color ? colors[color] : 'text-plex-text'}`}>
        {value.toLocaleString()}
      </span>
    </div>
  );
}

function ItemRow({ item, showMissing }: { item: MediaItem; showMissing?: boolean }) {
  const missing = showMissing ? [
    !item.summary && 'synopsis',
    !item.thumb && 'poster',
  ].filter(Boolean) : [];

  return (
    <div className="flex items-center gap-3 px-3 py-2 bg-plex-surface border border-plex-border rounded">
      <div className="flex-1 min-w-0">
        <div className="text-sm text-plex-text truncate font-medium">{item.title}</div>
        <div className="text-xs text-plex-muted mt-0.5">
          {item.media_type}{item.year ? ` · ${item.year}` : ''}
          {item.studio ? ` · ${item.studio}` : ''}
          {missing.length > 0 && (
            <span className="ml-2 text-orange-400">⚠ manque : {missing.join(', ')}</span>
          )}
        </div>
      </div>
      {item.rating != null && item.rating > 0 && (
        <span className="text-xs text-plex-accent">★ {item.rating.toFixed(1)}</span>
      )}
    </div>
  );
}
