import React, { useCallback, useEffect, useState } from 'react';
import { Routes, Route, Navigate, useNavigate, useParams } from 'react-router';
import Dashboard from './components/Dashboard.jsx';
import Editor from './Editor.jsx';
import { API_BASE, renderUrl } from './lib/api.js';

function mapServer(p) {
  return {
    id: p.project_id,
    name: 'Project ' + String(p.project_id).slice(0, 8),
    createdAt: p.created_at ? new Date(p.created_at).getTime() : Date.now(),
    updatedAt: p.updated_at ? new Date(p.updated_at).getTime() : Date.now(),
    jobCount: p.job_count,
    latestStatus: p.latest_status,
    latestJobId: p.latest_job_id,
    latestRenderUrl: renderUrl(p.latest_render_url),
    server: true,
  };
}

// Shared project store — lifted so both pages share the same data
function useProjects() {
  const [projects, setProjects] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  const fetchProjects = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_BASE}/api/projects`);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = await res.json();
      setProjects(Array.isArray(data) ? data.map(mapServer).sort((a, b) => (b.updatedAt || 0) - (a.updatedAt || 0)) : []);
    } catch (e) {
      setError(e.message || 'Failed to load projects');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { fetchProjects(); }, [fetchProjects]);

  return { projects, setProjects, loading, error, fetchProjects };
}

function DashboardPage({ projects, loading, error, fetchProjects, setProjects }) {
  const navigate = useNavigate();

  const deleteProject = async (id) => {
    if (!confirm(`Delete this project and all its jobs? This cannot be undone.`)) return;
    try {
      const res = await fetch(`${API_BASE}/api/projects/${id}`, { method: 'DELETE' });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
    } catch (e) {
      alert(`Delete failed: ${e.message}`);
      return;
    }
    setProjects((list) => list.filter((p) => p.id !== id));
  };

  const renameProject = (id, name) => {
    setProjects((list) => list.map((p) => p.id === id ? { ...p, name } : p));
  };

  return (
    <Dashboard
      projects={projects}
      loading={loading}
      error={error}
      onRefresh={fetchProjects}
      onOpen={(id) => navigate(`/${id}`)}
      onCreate={() => navigate('/new')}
      onDelete={deleteProject}
      onRename={renameProject}
    />
  );
}

function EditorPage({ projects, loading, fetchProjects }) {
  const { projectId } = useParams();
  const navigate = useNavigate();
  const isNew = projectId === 'new';

  const project = isNew
    ? { id: 'new', name: 'New project', server: false }
    : projects.find((p) => p.id === projectId);

  // Stay blank while projects are loading (prevents flash-redirect)
  if (!project && loading) return null;
  // Project genuinely doesn't exist
  if (!project && !loading) return <Navigate to="/projects" replace />;

  const handleCreated = async (newProjectId) => {
    await fetchProjects();
    navigate(`/${newProjectId}`, { replace: true });
  };

  return (
    <Editor
      key={isNew ? 'new' : projectId}
      project={project}
      onBack={() => navigate('/')}
      onCreated={handleCreated}
    />
  );
}

export default function App() {
  const store = useProjects();
  return (
    <Routes>
      <Route path="/" element={<DashboardPage {...store} />} />
      <Route path="/:projectId" element={<EditorPage {...store} />} />
      <Route path="*" element={<Navigate to="/" replace />} />
    </Routes>
  );
}



