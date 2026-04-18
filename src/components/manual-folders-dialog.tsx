import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { FolderPlus, Trash2, FolderOpen } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useLanguage } from "@/contexts/LanguageContext";

interface ManualFoldersDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onFoldersChanged: () => void;
}

export function ManualFoldersDialog({ isOpen, onClose, onFoldersChanged }: ManualFoldersDialogProps) {
  const { t } = useLanguage();
  const translations = t.manualFolders;

  const [folders, setFolders] = useState<string[]>([]);
  const [wasChanged, setWasChanged] = useState(false);

  const loadFolders = async () => {
    const list = await invoke<string[]>("get_custom_folders");
    setFolders(list);
  };

  useEffect(() => {
    if (isOpen) {
      loadFolders();
      setWasChanged(false);
    }
  }, [isOpen]);

  const handleClose = () => {
    if (wasChanged) {
      onFoldersChanged();
    }
    onClose();
  };

  const handleAddFolder = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: translations.pickerTitle,
      });

      if (selected && typeof selected === "string") {
        await invoke("add_custom_folder", { folder: selected });
        await loadFolders();
        setWasChanged(true);
      }
    } catch (e) {
      console.error("Failed to open folder picker", e);
    }
  };

  const handleRemoveFolder = async (path: string) => {
    await invoke("remove_custom_folder", { folder: path });
    await loadFolders();
    setWasChanged(true);
  };

  if (!isOpen) return null;

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center"
      onClick={(e) => { if (e.target === e.currentTarget) handleClose(); }}
    >
      <div className="absolute inset-0 bg-black/70" />

      <div className="relative z-10 w-full max-w-md mx-4 bg-[#1a1a1a] rounded-2xl shadow-2xl overflow-hidden">
        <div className="flex items-center justify-between px-6 pt-6 pb-4 border-b border-white/5">
          <div className="flex items-center gap-3">
            <div className="w-9 h-9 rounded-xl bg-primary/10 flex items-center justify-center">
              <FolderOpen className="w-5 h-5 text-primary" />
            </div>
            <div>
              <h2 className="text-base font-bold text-foreground">{translations.title}</h2>
              <p className="text-xs text-muted-foreground">{translations.subtitle}</p>
            </div>
          </div>
        </div>

        <div className="px-6 py-4 space-y-3">
          <button
            onClick={handleAddFolder}
            className="w-full flex items-center gap-3 px-4 py-3 rounded-xl border border-dashed border-primary/30 text-primary/80 hover:bg-primary/5 hover:text-primary hover:border-primary/60 transition-all duration-200 text-sm font-medium"
          >
            <FolderPlus className="w-4 h-4" />
            {translations.addButton}
          </button>

          <div className="max-h-[280px] overflow-y-auto space-y-2 no-scrollbar">
            {folders.length === 0 ? (
              <div className="text-center py-10 text-muted-foreground/40">
                <FolderOpen className="w-8 h-8 mx-auto mb-2 opacity-30" />
                <p className="text-xs">{translations.emptyState}</p>
              </div>
            ) : (
              folders.map((path) => (
                <div
                  key={path}
                  className="flex items-center justify-between p-3 rounded-xl bg-white/[0.03] border border-white/5 hover:border-white/10 transition-all group"
                >
                  <div className="flex flex-col min-w-0 pr-3">
                    <span className="text-xs font-semibold text-foreground/90 truncate">
                      {path.split("/").pop()}
                    </span>
                    <span className="text-[10px] text-muted-foreground truncate mt-0.5">{path}</span>
                  </div>
                  <button
                    onClick={() => handleRemoveFolder(path)}
                    className="w-7 h-7 flex items-center justify-center rounded-lg text-muted-foreground hover:text-red-400 hover:bg-red-400/10 transition-all shrink-0 opacity-0 group-hover:opacity-100"
                  >
                    <Trash2 className="w-3.5 h-3.5" />
                  </button>
                </div>
              ))
            )}
          </div>
        </div>

        <div className="px-6 pb-6 flex justify-end">
          <Button
            variant="outline"
            size="sm"
            onClick={handleClose}
            className="rounded-xl border-white/10 hover:bg-white/5 text-muted-foreground hover:text-foreground"
          >
            {translations.close}
          </Button>
        </div>
      </div>
    </div>
  );
}
