import { Gamepad2, Box, LayoutGrid, Settings, Info } from "lucide-react";
import locales from "@/locales/en.json";

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";

export function AppSidebar() {
  return (
    <Sidebar className="border-r-0">
      <SidebarHeader className="p-6">
        <div className="flex items-center gap-3">
          <img src="/tauri.svg" alt="OptiTux Logo" className="w-6 h-6" />
          <span className="font-semibold text-lg tracking-tight">
            {locales.app.title}
          </span>
        </div>
      </SidebarHeader>

      <SidebarContent>
        <SidebarMenu className="px-4 space-y-1">
          <SidebarMenuItem>
            <SidebarMenuButton isActive size="lg" className="rounded-xl font-medium" render={<a href="#" />}>
              <Gamepad2 className="w-5 h-5 mr-3" />
              <span>{locales.sidebar.gamesList}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" className="rounded-xl text-muted-foreground hover:text-foreground" render={<a href="#" />}>
              <Box className="w-5 h-5 mr-3" />
              <span>{locales.sidebar.optiscalerVersions}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" className="rounded-xl text-muted-foreground hover:text-foreground" render={<a href="#" />}>
              <LayoutGrid className="w-5 h-5 mr-3" />
              <span>{locales.sidebar.community}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarContent>

      <SidebarFooter className="py-6">
        <SidebarMenu className="px-4 space-y-1">
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" className="rounded-xl text-muted-foreground hover:text-foreground" render={<a href="#" />}>
              <Settings className="w-5 h-5 mr-3" />
              <span>{locales.sidebar.settings}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
          <SidebarMenuItem>
            <SidebarMenuButton size="lg" className="rounded-xl text-muted-foreground hover:text-foreground" render={<a href="#" />}>
              <Info className="w-5 h-5 mr-3" />
              <span>{locales.sidebar.about}</span>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarFooter>
    </Sidebar>
  );
}
