import { SidebarProvider } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";

function App() {
  return (
    <SidebarProvider>
      <div className="flex min-h-screen bg-sidebar dark:bg-[#121212] w-full p-2 gap-2">
        <AppSidebar />
        <main className="flex-1 bg-background dark:bg-[#0A0A0A] rounded-xl border border-border/50 shadow-inner overflow-hidden flex flex-col relative">
          <div className="flex flex-1 items-center justify-center p-6">
            <p className="text-muted-foreground text-sm">
              Content for Games List will appear here.
            </p>
          </div>
        </main>
      </div>
    </SidebarProvider>
  );
}

export default App;
