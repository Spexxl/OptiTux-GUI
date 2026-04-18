import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  Download,
  Trash2,
  FolderOpen,
  Loader2,
  CheckCircle2,
  AlertCircle,
  PackageOpen,
  RefreshCw,
  Database,
  Info,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
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

interface DownloadProgress {
  downloaded: number;
  total: number;
  percent: number;
  phase: string;
}

type SourceTab = "stable" | "db";
type DownloadState = "idle" | "downloading" | "done" | "error";

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(1))} ${sizes[i]}`;
}

export function OptiscalerVersions() {
  const { t } = useLanguage();
  const l = t.optiscalerVersions;

  const getPhaseLabel = (phase: string): string => {
    if (phase === "downloading_int8") return l.downloadingInt8;
    if (phase === "done") return l.done;
    return l.downloading;
  };

  const [sourceTab, setSourceTab] = useState<SourceTab>("stable");
  const [allReleases, setAllReleases] = useState<Release[]>([]);
  const [downloadedVersions, setDownloadedVersions] = useState<string[]>([]);
  const [selectedVersion, setSelectedVersion] = useState<string>("");
  const [loadingReleases, setLoadingReleases] = useState(true);
  const [loadError, setLoadError] = useState(false);
  const [downloadState, setDownloadState] = useState<DownloadState>("idle");
  const [downloadProgress, setDownloadProgress] = useState<DownloadProgress | null>(null);
  const [deletingVersion, setDeletingVersion] = useState<string | null>(null);

  const filteredReleases = allReleases.filter((r) => r.source === sourceTab);

  const selectedRelease = filteredReleases.find((r) => r.tag_name === selectedVersion);
  const selectedAsset = selectedRelease?.assets.find((a) =>
    a.name.endsWith(".zip") || a.name.endsWith(".7z")
  );

  const isSelectedDownloaded = downloadedVersions.some((v) =>
    v.toLowerCase().includes(selectedVersion.toLowerCase().replace("v", ""))
  );

  const fetchReleases = useCallback(async () => {
    setLoadingReleases(true);
    setLoadError(false);
    try {
      const releases = await invoke<Release[]>("get_online_releases");
      setAllReleases(releases);
      const stableReleases = releases.filter((r) => !r.prerelease);
      if (stableReleases.length > 0) {
        setSelectedVersion(stableReleases[0].tag_name);
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

  useEffect(() => {
    fetchReleases();
    fetchDownloaded();
  }, [fetchReleases, fetchDownloaded]);

  useEffect(() => {
    const unlisten = listen<DownloadProgress>("download-progress", (event) => {
      setDownloadProgress(event.payload);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  useEffect(() => {
    const stableReleases = filteredReleases;
    if (stableReleases.length > 0 && !stableReleases.find((r) => r.tag_name === selectedVersion)) {
      setSelectedVersion(stableReleases[0].tag_name);
    }
  }, [sourceTab, allReleases]);

  const handleDownload = async () => {
    if (!selectedAsset || !selectedVersion) return;

    setDownloadState("downloading");
    setDownloadProgress({ downloaded: 0, total: selectedAsset.size, percent: 0, phase: "downloading" });

    try {
      await invoke("download_optiscaler_version", {
        tagName: selectedVersion,
        assetName: selectedAsset.name,
        assetUrl: selectedAsset.browser_download_url,
        assetSize: selectedAsset.size,
      });
      setDownloadState("done");
      await fetchDownloaded();
      setTimeout(() => {
        setDownloadState("idle");
        setDownloadProgress(null);
      }, 2500);
    } catch {
      setDownloadState("error");
      setTimeout(() => {
        setDownloadState("idle");
        setDownloadProgress(null);
      }, 3000);
    }
  };

  const handleDelete = async (folderName: string) => {
    setDeletingVersion(folderName);
    try {
      await invoke("remove_downloaded_version", { folderName });
      await fetchDownloaded();
    } finally {
      setDeletingVersion(null);
    }
  };

  const handleOpenFolder = () => {
    invoke("open_versions_folder").catch(() => { });
  };

  const progressPercent = downloadProgress?.percent ?? 0;
  const progressPhase = downloadProgress?.phase ?? "";

  return (
    <div className="flex flex-col h-full w-full overflow-hidden animate-in fade-in duration-500">
      <div className="shrink-0 flex items-center justify-between px-8 py-5 border-b border-border/40 bg-background/50 backdrop-blur-md">
        <div>
          <h1 className="text-lg font-bold tracking-tight">{l.title}</h1>
          <p className="text-xs text-muted-foreground mt-0.5">{l.subtitle}</p>
        </div>
        <Button
          variant="secondary"
          size="sm"
          className="rounded-lg gap-2 text-xs font-semibold"
          onClick={handleOpenFolder}
        >
          <FolderOpen className="w-3.5 h-3.5" />
          {l.openFolder}
        </Button>
      </div>

      <div className="flex-1 overflow-y-auto no-scrollbar px-8 py-8 space-y-8">
        <div className="rounded-2xl border border-border/50 bg-card/50 backdrop-blur-sm shadow-sm overflow-hidden">
          <div className="px-6 pt-6 pb-4 space-y-5">
            <div className="flex items-center gap-2 p-1 bg-muted/40 rounded-xl w-fit border border-border/20">
              <button
                disabled={downloadState === "downloading"}
                onClick={() => setSourceTab("stable")}
                className={`flex items-center gap-1.5 px-4 py-1.5 rounded-lg text-xs font-semibold transition-all duration-200 ${sourceTab === "stable"
                    ? "bg-background shadow-sm text-foreground"
                    : "text-muted-foreground hover:text-foreground disabled:opacity-50"
                  }`}
              >
                <RefreshCw className={`w-3 h-3 ${downloadState === "downloading" ? "animate-spin" : ""}`} />
                {l.sourceStable}
              </button>
              <button
                disabled={downloadState === "downloading"}
                onClick={() => setSourceTab("db")}
                className={`flex items-center gap-1.5 px-4 py-1.5 rounded-lg text-xs font-semibold transition-all duration-200 ${sourceTab === "db"
                    ? "bg-background shadow-sm text-foreground"
                    : "text-muted-foreground hover:text-foreground disabled:opacity-50"
                  }`}
              >
                <Database className="w-3 h-3" />
                {l.sourceDb}
              </button>
            </div>

            {loadingReleases ? (
              <div className="flex items-center gap-2 text-muted-foreground text-sm py-2">
                <Loader2 className="w-4 h-4 animate-spin" />
                {l.loadingVersions}
              </div>
            ) : loadError ? (
              <div className="flex items-center gap-3">
                <div className="flex items-center gap-2 text-destructive text-sm">
                  <AlertCircle className="w-4 h-4" />
                  {l.loadError}
                </div>
                <Button
                  variant="secondary"
                  size="sm"
                  className="rounded-lg gap-1.5 text-xs"
                  onClick={fetchReleases}
                >
                  <RefreshCw className="w-3 h-3" />
                  {l.retry}
                </Button>
              </div>
            ) : (
              <div className="flex items-end gap-3 flex-wrap">
                <div className="flex flex-col gap-1.5 min-w-64">
                  <label className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
                    {l.selectVersion}
                  </label>
                  <select
                    disabled={downloadState === "downloading"}
                    value={selectedVersion}
                    onChange={(e) => setSelectedVersion(e.target.value)}
                    className="w-full bg-background border border-border/50 rounded-xl px-3 py-2 text-sm font-medium appearance-none cursor-pointer focus:outline-none focus:ring-1 focus:ring-ring transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    {filteredReleases.length === 0 ? (
                      <option value="">{l.noVersionsAvailable}</option>
                    ) : (
                      filteredReleases.map((r) => (
                        <option key={r.tag_name} value={r.tag_name}>
                          {r.tag_name}
                          {downloadedVersions.some((v) =>
                            v.toLowerCase().includes(r.tag_name.toLowerCase().replace("v", ""))
                          )
                            ? " ✓"
                            : ""}
                        </option>
                      ))
                    )}
                  </select>
                </div>

                <div className="flex flex-col gap-1.5">
                  {selectedAsset && (
                    <span className="text-[10px] text-muted-foreground font-medium">
                      {selectedAsset.name} · {formatBytes(selectedAsset.size)}
                    </span>
                  )}
                  {isSelectedDownloaded ? (
                    <Badge className="bg-green-500/15 text-green-400 border-green-500/20 font-semibold gap-1.5 px-3 py-1.5 rounded-lg text-xs w-fit border">
                      <CheckCircle2 className="w-3 h-3" />
                      {l.downloaded}
                    </Badge>
                  ) : (
                    <Button
                      size="sm"
                      className="rounded-xl gap-2 font-semibold text-xs px-5"
                      disabled={!selectedAsset || downloadState === "downloading"}
                      onClick={handleDownload}
                    >
                      {downloadState === "downloading" ? (
                        <Loader2 className="w-3.5 h-3.5 animate-spin" />
                      ) : downloadState === "done" ? (
                        <CheckCircle2 className="w-3.5 h-3.5" />
                      ) : downloadState === "error" ? (
                        <AlertCircle className="w-3.5 h-3.5" />
                      ) : (
                        <Download className="w-3.5 h-3.5" />
                      )}
                      {downloadState === "idle" ? l.download :
                        downloadState === "done" ? l.done :
                          downloadState === "error" ? l.error :
                            getPhaseLabel(progressPhase)}
                    </Button>
                  )}
                </div>
              </div>
            )}

            {downloadState === "downloading" && downloadProgress && (
              <div className="space-y-2 pt-1 animate-in fade-in duration-300">
                <div className="flex justify-between items-center">
                  <span className="text-xs text-muted-foreground font-medium">
                    {getPhaseLabel(progressPhase)}
                  </span>
                  <span className="text-xs font-bold tabular-nums">
                    {Math.round(progressPercent)}%
                  </span>
                </div>
                <Progress value={progressPercent} className="h-1.5" />
                {progressPhase === "downloading" && downloadProgress.total > 0 && (
                  <span className="text-[10px] text-muted-foreground">
                    {formatBytes(downloadProgress.downloaded)} / {formatBytes(downloadProgress.total)}
                  </span>
                )}
              </div>
            )}

            {!loadingReleases && !loadError && !isSelectedDownloaded && selectedAsset && downloadState === "idle" && (
              <div className="flex items-center gap-2 text-xs text-muted-foreground bg-muted/30 rounded-lg px-3 py-2 w-fit border border-border/20">
                <Info className="w-3 h-3 shrink-0" />
                {l.int8Included}
              </div>
            )}
          </div>
        </div>

        <div className="space-y-3">
          <h2 className="text-sm font-bold text-muted-foreground uppercase tracking-widest px-1">
            {l.localVersions}
          </h2>

          {downloadedVersions.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-16 text-muted-foreground gap-3 animate-in fade-in duration-300">
              <PackageOpen className="w-10 h-10 opacity-30" />
              <span className="text-sm font-medium">{l.noLocalVersions}</span>
            </div>
          ) : (
            <div className="space-y-2">
              {downloadedVersions.map((version) => (
                <div
                  key={version}
                  className="flex items-center justify-between px-5 py-3.5 rounded-xl border border-border/40 bg-card/40 backdrop-blur-sm hover:bg-card/70 transition-colors group"
                >
                  <div className="flex items-center gap-3">
                    <div className="w-2 h-2 rounded-full bg-green-500/70 group-hover:bg-green-500 transition-colors" />
                    <span className="text-sm font-semibold font-mono">{version}</span>
                  </div>
                  <Button
                    variant="ghost"
                    size="sm"
                    className="gap-1.5 text-xs text-muted-foreground hover:text-destructive hover:bg-destructive/10 rounded-lg transition-colors"
                    disabled={deletingVersion === version}
                    onClick={() => handleDelete(version)}
                  >
                    {deletingVersion === version ? (
                      <Loader2 className="w-3.5 h-3.5 animate-spin" />
                    ) : (
                      <Trash2 className="w-3.5 h-3.5" />
                    )}
                    {l.delete}
                  </Button>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
