import { useState, useEffect, useCallback } from "react";
import { Download, Loader2, CheckCircle2, AlertCircle, RefreshCw, Database, Sparkles } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Button } from "@/components/ui/button";
import { useLanguage } from "@/contexts/LanguageContext";

interface Asset {
  name: string;
  browser_download_url: string;
  size: number;
}

interface Release {
  tag_name: string;
  assets: Asset[];
  prerelease: boolean;
  source: string;
}

interface GpuInfo {
  name: string;
  pci_id: string;
  vendor: string;
  architecture: string;
  is_primary: boolean;
}

interface InstallDialogProps {
  isOpen: boolean;
  onClose: () => void;
  game: {
    name: string;
    install_path: string;
    executable_path: string | null;
    upscalars: string[];
    platform: string;
    app_id: string;
    cover_url: string | null;
    is_optiscaler_installed: boolean;
  };
  onInstallSuccess?: (appId: string) => void;
}

type SourceTab = "stable" | "db";
type InstallState = "idle" | "downloading" | "installing" | "downloading_int8" | "configuring" | "done" | "error";

function getDefaultUpscaler(arch: string): string {
  switch (arch) {
    case "RDNA4":
    case "RDNA1_2_3":
      return "fsr";
    case "RTX":
      return "dlss";
    case "GTX":
    case "IntelArc":
      return "xess";
    default:
      return "fsr";
  }
}

function shouldDefaultInt8(arch: string): boolean {
  return arch === "RDNA1_2_3";
}

export function InstallDialog({ isOpen, onClose, game, onInstallSuccess }: InstallDialogProps) {
  const { t } = useLanguage();
  const translations = t.installDialog;

  const [sourceTab, setSourceTab] = useState<SourceTab>("stable");
  const [allReleases, setAllReleases] = useState<Release[]>([]);
  const [downloadedVersions, setDownloadedVersions] = useState<string[]>([]);
  const [selectedVersion, setSelectedVersion] = useState<string>("");
  const [loadingReleases, setLoadingReleases] = useState(true);
  const [loadError, setLoadError] = useState(false);
  const [upscaler, setUpscaler] = useState("fsr");
  const [installInt8, setInstallInt8] = useState(false);
  const [enableFramegen, setEnableFramegen] = useState(false);
  const [installState, setInstallState] = useState<InstallState>("idle");

  const filteredReleases = allReleases.filter((r) => r.source === sourceTab);

  const isMfgVersion = selectedVersion.toUpperCase().includes("MFG");

  const isVersionDownloaded = (tagName: string) =>
    downloadedVersions.some((v) => v.toLowerCase().includes(tagName.toLowerCase().replace("v", "")));

  const selectedRelease = filteredReleases.find((r) => r.tag_name === selectedVersion);
  const selectedAsset = selectedRelease?.assets.find((a) =>
    a.name.endsWith(".zip") || a.name.endsWith(".7z")
  );

  const fetchReleases = useCallback(async () => {
    setLoadingReleases(true);
    setLoadError(false);
    try {
      const releases = await invoke<Release[]>("get_online_releases");
      setAllReleases(releases);
      const stable = releases.filter((r) => r.source === "stable" && !r.prerelease);
      if (stable.length > 0) {
        setSelectedVersion(stable[0].tag_name);
      }
    } catch {
      setLoadError(true);
    } finally {
      setLoadingReleases(false);
    }
  }, []);

  const fetchDownloaded = useCallback(async () => {
    const versions = await invoke<string[]>("get_downloaded_versions");
    setDownloadedVersions(versions);
  }, []);

  const loadGpuDefaults = useCallback(async () => {
    try {
      const gpu = await invoke<GpuInfo | null>("get_gpu_info");
      if (gpu) {
        setUpscaler(getDefaultUpscaler(gpu.architecture));
        setInstallInt8(shouldDefaultInt8(gpu.architecture));
      }
    } catch { }
  }, []);

  useEffect(() => {
    if (isOpen) {
      fetchReleases();
      fetchDownloaded();
      loadGpuDefaults();
      setInstallState("idle");
      setEnableFramegen(false);
    }
  }, [isOpen, fetchReleases, fetchDownloaded, loadGpuDefaults]);

  useEffect(() => {
    if (filteredReleases.length > 0 && !filteredReleases.find((r) => r.tag_name === selectedVersion)) {
      setSelectedVersion(filteredReleases[0].tag_name);
    }
  }, [sourceTab, allReleases]);

  const handleInstall = useCallback(async () => {
    if (installState !== "idle" || !selectedVersion) return;

    const versionAlreadyDownloaded = isVersionDownloaded(selectedVersion);

    if (!versionAlreadyDownloaded && selectedAsset) {
      setInstallState("downloading");
      try {
        await invoke("download_optiscaler_version", {
          tagName: selectedVersion,
          assetName: selectedAsset.name,
          assetUrl: selectedAsset.browser_download_url,
          assetSize: selectedAsset.size,
        });
        await fetchDownloaded();
      } catch (e) {
        console.error(e);
        setInstallState("error");
        setTimeout(() => setInstallState("idle"), 2500);
        return;
      }
    }

    setInstallState("installing");

    const updatedVersions = await invoke<string[]>("get_downloaded_versions");
    const matchingFolder = updatedVersions.find((v) =>
      v.toLowerCase().includes(selectedVersion.toLowerCase().replace("v", ""))
    );

    if (!matchingFolder) {
      setInstallState("error");
      setTimeout(() => setInstallState("idle"), 2500);
      return;
    }

    const unlisten = await listen<{ phase: string; percent: number }>("custom-install-progress", (event) => {
      const { phase } = event.payload;
      if (phase === "downloading_int8") setInstallState("downloading_int8");
      else if (phase === "configuring") setInstallState("configuring");
      else if (phase === "done") setInstallState("done");
    });

    try {
      await invoke("custom_install_optiscaler", {
        game,
        versionFolder: matchingFolder,
        upscaler,
        installInt8,
        enableFramegen: isMfgVersion ? false : enableFramegen,
        isMfgVersion,
      });
      setInstallState("done");
      setTimeout(() => {
        onInstallSuccess?.(game.app_id);
        handleClose();
      }, 1500);
    } catch (e) {
      console.error(e);
      setInstallState("error");
      setTimeout(() => setInstallState("idle"), 2500);
    } finally {
      unlisten();
    }
  }, [installState, selectedVersion, selectedAsset, game, upscaler, installInt8, enableFramegen, onInstallSuccess]);

  const handleClose = () => {
    if (installState !== "idle" && installState !== "done" && installState !== "error") return;
    setInstallState("idle");
    onClose();
  };

  const getInstallButtonLabel = () => {
    const l = translations;
    switch (installState) {
      case "downloading": return l.downloading;
      case "installing": return l.installing;
      case "downloading_int8": return l.downloadingInt8;
      case "configuring": return l.configuring;
      case "done": return l.done;
      case "error": return l.error;
      default: return l.install;
    }
  };

  const getInstallButtonIcon = () => {
    if (installState === "done") return <CheckCircle2 className="w-4 h-4" />;
    if (installState === "error") return <AlertCircle className="w-4 h-4" />;
    if (installState !== "idle") return <Loader2 className="w-4 h-4 animate-spin" />;
    return <Download className="w-4 h-4" />;
  };

  const isWorking = installState !== "idle" && installState !== "done" && installState !== "error";

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
              <Download className="w-5 h-5 text-primary" />
            </div>
            <div>
              <h2 className="text-base font-bold text-foreground">{translations.title}</h2>
              <p className="text-xs text-muted-foreground truncate max-w-[240px]">{game.name}</p>
            </div>
          </div>
        </div>

        <div className="px-6 py-5 space-y-5">
          <div className="space-y-2">
            <label className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
              {translations.repository}
            </label>
            <div className="flex gap-1 p-1 rounded-xl bg-white/4 border border-white/5">
              <button
                disabled={isWorking}
                onClick={() => setSourceTab("stable")}
                className={`flex-1 flex items-center justify-center gap-1.5 py-2 rounded-lg text-xs font-semibold transition-all duration-200 ${sourceTab === "stable"
                    ? "bg-primary/90 text-primary-foreground shadow-sm"
                    : "text-muted-foreground hover:text-foreground hover:bg-white/5 disabled:opacity-50"
                  }`}
              >
                <Sparkles className="w-3 h-3" />
                {t.optiscalerVersions.stable}
              </button>
              <button
                disabled={isWorking}
                onClick={() => setSourceTab("db")}
                className={`flex-1 flex items-center justify-center gap-1.5 py-2 rounded-lg text-xs font-semibold transition-all duration-200 ${sourceTab === "db"
                    ? "bg-primary/90 text-primary-foreground shadow-sm"
                    : "text-muted-foreground hover:text-foreground hover:bg-white/5 disabled:opacity-50"
                  }`}
              >
                <Database className="w-3 h-3" />
                {t.optiscalerVersions.sourceDb}
              </button>
            </div>
          </div>

          <div className="space-y-2">
            <label className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
              {translations.version}
            </label>
            {loadingReleases ? (
              <div className="flex items-center gap-2 text-muted-foreground text-sm py-2">
                <Loader2 className="w-4 h-4 animate-spin" />
                {translations.loadingVersions}
              </div>
            ) : loadError ? (
              <div className="flex items-center gap-2">
                <span className="text-destructive text-xs">{t.optiscalerVersions.loadError}</span>
                <button onClick={fetchReleases} className="text-xs text-primary hover:underline">
                  <RefreshCw className="w-3 h-3 inline mr-1" />
                  {t.optiscalerVersions.retry}
                </button>
              </div>
            ) : (
              <div className="space-y-1.5">
                <select
                  disabled={isWorking}
                  value={selectedVersion}
                  onChange={(e) => setSelectedVersion(e.target.value)}
                  className="w-full bg-white/4 border border-white/10 rounded-xl px-3 py-2.5 text-sm font-medium appearance-none cursor-pointer focus:outline-none focus:border-primary/50 focus:bg-primary/5 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {filteredReleases.length === 0 ? (
                    <option value="">{translations.noVersions}</option>
                  ) : (
                    filteredReleases.map((r) => (
                      <option key={r.tag_name} value={r.tag_name}>
                        {r.tag_name}
                        {isVersionDownloaded(r.tag_name) ? " ✓" : ""}
                      </option>
                    ))
                  )}
                </select>
                {selectedVersion && (
                  <span className={`text-[10px] font-medium ${isVersionDownloaded(selectedVersion) ? "text-green-400" : "text-amber-400"}`}>
                    {isVersionDownloaded(selectedVersion) ? `✓ ${translations.downloaded}` : `↓ ${translations.notDownloaded}`}
                  </span>
                )}
              </div>
            )}
          </div>

          <div className="space-y-2">
            <label className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
              {translations.upscaler}
            </label>
            <select
              disabled={isWorking}
              value={upscaler}
              onChange={(e) => setUpscaler(e.target.value)}
              className="w-full bg-white/4 border border-white/10 rounded-xl px-3 py-2.5 text-sm font-medium appearance-none cursor-pointer focus:outline-none focus:border-primary/50 focus:bg-primary/5 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <option value="fsr">{translations.upscalerFsr}</option>
              <option value="dlss">{translations.upscalerDlss}</option>
              <option value="xess">{translations.upscalerXess}</option>
            </select>
          </div>

          <div className="space-y-3">
            <label
              className={`flex items-center gap-3 px-3 py-2.5 rounded-xl border transition-all duration-200 cursor-pointer ${installInt8
                  ? "border-primary/40 bg-primary/5"
                  : "border-white/5 bg-white/2 hover:border-white/10"
                } ${isWorking ? "opacity-50 cursor-not-allowed" : ""}`}
            >
              <input
                type="checkbox"
                checked={installInt8}
                disabled={isWorking}
                onChange={(e) => setInstallInt8(e.target.checked)}
                className="sr-only"
              />
              <div className={`w-4 h-4 rounded border-2 flex items-center justify-center transition-all duration-200 shrink-0 ${installInt8
                  ? "bg-primary border-primary"
                  : "border-white/20"
                }`}>
                {installInt8 && (
                  <svg className="w-2.5 h-2.5 text-primary-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="3">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
                  </svg>
                )}
              </div>
              <span className="text-sm font-medium text-foreground/80">{translations.installInt8}</span>
            </label>

            <label
              className={`flex items-center gap-3 px-3 py-2.5 rounded-xl border transition-all duration-200 ${isMfgVersion || enableFramegen
                  ? "border-primary/40 bg-primary/5"
                  : "border-white/5 bg-white/2 hover:border-white/10"
                } ${(isWorking || isMfgVersion) ? "opacity-60 cursor-not-allowed" : "cursor-pointer"}`}
            >
              <input
                type="checkbox"
                checked={isMfgVersion || enableFramegen}
                disabled={isWorking || isMfgVersion}
                onChange={(e) => !isMfgVersion && setEnableFramegen(e.target.checked)}
                className="sr-only"
              />
              <div className={`w-4 h-4 rounded border-2 flex items-center justify-center transition-all duration-200 shrink-0 ${isMfgVersion || enableFramegen
                  ? "bg-primary border-primary"
                  : "border-white/20"
                }`}>
                {(isMfgVersion || enableFramegen) && (
                  <svg className="w-2.5 h-2.5 text-primary-foreground" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="3">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
                  </svg>
                )}
              </div>
              <div className="flex flex-col">
                <span className="text-sm font-medium text-foreground/80">{translations.enableFramegen}</span>
                {isMfgVersion && (
                  <span className="text-[10px] text-primary/70 font-medium">{translations.mfgNotice}</span>
                )}
              </div>
            </label>
          </div>
        </div>

        <div className="px-6 pb-6 flex justify-end gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={handleClose}
            disabled={isWorking}
            className="rounded-xl border-white/10 hover:bg-white/5 text-muted-foreground hover:text-foreground"
          >
            {translations.close}
          </Button>
          <Button
            size="sm"
            onClick={handleInstall}
            disabled={isWorking || installState === "done" || !selectedVersion}
            className={`rounded-xl gap-2 font-semibold ${installState === "done"
                ? "bg-green-500/90 hover:bg-green-500 text-white"
                : installState === "error"
                  ? "bg-red-500/80 hover:bg-red-500 text-white"
                  : ""
              }`}
          >
            {getInstallButtonIcon()}
            {getInstallButtonLabel()}
          </Button>
        </div>
      </div>
    </div>
  );
}
