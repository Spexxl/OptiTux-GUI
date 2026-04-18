import { LayoutGrid } from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";
import { BuyMeACoffeeButton, PatreonButton } from "@/components/support-buttons";

export function Community() {
  const { t } = useLanguage();
  const translations = t.community;

  return (
    <div className="relative w-full h-full flex items-center justify-center overflow-hidden bg-background/50 animate-in fade-in duration-500">
      <div className="absolute inset-0 z-0 pointer-events-none opacity-[0.03] dark:opacity-[0.05]">
        <div className="absolute top-[15%] left-[10%] w-64 h-48 border-2 border-primary rounded-2xl transform -rotate-12" />
        <div className="absolute top-[45%] right-[15%] w-72 h-40 border-2 border-primary rounded-2xl transform rotate-6" />
        <div className="absolute bottom-[20%] left-[25%] w-56 h-64 border-2 border-primary rounded-2xl transform -rotate-3" />
        <div className="absolute top-[10%] right-[10%] w-48 h-48 border-2 border-primary rounded-full animate-pulse delay-300" />
        <div className="absolute bottom-[10%] right-[30%] w-40 h-40 border-2 border-primary rounded-2xl transform rotate-12" />
      </div>

      <div className="relative z-10 max-w-2xl px-8 text-center space-y-8 ">
        <div className="inline-flex p-4 rounded-3xl bg-primary/10 text-primary mb-4 ring-1 ring-primary/20">
          <LayoutGrid className="w-12 h-12" />
        </div>

        <div className="space-y-4">
          <h1 className="text-3xl font-semibold tracking-tight bg-linear-to-r from-primary to-primary/50 bg-clip-text text-transparent">
            {translations.title}
          </h1>
          <div className="inline-block px-3 py-1 rounded-full bg-primary/20 text-primary text-xs font-bold uppercase tracking-widest animate-pulse">
            {translations.subtitle}
          </div>
          <p className="text-muted-foreground text-lg leading-relaxed">
            {translations.description}
          </p>
        </div>

        <div className="p-6 rounded-2xl bg-muted/30 border border-border/50 backdrop-blur-md space-y-6 shadow-xl">
          <p className="text-sm text-muted-foreground/80 leading-relaxed">
            {translations.supportNotice}
          </p>
          <div className="flex flex-wrap items-center justify-center gap-4">
            <BuyMeACoffeeButton />
            <PatreonButton />
          </div>
        </div>
      </div>
    </div>
  );
}
