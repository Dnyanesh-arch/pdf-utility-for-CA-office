import { NavLink, Route, Routes } from 'react-router-dom';
import { Dashboard } from './pages/Dashboard';
import { MergePage } from './pages/MergePage';
import { CompressPage } from './pages/CompressPage';
import { SplitPage } from './pages/SplitPage';
import { JobsPage } from './pages/JobsPage';
import { SettingsPage } from './pages/SettingsPage';

const links = [
  { to: '/', label: 'Dashboard' },
  { to: '/merge', label: 'Merge + Index' },
  { to: '/compress', label: 'Compress' },
  { to: '/split', label: 'Split' },
  { to: '/jobs', label: 'Jobs' },
  { to: '/settings', label: 'Settings' }
];

export default function App() {
  return (
    <div className="app-shell">
      <aside className="sidebar">
        <h1>CA PDF Utility</h1>
        <nav>
          {links.map((link) => (
            <NavLink key={link.to} to={link.to} end={link.to === '/'}>
              {link.label}
            </NavLink>
          ))}
        </nav>
      </aside>
      <main className="content">
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/merge" element={<MergePage />} />
          <Route path="/compress" element={<CompressPage />} />
          <Route path="/split" element={<SplitPage />} />
          <Route path="/jobs" element={<JobsPage />} />
          <Route path="/settings" element={<SettingsPage />} />
        </Routes>
      </main>
    </div>
  );
}
