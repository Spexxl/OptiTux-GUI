import { Heart } from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";
import { BuyMeACoffeeButton, PatreonButton } from "@/components/support-buttons";
import { openUrl } from "@tauri-apps/plugin-opener";

const GitHub = ({ className }: { className?: string }) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    className={className}
  >
    <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" />
    <path d="M9 18c-4.51 2-5-2-7-2" />
  </svg>
);

export function About() {
  const { t } = useLanguage();

  const open = (url: string) => openUrl(url).catch(console.error);

  return (
    <div className="relative w-full h-full flex items-center justify-center overflow-auto bg-background/30 animate-in fade-in duration-500">
      <div className="relative z-10 flex flex-col items-center text-center gap-10 px-6 py-16 max-w-lg w-full">

        <div className="relative group">
          <div className="absolute -inset-4 bg-primary/20 rounded-full blur-2xl opacity-40 transition-all duration-1000" />
          <img
            src="/OptiTuxLogo.png"
            alt="OptiTux"
            className="relative w-28 h-28 rounded-3xl object-contain drop-shadow-2xl"
          />
        </div>

        <div className="space-y-2">
          <h1 className="text-4xl font-bold tracking-tight bg-linear-to-r from-foreground to-foreground/60 bg-clip-text text-transparent">
            OptiTux
          </h1>
          <p className="text-2xl font-semibold text-primary/80 tracking-widest">
            v{APP_VERSION}
          </p>
          <p className="text-sm text-muted-foreground leading-relaxed max-w-xs mx-auto pt-1">
            {t.about.description}
          </p>
        </div>

        <div className="flex items-center gap-3">
          <button
            onClick={() => open("https://github.com/Spexxl/OptiTux-GUI")}
            className="flex items-center gap-2 px-4 py-2 rounded-xl bg-white/5 hover:bg-white/10 border border-white/8 hover:border-white/15 text-sm font-medium text-muted-foreground hover:text-foreground transition-all duration-200"
          >
            <GitHub className="w-4 h-4" />
            {t.about.visitGithub}
          </button>
        </div>

        <div className="w-full pt-2 border-t border-white/5 space-y-4">
          <div className="flex items-center justify-center gap-1.5 text-muted-foreground/50">
            <Heart className="w-3.5 h-3.5" />
            <span className="text-xs font-medium uppercase tracking-widest">{t.about.supportTitle}</span>
          </div>
          <div className="flex flex-wrap items-center justify-center gap-3">
            <PatreonButton />
            <BuyMeACoffeeButton />
          </div>
        </div>

      </div>
    </div>
  );
}
