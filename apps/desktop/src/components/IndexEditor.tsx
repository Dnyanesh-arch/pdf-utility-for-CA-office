import { IndexOptions } from '../types/contracts';

interface Props {
  value: IndexOptions;
  onChange: (next: IndexOptions) => void;
}

export function IndexEditor({ value, onChange }: Props) {
  const update = <K extends keyof IndexOptions>(key: K, next: IndexOptions[K]) => {
    onChange({ ...value, [key]: next });
  };

  return (
    <section className="card">
      <h3>Auto Index Page</h3>
      <label>
        <input
          type="checkbox"
          checked={value.enabled}
          onChange={(e) => update('enabled', e.target.checked)}
        />
        Insert index as first page
      </label>
      <div className="grid-2">
        <input
          placeholder="Bundle title"
          value={value.bundleTitle}
          onChange={(e) => update('bundleTitle', e.target.value)}
        />
        <input
          placeholder="Client name"
          value={value.clientName ?? ''}
          onChange={(e) => update('clientName', e.target.value)}
        />
        <input
          placeholder="AY"
          value={value.assessmentYear ?? ''}
          onChange={(e) => update('assessmentYear', e.target.value)}
        />
        <input
          placeholder="FY"
          value={value.financialYear ?? ''}
          onChange={(e) => update('financialYear', e.target.value)}
        />
      </div>
    </section>
  );
}
