import { RefreshCw, Plus, Search, Cpu, Ghost, Loader2, Shield } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { GameCard, Game } from "@/components/game-card";
import { ManualFoldersDialog } from "@/components/manual-folders-dialog";
import locales from "@/locales/en.json";
import { useState, useEffect, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";

interface GpuInfo {
  name: string;
  is_primary: boolean;
}

type PlatformFilter = "All" | "Steam" | "Heroic" | "Lutris" | "Custom";

const PLATFORM_FILTERS: PlatformFilter[] = ["All", "Steam", "Heroic", "Lutris", "Custom"];

const PLATFORM_LABELS: Record<PlatformFilter, string> = {
  All: "All",
  Steam: "Steam",
  Heroic: "Heroic",
  Lutris: "Lutris",
  Custom: "Manual",
};

export function GamesList() {
  const [gpuName, setGpuName] = useState(locales.toolbar.detectingGpu);
  const [games, setGames] = useState<Game[]>([]);
  const [isScanning, setIsScanning] = useState(true);
  const [searchTerm, setSearchTerm] = useState("");
  const [platformFilter, setPlatformFilter] = useState<PlatformFilter>("All");
  const [onlyInstalled, setOnlyInstalled] = useState(false);
  const [isFoldersDialogOpen, setIsFoldersDialogOpen] = useState(false);

  const fetchGpu = async () => {
    try {
      const info = await invoke<GpuInfo | null>("get_gpu_info");
      if (info) {
        setGpuName(info.name);
      }
    } catch (e) {
      console.error(e);
      setGpuName(locales.toolbar.unknownGpu);
    }
  };

  const fetchGames = async (forceRescan = false) => {
    setIsScanning(true);
    try {
      const result = await invoke<Game[]>("scan_games", {
        forceRescan,
        customFolders: [],
      });
      setGames(result);
    } catch (error) {
      console.error(error);
    } finally {
      setIsScanning(false);
    }
  };

  useEffect(() => {
    fetchGpu();
    fetchGames(false);
  }, []);

  const displayedGames = useMemo(() => {
    return [...games]
      .filter((game) => {
        const matchesSearch = game.name.toLowerCase().includes(searchTerm.toLowerCase());
        const matchesPlatform = platformFilter === "All" || game.platform === platformFilter;
        const matchesInstalled = !onlyInstalled || game.is_optiscaler_installed;
        return matchesSearch && matchesPlatform && matchesInstalled;
      })
      .sort((a, b) => a.name.localeCompare(b.name));
  }, [games, searchTerm, platformFilter, onlyInstalled]);

  return (
    <div className="flex flex-col h-full w-full overflow-hidden animate-in fade-in duration-500">
      <div className="flex shrink-0 flex-col gap-3 px-8 py-4 border-b border-border/40 bg-background/50 backdrop-blur-md z-10">
        <div className="flex items-center justify-between gap-4">
          <div className="flex items-center gap-2">
            <Button
              variant="secondary"
              size="sm"
              className="rounded-lg gap-2 text-xs font-semibold"
              disabled={isScanning}
              onClick={() => fetchGames(true)}
            >
              {isScanning ? (
                <Loader2 className="w-3.5 h-3.5 animate-spin" />
              ) : (
                <RefreshCw className="w-3.5 h-3.5" />
              )}
              {locales.toolbar.scanGames}
            </Button>
            <Button
              variant="secondary"
              size="sm"
              className="rounded-lg gap-2 text-xs font-semibold"
              onClick={() => setIsFoldersDialogOpen(true)}
            >
              <Plus className="w-3.5 h-3.5" />
              {locales.toolbar.addManual}
            </Button>
          </div>

          <div className="flex-1 max-w-md relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
            <Input
              placeholder={locales.toolbar.search}
              className="w-full pl-10 bg-muted/40 border-none rounded-xl text-sm focus-visible:ring-1"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>

          <div className="flex items-center gap-3 bg-secondary/30 px-4 py-2 rounded-xl border border-border/20 shadow-sm">
            <Cpu className="w-4 h-4 text-primary" />
            <span className="text-xs font-bold text-foreground">{gpuName}</span>
          </div>
        </div>

        <div className="flex items-center gap-2">
          {PLATFORM_FILTERS.map((platform) => (
            <button
              key={platform}
              onClick={() => setPlatformFilter(platform)}
              className={`px-3 py-1 rounded-lg text-xs font-semibold transition-all duration-200 ${platformFilter === platform
                ? "bg-primary text-primary-foreground shadow-sm"
                : "bg-muted/40 text-muted-foreground hover:bg-muted hover:text-foreground"
                }`}
            >
              {PLATFORM_LABELS[platform]}
            </button>
          ))}

          <div className="w-px h-4 bg-border/50 mx-1" />

          <button
            onClick={() => setOnlyInstalled((prev) => !prev)}
            className={`flex items-center gap-1.5 px-3 py-1 rounded-lg text-xs font-semibold transition-all duration-200 ${onlyInstalled
              ? "bg-green-500/20 text-green-400 ring-1 ring-green-500/30"
              : "bg-muted/40 text-muted-foreground hover:bg-muted hover:text-foreground"
              }`}
          >
            <Shield className="w-3 h-3" />
            {locales.gamesList.optiscalerFilter}
          </button>

          <span className="ml-auto text-xs text-muted-foreground font-medium">
            {displayedGames.length} / {games.length} {locales.gamesList.gamesCount}
          </span>
        </div>
      </div>

      <div className="flex-1 relative overflow-hidden border-t border-border/10">
        {isScanning && (
          <div className="absolute inset-0 z-50 flex flex-col items-center justify-center bg-background/80 backdrop-blur-sm text-muted-foreground animate-in fade-in duration-300">
            <Loader2 className="w-10 h-10 mb-4 animate-spin text-primary/50" />
            <p className="text-sm font-medium">{locales.gamesList.scanning}</p>
          </div>
        )}

        <div className="h-full overflow-y-auto no-scrollbar px-8 py-8">
          {!isScanning && displayedGames.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-full text-muted-foreground animate-in fade-in zoom-in-95 duration-500">
              <Ghost className="w-16 h-16 mb-4 opacity-50" />
              <h3 className="text-xl font-bold text-foreground mb-1">
                {locales.gamesList.emptyStateTitle}
              </h3>
              <p className="text-sm max-w-sm text-center">
                {locales.gamesList.emptyStateDesc}
              </p>
            </div>
          ) : (
            <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-6">
              {displayedGames.map((game) => (
                <GameCard key={game.app_id} game={game} />
              ))}
            </div>
          )}
        </div>
      </div>

      <ManualFoldersDialog
        isOpen={isFoldersDialogOpen}
        onClose={() => setIsFoldersDialogOpen(false)}
        onFoldersChanged={() => fetchGames(true)}
      />
    </div>
  );
}
