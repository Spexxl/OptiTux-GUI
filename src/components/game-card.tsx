import { Sparkles, Download, Trash2, Check } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import locales from "@/locales/en.json";

interface GameCardProps {
  game: {
    name: String;
    platform: string;
    upscalars: string[];
    isInstalled: boolean;
    coverArt?: string;
  };
}

export function GameCard({ game }: GameCardProps) {
  const platformDisplay = game.platform === "Custom" ? "Manual" : game.platform;

  const techBadgeStyles: Record<string, string> = {
    DLSS: "bg-green-500/10 text-green-500",
    FSR: "bg-red-500/10 text-red-500",
    XeSS: "bg-blue-500/10 text-blue-500",
  };

  return (
    <div className="group relative flex flex-col space-y-3 w-full animate-in fade-in zoom-in-95 duration-300">
      <div className="relative aspect-3/4 rounded-xl overflow-hidden bg-muted border border-border/50 shadow-lg">
        <div
          className="absolute inset-0 bg-cover bg-center transition-transform duration-500 group-hover:scale-110"
          style={{
            backgroundImage: game.coverArt ? `url(${game.coverArt})` : "none",
            backgroundColor: "#1a1a1a"
          }}
        />

        <div className="absolute top-3 left-3 flex gap-2">
          <Badge
            variant="secondary"
            className="bg-black/60 backdrop-blur-md text-white border-none hover:bg-black/80 transition-colors uppercase text-[10px] font-bold px-2 py-0.5 font-sans"
          >
            {platformDisplay}
          </Badge>

          {game.isInstalled && (
            <Badge className="bg-green-500/80 backdrop-blur-md text-white border-none text-[10px] font-bold px-2 py-0.5 gap-1 font-sans">
              <Check className="w-3 h-3" />
              {locales.gameCard.installedStatus}
            </Badge>
          )}
        </div>

        <div className="absolute inset-0 bg-black/70 opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex flex-col items-center justify-center space-y-3 p-4">
          {!game.isInstalled ? (
            <>
              <Button size="sm" className="w-full bg-primary/95 hover:bg-primary text-white rounded-lg font-semibold gap-2 shadow-xl translate-y-2 group-hover:translate-y-0 transition-transform duration-300">
                <Sparkles className="w-4 h-4" />
                {locales.gameCard.quickInstall}
              </Button>

              <Button variant="secondary" size="sm" className="w-full bg-white/10 hover:bg-white/20 text-white backdrop-blur-md border-white/10 rounded-lg font-semibold gap-2 translate-y-2 group-hover:translate-y-0 transition-transform duration-300 delay-75">
                <Download className="w-4 h-4" />
                {locales.gameCard.install}
              </Button>
            </>
          ) : (
            <Button variant="destructive" size="sm" className="w-full rounded-lg font-semibold gap-2 translate-y-2 group-hover:translate-y-0 transition-transform duration-300">
              <Trash2 className="w-4 h-4" />
              {locales.gameCard.uninstall}
            </Button>
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
    </div>
  );
}
