import { Languages } from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";
import { Separator } from "@/components/ui/separator";

export function Settings() {
  const { language, setLanguage, t } = useLanguage();

  return (
    <div className="flex flex-col h-full bg-background/50 animate-in fade-in duration-500">
      <div className="flex-1 overflow-auto p-8 lg:p-12">
        <div className="max-w-4xl mx-auto space-y-12">

          <div className="space-y-1">
            <h2 className="text-3xl font-bold tracking-tight text-foreground/90">
              {t.settings.title}
            </h2>
            <p className="text-muted-foreground text-sm">
              {t.settings.languageDescription}
            </p>
          </div>

          <Separator className="bg-border/40" />

          <div className="grid gap-10">
            <section className="grid md:grid-cols-[280px_1fr] gap-6 items-center">
              <div className="space-y-1">
                <div className="flex items-center gap-2 text-foreground/80">
                  <Languages className="w-5 h-5 text-primary" />
                  <h3 className="font-semibold text-lg">{t.settings.language}</h3>
                </div>
              </div>

              <div className="flex gap-4">
                <button
                  onClick={() => setLanguage("en")}
                  className={`flex-1 group relative p-4 rounded-2xl border transition-all duration-300 ${language === "en"
                    ? "bg-primary/10 border-primary/40 text-primary ring-1 ring-primary/20 shadow-[0_0_20px_rgba(var(--primary),0.1)]"
                    : "bg-white/2 border-white/5 text-muted-foreground hover:bg-white/5 hover:border-white/10"
                    }`}
                >
                  <div className="flex flex-col items-center gap-1">
                    <span className={`text-base font-bold tracking-tight transition-colors ${language === "en" ? "text-primary" : "group-hover:text-foreground"}`}>
                      {t.settings.english}
                    </span>
                    <span className="text-[9px] font-black uppercase tracking-[0.2em] opacity-40">Default</span>
                  </div>
                  {language === "en" && (
                    <div className="absolute top-2 right-2 w-1.5 h-1.5 rounded-full bg-primary animate-pulse" />
                  )}
                </button>

                <button
                  onClick={() => setLanguage("pt-br")}
                  className={`flex-1 group relative p-4 rounded-2xl border transition-all duration-300 ${language === "pt-br"
                    ? "bg-primary/10 border-primary/40 text-primary ring-1 ring-primary/20 shadow-[0_0_20px_rgba(var(--primary),0.1)]"
                    : "bg-white/2 border-white/5 text-muted-foreground hover:bg-white/5 hover:border-white/10"
                    }`}
                >
                  <div className="flex flex-col items-center gap-1">
                    <span className={`text-base font-bold tracking-tight transition-colors ${language === "pt-br" ? "text-primary" : "group-hover:text-foreground"}`}>
                      {t.settings.portuguese}
                    </span>
                    <span className="text-[9px] font-black uppercase tracking-[0.2em] opacity-40">Native</span>
                  </div>
                  {language === "pt-br" && (
                    <div className="absolute top-2 right-2 w-1.5 h-1.5 rounded-full bg-primary animate-pulse" />
                  )}
                </button>
              </div>
            </section>
          </div>


        </div>
      </div>
    </div>
  );
}
