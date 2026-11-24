import { CheckIcon, AlertIcon, CloseIcon } from "./Icons";

export interface ToastData {
  kind: "success" | "error";
  text: string;
}

interface Props {
  toast: ToastData;
  onClose: () => void;
  closeLabel: string;
}

export function Toast({ toast, onClose, closeLabel }: Props) {
  return (
    <div className={`toast toast-${toast.kind}`} role="status">
      <span className="toast-icon">
        {toast.kind === "success" ? (
          <CheckIcon size={16} strokeWidth={2.6} />
        ) : (
          <AlertIcon size={16} />
        )}
      </span>
      <span>{toast.text}</span>
      <button type="button" onClick={onClose} aria-label={closeLabel}>
        <CloseIcon size={14} />
      </button>
    </div>
  );
}
