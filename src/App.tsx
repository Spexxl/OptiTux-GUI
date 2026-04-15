import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";
import { useState } from "react";

function App() {
  const [activeTab, setActiveTab] = useState("gamesList");

  return (
    <SidebarProvider>
      <div className="flex min-h-screen bg-sidebar dark:bg-[#121212] w-full p-2 gap-2">
        <AppSidebar activeTab={activeTab} onTabChange={setActiveTab} />
        <main className="flex-1 bg-background dark:bg-[#0A0A0A] rounded-xl border border-border/50 shadow-inner overflow-hidden flex flex-col relative transition-all duration-300">
          <div className="flex flex-1 flex-col p-8 overflow-y-auto">
            {activeTab === "gamesList" && (
              <div className="space-y-4">
                <h2 className="text-2xl font-bold tracking-tight text-foreground">Games List</h2>
                <p className="text-muted-foreground">Manage and optimize your installed games.</p>
              </div>
            )}
            
            {activeTab === "optiscalerVersions" && (
              <div className="space-y-4">
                <h2 className="text-2xl font-bold tracking-tight text-foreground">Optiscaler Versions</h2>
                <p className="text-muted-foreground">Download and manage different versions of Optiscaler.</p>
              </div>
            )}

            {activeTab === "community" && (
              <div className="space-y-4">
                <h2 className="text-2xl font-bold tracking-tight text-foreground">Community</h2>
                <p className="text-muted-foreground">Connect with other players and share profiles.</p>
              </div>
            )}

            {activeTab === "settings" && (
              <div className="space-y-4">
                <h2 className="text-2xl font-bold tracking-tight text-foreground">Settings</h2>
                <p className="text-muted-foreground">Configure global application behavior.</p>
              </div>
            )}

            {activeTab === "about" && (
              <div className="space-y-4">
                <h2 className="text-2xl font-bold tracking-tight text-foreground">About</h2>
                <p className="text-muted-foreground">OptiTux-GUI v0.1.0 - Optiscaler interface manager for Linux.</p>
              </div>
            )}
          </div>
        </main>
      </div>
    </SidebarProvider>
  );
}

export default App;
