interface Props {
  open: boolean;
  title: string;
  message: string;
}

export function ProgressModal({ open, title, message }: Props) {
  if (!open) return null;
  return (
    <div className="modal-overlay">
      <div className="modal card">
        <h3>{title}</h3>
        <p>{message}</p>
        <div className="progress" />
      </div>
    </div>
  );
}
