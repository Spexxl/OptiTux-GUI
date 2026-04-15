import { RefreshCw, Plus, Search, Cpu } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { GameCard } from "@/components/game-card";
import locales from "@/locales/en.json";
import { useState } from "react";

const MOCK_GAMES = [
  {
    name: "God of War",
    platform: "Steam",
    upscalars: ["DLSS"],
    isInstalled: false,
    coverArt: "https://images.igdb.com/igdb/image/upload/t_cover_big/co1vcp.webp"
  },
  {
    name: "God of War Ragnarök",
    platform: "Heroic",
    upscalars: ["DLSS", "FSR", "XeSS"],
    isInstalled: true,
    coverArt: "https://images.igdb.com/igdb/image/upload/t_cover_big/co58tc.webp"
  },
  {
    name: "Lossless Scaling",
    platform: "Lutris",
    upscalars: [],
    isInstalled: false,
    coverArt: "https://images.igdb.com/igdb/image/upload/t_cover_big/co7v2d.webp"
  },
  {
    name: "Marvel's Spider-Man 2",
    platform: "Custom",
    upscalars: ["DLSS", "FSR"],
    isInstalled: false,
    coverArt: "https://images.igdb.com/igdb/image/upload/t_cover_big/co6cl3.webp"
  }
];

export function GamesList() {
  const [gpuName] = useState("NVIDIA GeForce RTX 4070");

  return (
    <div className="flex flex-col h-full animate-in fade-in duration-500">
      <div className="flex items-center justify-between px-8 py-4 border-b border-border/40 bg-background/50 backdrop-blur-md sticky top-0 z-10 gap-8">
        <div className="flex items-center gap-2">
          <Button variant="secondary" size="sm" className="rounded-lg gap-2 text-xs font-semibold">
            <RefreshCw className="w-3.5 h-3.5" />
            {locales.toolbar.scanGames}
          </Button>
          <Button variant="secondary" size="sm" className="rounded-lg gap-2 text-xs font-semibold">
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

      <div className="flex-1 p-8 overflow-y-auto no-scrollbar">
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-6">
          {MOCK_GAMES.map((game, index) => (
            <GameCard key={index} game={game} />
          ))}
        </div>
      </div>
    </div>
  );
}
