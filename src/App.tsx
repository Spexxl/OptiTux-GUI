import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";
import { GamesList } from "@/pages/games-list";
import { OptiscalerVersions } from "@/pages/optiscaler-versions";
import { Community } from "@/pages/community";
import { Settings } from "@/pages/settings";
import { About } from "@/pages/about";
import { useState } from "react";

function App() {
  const [activeTab, setActiveTab] = useState("gamesList");

  return (
    <SidebarProvider>
      <div className="flex min-h-screen bg-sidebar dark:bg-[#121212] w-full p-2 gap-2">
        <AppSidebar activeTab={activeTab} onTabChange={setActiveTab} />
        <main className="flex-1 bg-background dark:bg-[#0A0A0A] rounded-xl border border-border/50 shadow-inner overflow-hidden flex flex-col relative transition-all duration-300">
          <div className="flex flex-1 flex-col overflow-hidden">
            {activeTab === "gamesList" && <GamesList />}
            {activeTab === "optiscalerVersions" && <OptiscalerVersions />}
            {activeTab === "community" && <Community />}
            {activeTab === "settings" && <Settings />}
            {activeTab === "about" && <About />}
          </div>
        </main>
      </div>
    </SidebarProvider>
  );
}

export default App;
