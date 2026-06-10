import StatusBadge from './StatusBadge';
import type { MediaNode } from '@/lib/types';

interface Props {
  nodes: MediaNode[];
  onSelect?: (node: MediaNode) => void;
  selectedPath?: string;
}

export default function MediaTree({ nodes, onSelect, selectedPath }: Props) {
  if (nodes.length === 0) {
    return (
      <div className="text-plex-muted text-sm text-center py-8">
        Aucun média à afficher
      </div>
    );
  }

  return (
    <ul className="space-y-1">
      {nodes.map((node) => (
        <li
          key={node.path}
          onClick={() => onSelect?.(node)}
          className={`flex items-center gap-3 px-3 py-2 rounded cursor-pointer transition-colors ${
            selectedPath === node.path
              ? 'bg-plex-accent/20 border border-plex-accent/40'
              : 'hover:bg-plex-surface border border-transparent'
          }`}
        >
          <span className="text-plex-muted text-xs">🎬</span>
          <span className="flex-1 text-sm text-plex-text truncate">{node.name}</span>
          <StatusBadge status={node.status} />
        </li>
      ))}
    </ul>
  );
}
