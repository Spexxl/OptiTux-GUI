import { Gamepad2, Box, LayoutGrid, Settings, Info } from "lucide-react";
import locales from "@/locales/en.json";
import links from "@/data/links.json";
import { openUrl } from "@tauri-apps/plugin-opener";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";

interface AppSidebarProps {
  activeTab: string;
  onTabChange: (tab: string) => void;
}

export function AppSidebar({ activeTab, onTabChange }: AppSidebarProps) {
  return (
    <Sidebar className="border-r-0">
      <SidebarHeader className="p-6">
        <div className="flex items-center gap-3">
          <img src="/OptiTuxLogo.png" alt="Logo" className="w-10 h-10" />
          <span className="font-semibold text-lg tracking-tight">
            {locales.app.title}
          </span>
        </div>
      </SidebarHeader>

      <SidebarContent>
        <SidebarMenu className="px-4 space-y-1 ">
          <SidebarMenuItem>
            <SidebarMenuButton
              isActive={activeTab === "gamesList"}
              size="lg"
              className="rounded-xl font-medium"
              onClick={() => onTabChange("gamesList")}
            >
              <Gamepad2 className="w-5 h-5 mr-3" />
              <span>{locales.sidebar.gamesList}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
          <SidebarMenuItem>
            <SidebarMenuButton
              isActive={activeTab === "optiscalerVersions"}
              size="lg"
              className="rounded-xl"
              onClick={() => onTabChange("optiscalerVersions")}
            >
              <Box className="w-5 h-5 mr-3" />
              <span>{locales.sidebar.optiscalerVersions}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
          <SidebarMenuItem>
            <SidebarMenuButton
              isActive={activeTab === "community"}
              size="lg"
              className="rounded-xl"
              onClick={() => onTabChange("community")}
            >
              <LayoutGrid className="w-5 h-5 mr-3" />
              <span>{locales.sidebar.community}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarContent>

      <SidebarFooter className="py-6">
        <SidebarMenu className="px-4 space-y-1">
          <SidebarMenuItem>
            <SidebarMenuButton
              isActive={activeTab === "settings"}
              size="lg"
              className="rounded-xl"
              onClick={() => onTabChange("settings")}
            >
              <Settings className="w-5 h-5 mr-3" />
              <span>{locales.sidebar.settings}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
          <SidebarMenuItem>
            <SidebarMenuButton
              isActive={activeTab === "about"}
              size="lg"
              className="rounded-xl"
              onClick={() => onTabChange("about")}
            >
              <Info className="w-5 h-5 mr-3" />
              <span>{locales.sidebar.about}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
          <div className="my-2 border-t border-border/40 mx-2" />
          <SidebarMenuItem>
            <SidebarMenuButton
              size="lg"
              className="rounded-xl text-indigo-400 hover:text-indigo-300 hover:bg-indigo-500/10 transition-all duration-300"
              onClick={() => openUrl(links.discord)}
            >
              <svg
                viewBox="0 0 127.14 96.36"
                className="w-5 h-5 mr-3 fill-current"
              >
                <path d="M107.7,8.07A105.15,105.15,0,0,0,81.47,0a72.06,72.06,0,0,0-3.36,6.83A97.68,97.68,0,0,0,49,6.83,72.37,72.37,0,0,0,45.64,0,105.89,105.89,0,0,0,19.39,8.09C2.71,32.65-1.82,56.6.64,80.21h0A105.73,105.73,0,0,0,32.71,96.36,77.7,77.7,0,0,0,39.6,85.25a68.42,68.42,0,0,1-10.85-5.18c.91-.66,1.8-1.34,2.66-2a75.57,75.57,0,0,0,64.32,0c.87.71,1.76,1.39,2.66,2a68.68,68.68,0,0,1-10.87,5.19,77,77,0,0,0,6.89,11.1,105.25,105.25,0,0,0,32.19-16.14h0C130.66,50.45,122.37,26.78,107.7,8.07ZM42.45,65.69C36.18,65.69,31,60,31,53s5-12.74,11.43-12.74S54,46,53.89,53,48.84,65.69,42.45,65.69Zm42.24,0C78.41,65.69,73.25,60,73.25,53s5-12.74,11.44-12.74S96.23,46,96.12,53,91.07,65.69,84.69,65.69Z" />
              </svg>
              <span className="font-semibold">Discord</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
    </Sidebar>
  );
}
