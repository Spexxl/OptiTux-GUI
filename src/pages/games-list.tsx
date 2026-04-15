import { RefreshCw, Plus, Search, Cpu, Ghost, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { GameCard, Game } from "@/components/game-card";
import locales from "@/locales/en.json";
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface GpuInfo {
  name: string;
  is_primary: boolean;
}

export function GamesList() {
  const [gpuName, setGpuName] = useState("Detecting GPU...");
  const [games, setGames] = useState<Game[]>([]);
  const [isScanning, setIsScanning] = useState(true);

  const fetchGpu = async () => {
    try {
      const info = await invoke<GpuInfo | null>("get_gpu_info");
      if (info) {
        setGpuName(info.name);
      }
    } catch (e) {
      console.error(e);
      setGpuName("Unknown GPU");
    }
  };

  const fetchGames = async (forceRescan = false) => {
    setIsScanning(true);
    try {
      const result = await invoke<Game[]>("scan_games", { 
        forceRescan, 
        customFolders: [] 
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

  return (
    <div className="flex flex-col h-full w-full overflow-hidden animate-in fade-in duration-500">
      <div className="flex shrink-0 items-center justify-between px-8 py-4 border-b border-border/40 bg-background/50 backdrop-blur-md z-10 gap-8">
        <div className="flex items-center gap-2">
          <Button 
            variant="secondary" 
            size="sm" 
            className="rounded-lg gap-2 text-xs font-semibold"
            disabled={isScanning}
            onClick={() => fetchGames(true)}
          >
            {isScanning ? <Loader2 className="w-3.5 h-3.5 animate-spin" /> : <RefreshCw className="w-3.5 h-3.5" />}
            {locales.toolbar.scanGames}
          </Button>
          <Button variant="secondary" size="sm" className="rounded-lg gap-2 text-xs font-semibold disabled:opacity-50">
            <Plus className="w-3.5 h-3.5" />
            {locales.toolbar.addManual}
          </Button>
        </div>

        <div className="flex-1 max-w-md relative">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
          <Input 
            placeholder={locales.toolbar.search} 
            className="w-full pl-10 bg-muted/40 border-none rounded-xl text-sm focus-visible:ring-1"
          />
        </div>

        <div className="flex items-center gap-3 bg-secondary/30 px-4 py-2 rounded-xl border border-border/20 shadow-sm">
          <Cpu className="w-4 h-4 text-primary" />
          <span className="text-xs font-bold text-foreground">
            {gpuName}
          </span>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto no-scrollbar px-8 py-8 relative">
        {!isScanning && games.length === 0 ? (
          <div className="absolute inset-0 flex flex-col items-center justify-center text-muted-foreground animate-in fade-in zoom-in-95 duration-500">
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
            {games.map((game, index) => (
              <GameCard key={index} game={game} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
