import { useState, useRef } from "react";
import { Pencil, ImagePlus, RotateCcw, Link, Upload } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { useLanguage } from "@/contexts/LanguageContext";

interface CoverEditDialogProps {
  isOpen: boolean;
  onClose: () => void;
  appId: string;
  gameName: string;
  onCoverChanged: (newUrl: string | null) => void;
}

type TabId = "url" | "upload";

export function CoverEditDialog({ isOpen, onClose, appId, gameName, onCoverChanged }: CoverEditDialogProps) {
  const { t } = useLanguage();
  const l = t.coverEditDialog;

  const [activeTab, setActiveTab] = useState<TabId>("url");
  const [urlInput, setUrlInput] = useState("");
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
  const [isResetting, setIsResetting] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    setSelectedFile(file);
    const reader = new FileReader();
    reader.onload = (ev) => setPreviewUrl(ev.target?.result as string);
    reader.readAsDataURL(file);
  };

  const handleSave = async () => {
    try {
      if (activeTab === "url" && urlInput) {
        await invoke("set_custom_cover", { appId, coverUrl: urlInput });
        onCoverChanged(urlInput);
        handleClose();
      } else if (activeTab === "upload" && selectedFile) {
        const buffer = await selectedFile.arrayBuffer();
        const bytes = Array.from(new Uint8Array(buffer));
        const ext = selectedFile.name.split(".").pop() ?? "jpg";
        const filePath = await invoke<string>("save_cover_image", { appId, bytes, extension: ext });
        await invoke("set_custom_cover", { appId, coverUrl: filePath });
        onCoverChanged(filePath);
        handleClose();
      }
    } catch (e) {
      console.error(e);
    }
  };

  const handleReset = async () => {
    setIsResetting(true);
    try {
      await invoke("remove_custom_cover", { appId });
      const autoCover = await invoke<string | null>("fetch_auto_cover", { gameName });
      onCoverChanged(autoCover);
      handleClose();
    } catch (e) {
      console.error(e);
    } finally {
      setIsResetting(false);
    }
  };

  const handleClose = () => {
    setUrlInput("");
    setSelectedFile(null);
    setPreviewUrl(null);
    setActiveTab("url");
    onClose();
  };

  const canSave = activeTab === "url" ? !!urlInput : !!selectedFile;

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
              <Pencil className="w-5 h-5 text-primary" />
            </div>
            <div>
              <h2 className="text-base font-bold text-foreground">{l.title}</h2>
              <p className="text-xs text-muted-foreground truncate max-w-[240px]">{gameName}</p>
            </div>
          </div>
        </div>

        <div className="px-6 pt-4 pb-2">
          <div className="flex gap-1 p-1 rounded-xl bg-white/[0.04] border border-white/5">
            {(["url", "upload"] as TabId[]).map((tab) => (
              <button
                key={tab}
                onClick={() => setActiveTab(tab)}
                className={`flex-1 flex items-center justify-center gap-2 py-2 rounded-lg text-xs font-semibold transition-all duration-200 ${
                  activeTab === tab
                    ? "bg-primary/90 text-primary-foreground shadow-sm"
                    : "text-muted-foreground hover:text-foreground hover:bg-white/5"
                }`}
              >
                {tab === "url" ? <Link className="w-3.5 h-3.5" /> : <Upload className="w-3.5 h-3.5" />}
                {tab === "url" ? l.urlTab : l.uploadTab}
              </button>
            ))}
          </div>
        </div>

        <div className="px-6 py-4 space-y-4">
          {activeTab === "url" ? (
            <div className="space-y-2">
              <label className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
                {l.urlLabel}
              </label>
              <input
                type="url"
                placeholder={l.urlPlaceholder}
                value={urlInput}
                onChange={(e) => setUrlInput(e.target.value)}
                className="w-full bg-white/[0.04] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-foreground placeholder:text-muted-foreground/50 outline-none focus:border-primary/50 focus:bg-primary/5 transition-all duration-200"
              />
              {urlInput && (
                <div className="mt-3 rounded-xl overflow-hidden border border-white/10 aspect-[3/4] max-h-48 w-32 relative bg-white/[0.03]">
                  <img
                    src={urlInput}
                    alt="Cover preview"
                    className="w-full h-full object-cover"
                    onError={(e) => { e.currentTarget.style.display = "none"; }}
                  />
                </div>
              )}
            </div>
          ) : (
            <div className="space-y-3">
              <label className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
                {l.uploadLabel}
              </label>
              <input
                ref={fileInputRef}
                type="file"
                accept="image/*"
                className="hidden"
                onChange={handleFileSelect}
              />
              <button
                onClick={() => fileInputRef.current?.click()}
                className="w-full flex items-center gap-3 px-4 py-3 rounded-xl border border-dashed border-primary/30 text-primary/80 hover:bg-primary/5 hover:text-primary hover:border-primary/60 transition-all duration-200 text-sm font-medium"
              >
                <ImagePlus className="w-4 h-4" />
                {selectedFile ? l.uploadSelected : l.uploadButton}
              </button>
              {selectedFile && (
                <div className="flex items-center gap-3 p-3 rounded-xl bg-white/[0.03] border border-white/5">
                  {previewUrl && (
                    <div className="w-12 h-16 rounded-lg overflow-hidden border border-white/10 shrink-0">
                      <img src={previewUrl} alt="Preview" className="w-full h-full object-cover" />
                    </div>
                  )}
                  <div className="min-w-0">
                    <span className="text-xs font-semibold text-foreground/90 truncate block">{selectedFile.name}</span>
                    <span className="text-[10px] text-muted-foreground">
                      {(selectedFile.size / 1024).toFixed(0)} KB
                    </span>
                  </div>
                </div>
              )}
            </div>
          )}

          <button
            onClick={handleReset}
            disabled={isResetting}
            className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl border border-dashed border-red-500/20 text-red-400/60 hover:text-red-400 hover:border-red-500/40 hover:bg-red-500/5 transition-all duration-200 text-xs font-medium disabled:opacity-40 disabled:cursor-not-allowed"
          >
            <RotateCcw className={`w-3.5 h-3.5 ${isResetting ? "animate-spin" : ""}`} />
            {isResetting ? l.resetConfirm : l.reset}
          </button>
        </div>

        <div className="px-6 pb-6 flex justify-end gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={handleClose}
            className="rounded-xl border-white/10 hover:bg-white/5 text-muted-foreground hover:text-foreground"
          >
            {l.close}
          </Button>
          <Button
            size="sm"
            onClick={handleSave}
            disabled={!canSave}
            className="rounded-xl gap-2"
          >
            <ImagePlus className="w-4 h-4" />
            {l.save}
          </Button>
        </div>
      </div>
    </div>
  );
}
