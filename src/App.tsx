import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";
import { GameCard } from "@/components/game-card";
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

function App() {
  const [activeTab, setActiveTab] = useState("gamesList");

  return (
    <SidebarProvider>
      <div className="flex min-h-screen bg-sidebar dark:bg-[#121212] w-full p-2 gap-2">
        <AppSidebar activeTab={activeTab} onTabChange={setActiveTab} />
        <main className="flex-1 bg-background dark:bg-[#0A0A0A] rounded-xl border border-border/50 shadow-inner overflow-hidden flex flex-col relative transition-all duration-300">
          <div className="flex flex-1 flex-col p-8 overflow-y-auto no-scrollbar">
            {activeTab === "gamesList" && (
              <div className="space-y-8 animate-in fade-in duration-500">
                <div className="flex flex-col space-y-1">
                  <h2 className="text-2xl font-bold tracking-tight text-foreground">Games List</h2>
                  <p className="text-muted-foreground text-sm">Manage and optimize your installed games.</p>
                </div>
                
                <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-6">
                  {MOCK_GAMES.map((game, index) => (
                    <GameCard key={index} game={game} />
                  ))}
                </div>
              </div>
            )}
            
            {activeTab === "optiscalerVersions" && (
              <div className="space-y-4 animate-in fade-in duration-500">
                <h2 className="text-2xl font-bold tracking-tight">Optiscaler Versions</h2>
                <p className="text-muted-foreground text-sm">Download and manage your Optiscaler assets.</p>
              </div>
            )}

            {activeTab === "community" && (
              <div className="space-y-4 animate-in fade-in duration-500">
                <h2 className="text-2xl font-bold tracking-tight">Community</h2>
                <p className="text-muted-foreground text-sm">Browse shared configurations and profiles.</p>
              </div>
            )}

            {activeTab === "settings" && (
              <div className="space-y-4 animate-in fade-in duration-500">
                <h2 className="text-2xl font-bold tracking-tight">Settings</h2>
                <p className="text-muted-foreground text-sm">Configure application behavior.</p>
              </div>
            )}

            {activeTab === "about" && (
              <div className="space-y-4 animate-in fade-in duration-500">
                <h2 className="text-2xl font-bold tracking-tight">About</h2>
                <p className="text-muted-foreground text-sm">OptiTux-GUI v0.1.0</p>
              </div>
            )}
          </div>
        </main>
      </div>
    </SidebarProvider>
  );
}

export default App;
