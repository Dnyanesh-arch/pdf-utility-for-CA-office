import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { JobHistoryItem } from '../types/contracts';

export function JobsPage() {
  const [jobs, setJobs] = useState<JobHistoryItem[]>([]);

  useEffect(() => {
    invoke<JobHistoryItem[]>('list_jobs').then(setJobs).catch(() => setJobs([]));
  }, []);

  return (
    <section>
      <h2>Jobs</h2>
      <div className="card">
        {jobs.map((job) => (
          <div key={job.id} className="list-row">
            <span>{job.timestamp} · {job.operation} · {job.status}</span>
            <span>{job.outputPath}</span>
          </div>
        ))}
        {!jobs.length && <p>No jobs yet.</p>}
      </div>
    </section>
  );
}
