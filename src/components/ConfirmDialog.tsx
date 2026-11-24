import { Modal } from "./Modal";
import { AlertIcon } from "./Icons";

interface Props {
  title: string;
  text: string;
  confirmLabel: string;
  cancelLabel: string;
  busy?: boolean;
  onConfirm: () => void;
  onClose: () => void;
}

export function ConfirmDialog({
  title,
  text,
  confirmLabel,
  cancelLabel,
  busy = false,
  onConfirm,
  onClose,
}: Props) {
  return (
    <Modal onClose={onClose} label={title}>
      <div className="confirm">
        <div className="confirm-icon">
          <AlertIcon size={22} />
        </div>
        <div>
          <h2>{title}</h2>
          <p>{text}</p>
        </div>
      </div>
      <footer className="modal-foot">
        <button type="button" className="btn btn-ghost" onClick={onClose} disabled={busy}>
          {cancelLabel}
        </button>
        <button
          type="button"
          className="btn btn-danger"
          onClick={onConfirm}
          disabled={busy}
        >
          {busy ? "…" : confirmLabel}
        </button>
      </footer>
    </Modal>
  );
}
