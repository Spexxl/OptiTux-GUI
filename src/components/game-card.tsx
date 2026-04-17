import { useState, useCallback } from "react";
import { Sparkles, Download, Trash2, Check, Target, Loader2, CheckCircle2, FolderOpen, Pencil, AlertCircle } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { CoverEditDialog } from "@/components/cover-edit-dialog";
import locales from "@/locales/en.json";

export interface Game {
  name: string;
  install_path: string;
  executable_path: string | null;
  upscalars: string[];
  platform: string;
  app_id: string;
  cover_url: string | null;
  is_optiscaler_installed: boolean;
}

interface GameCardProps {
  game: Game;
  onUninstallSuccess?: (appId: string) => void;
  onInstallSuccess?: (appId: string) => void;
}

type UninstallState = "idle" | "loading" | "done" | "error";
type QuickInstallPhase = "idle" | "fetching" | "downloading" | "downloading_int8" | "installing" | "done" | "error";

export function GameCard({ game, onUninstallSuccess, onInstallSuccess }: GameCardProps) {
  const platformDisplay = game.platform === "Custom" ? "Manual" : game.platform;
  const [isInstalled, setIsInstalled] = useState(game.is_optiscaler_installed);
  const [uninstallState, setUninstallState] = useState<UninstallState>("idle");
  const [coverUrl, setCoverUrl] = useState(game.cover_url);
  const [isEditDialogOpen, setIsEditDialogOpen] = useState(false);
  const [quickInstallPhase, setQuickInstallPhase] = useState<QuickInstallPhase>("idle");

  const displayUrl = coverUrl?.startsWith("/")
    ? convertFileSrc(coverUrl)
    : coverUrl;

  const techBadgeStyles: Record<string, string> = {
    DLSS: "bg-green-500/10 text-green-500",
    FSR: "bg-red-500/10 text-red-500",
    XeSS: "bg-blue-500/10 text-blue-500",
  };

  const handleQuickInstall = useCallback(async () => {
    if (quickInstallPhase !== "idle") return;

    setQuickInstallPhase("fetching");

    const unlisten = await listen<{ phase: string; percent: number }>("quick-install-progress", (event) => {
      const { phase } = event.payload;
      if (phase === "fetching") setQuickInstallPhase("fetching");
      else if (phase === "downloading") setQuickInstallPhase("downloading");
      else if (phase === "downloading_int8") setQuickInstallPhase("downloading_int8");
      else if (phase === "installing") setQuickInstallPhase("installing");
      else if (phase === "done") setQuickInstallPhase("done");
    });

    try {
      await invoke("quick_install_optiscaler", { game });
      setQuickInstallPhase("done");
      setTimeout(() => {
        setIsInstalled(true);
        setQuickInstallPhase("idle");
        onInstallSuccess?.(game.app_id);
      }, 2000);
    } catch (e) {
      console.error(e);
      setQuickInstallPhase("error");
      setTimeout(() => setQuickInstallPhase("idle"), 2500);
    } finally {
      unlisten();
    }
  }, [game, quickInstallPhase, onInstallSuccess]);

  const getQuickInstallLabel = () => {
    const l = locales.gameCard;
    switch (quickInstallPhase) {
      case "fetching": return l.quickInstallDownloading;
      case "downloading": return l.quickInstallDownloading;
      case "downloading_int8": return l.quickInstallDownloadingInt8;
      case "installing": return l.quickInstalling;
      case "done": return l.quickInstallDone;
      case "error": return l.quickInstallError;
      default: return l.quickInstall;
    }
  };

  const getQuickInstallIcon = () => {
    if (quickInstallPhase === "done") return <CheckCircle2 className="w-4 h-4" />;
    if (quickInstallPhase === "error") return <AlertCircle className="w-4 h-4" />;
    if (quickInstallPhase !== "idle") return <Loader2 className="w-4 h-4 animate-spin" />;
    return <Sparkles className="w-4 h-4" />;
  };

  const quickInstallBtnClass = () => {
    const base = "w-full rounded-lg font-semibold gap-2 translate-y-2 group-hover:translate-y-0 transition-all duration-300";
    if (quickInstallPhase === "done") return `${base} bg-green-500/90 hover:bg-green-500 text-white`;
    if (quickInstallPhase === "error") return `${base} bg-red-500/80 hover:bg-red-500 text-white`;
    if (quickInstallPhase !== "idle") return `${base} bg-gray-500/80 text-white cursor-not-allowed`;
    return `${base} bg-gray-400 hover:bg-gray-500 text-white shadow-xl hover:scale-[1.02] active:scale-[0.98]`;
  };

  const handleUninstall = async () => {
    if (uninstallState === "loading") return;

    setUninstallState("loading");

    try {
      await invoke("uninstall_optiscaler", { game });
      setUninstallState("done");

      setTimeout(() => {
        setIsInstalled(false);
        setUninstallState("idle");
        onUninstallSuccess?.(game.app_id);
      }, 1800);
    } catch (e) {
      console.error(e);
      setUninstallState("error");
      setTimeout(() => setUninstallState("idle"), 2500);
    }
  };

  const renderUninstallButton = () => {
    if (uninstallState === "loading") {
      return (
        <Button
          variant="destructive"
          size="sm"
          disabled
          className="w-full rounded-lg font-semibold gap-2 translate-y-2 group-hover:translate-y-0 transition-all duration-300 opacity-90"
        >
          <Loader2 className="w-4 h-4 animate-spin" />
          {locales.gameCard.uninstalling}
        </Button>
      );
    }

    if (uninstallState === "done") {
      return (
        <Button
          variant="secondary"
          size="sm"
          disabled
          className="w-full rounded-lg font-semibold gap-2 translate-y-2 group-hover:translate-y-0 transition-all duration-300 bg-green-500/20 text-green-400 border-green-500/30"
        >
          <CheckCircle2 className="w-4 h-4" />
          {locales.gameCard.uninstallDone}
        </Button>
      );
    }

    if (uninstallState === "error") {
      return (
        <Button
          variant="destructive"
          size="sm"
          disabled
          className="w-full rounded-lg font-semibold gap-2 translate-y-2 group-hover:translate-y-0 transition-all duration-300 opacity-70"
        >
          <Trash2 className="w-4 h-4" />
          {locales.gameCard.uninstallError}
        </Button>
      );
    }

    return (
      <Button
        variant="destructive"
        size="sm"
        className="w-full rounded-lg font-semibold gap-2 translate-y-2 group-hover:translate-y-0 transition-all duration-300 hover:scale-[1.02] active:scale-[0.98]"
        onClick={handleUninstall}
      >
        <Trash2 className="w-4 h-4" />
        {locales.gameCard.uninstall}
      </Button>
    );
  };

  return (
    <div className="group relative flex flex-col space-y-3 w-full animate-in fade-in zoom-in-95 duration-300">
      <div className="relative aspect-3/4 rounded-xl overflow-hidden bg-muted border border-border/50 shadow-lg flex flex-col">
        {displayUrl ? (
          <div
            className="absolute inset-0 bg-cover bg-center transition-transform duration-500 group-hover:scale-110"
            style={{
              backgroundImage: `url(${displayUrl})`,
              backgroundColor: "#1a1a1a"
            }}
          />
        ) : (
          <div className="absolute inset-0 bg-[#1a1a1a] flex flex-col items-center justify-center p-4 text-center transition-transform duration-500 group-hover:scale-110">
            <Target className="w-12 h-12 text-muted-foreground/30 mb-3" />
            <span className="font-bold text-lg text-muted-foreground/50 uppercase tracking-widest break-words w-full px-2" style={{ wordBreak: 'break-word', overflowWrap: 'break-word' }}>
              {game.name}
            </span>
          </div>
        )}

        <div className="absolute top-3 left-3 flex gap-2">
          <Badge
            variant="secondary"
            className="bg-black/60 backdrop-blur-md text-white border-none hover:bg-black/80 transition-colors uppercase text-[10px] font-bold px-2 py-0.5 font-sans"
          >
            {platformDisplay}
          </Badge>

          {isInstalled && (
            <Badge className="bg-green-500/80 backdrop-blur-md text-white border-none text-[10px] font-bold px-2 py-0.5 gap-1 font-sans">
              <Check className="w-3 h-3" />
              {locales.gameCard.installedStatus}
            </Badge>
          )}
        </div>

        <div className="absolute top-3 right-3 flex gap-1.5">
          <Button
            size="icon"
            variant="secondary"
            className="h-7 w-7 bg-black/60 backdrop-blur-md text-white border-none hover:bg-black/80 transition-all duration-300 rounded-lg shadow-xl hover:scale-110 active:scale-95 z-10"
            onClick={(e) => {
              e.stopPropagation();
              invoke("open_game_folder", { game });
            }}
          >
            <FolderOpen className="w-4 h-4" />
          </Button>

          <Button
            size="icon"
            variant="secondary"
            className="h-7 w-7 bg-black/60 backdrop-blur-md text-white border-none hover:bg-black/80 transition-all duration-300 rounded-lg shadow-xl hover:scale-110 active:scale-95 z-10"
            onClick={(e) => {
              e.stopPropagation();
              setIsEditDialogOpen(true);
            }}
          >
            <Pencil className="w-3.5 h-3.5" />
          </Button>
        </div>

        <div className="absolute inset-0 bg-black/70 opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex flex-col items-center justify-center space-y-3 p-4">
          {!isInstalled ? (
            <>
              <Button
                size="sm"
                className={quickInstallBtnClass()}
                disabled={quickInstallPhase !== "idle"}
                onClick={handleQuickInstall}
              >
                {getQuickInstallIcon()}
                {getQuickInstallLabel()}
              </Button>

              <Button variant="secondary" size="sm" className="w-full bg-white/10 hover:bg-white/20 text-white backdrop-blur-md border-white/10 rounded-lg font-semibold gap-2 translate-y-2 group-hover:translate-y-0 transition-transform duration-300 delay-75">
                <Download className="w-4 h-4" />
                {locales.gameCard.install}
              </Button>
            </>
          ) : (
            renderUninstallButton()
          )}
        </div>
      </div>

      <div className="space-y-2 px-1">
        <h3 className="font-bold text-sm text-foreground truncate">
          {game.name}
        </h3>

        <div className="flex flex-wrap gap-1.5">
          {game.upscalars.map((tech) => (
            <Badge key={tech} className={`text-[9px] font-extrabold px-1.5 py-0 rounded-sm border-none shadow-sm ${techBadgeStyles[tech] || "bg-zinc-500/10 text-zinc-500"}`}>
              {tech}
            </Badge>
          ))}
        </div>
      </div>

      <CoverEditDialog
        isOpen={isEditDialogOpen}
        onClose={() => setIsEditDialogOpen(false)}
        appId={game.app_id}
        gameName={game.name}
        onCoverChanged={setCoverUrl}
      />
    </div>
  );
}
