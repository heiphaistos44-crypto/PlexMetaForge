import type { MetadataStatus } from '@/lib/types';

interface Props {
  status: MetadataStatus;
}

const STYLES: Record<MetadataStatus, string> = {
  complete: 'bg-green-500/20 text-green-400 border-green-500/40',
  incomplete: 'bg-orange-500/20 text-orange-400 border-orange-500/40',
  missing: 'bg-red-500/20 text-red-400 border-red-500/40',
};

const LABELS: Record<MetadataStatus, string> = {
  complete: 'Complet',
  incomplete: 'Incomplet',
  missing: 'Absent',
};

export default function StatusBadge({ status }: Props) {
  return (
    <span
      className={`inline-block px-2 py-0.5 rounded border text-xs font-medium ${STYLES[status]}`}
    >
      {LABELS[status]}
    </span>
  );
}
