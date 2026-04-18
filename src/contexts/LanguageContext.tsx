import React, { createContext, useContext, useState, useEffect } from "react";
import en from "@/locales/en.json";
import ptBr from "@/locales/pt-br.json";

type Translations = typeof en;

type Language = "en" | "pt-br";

interface LanguageContextType {
  language: Language;
  setLanguage: (lang: Language) => void;
  t: Translations;
}

const LanguageContext = createContext<LanguageContextType | undefined>(undefined);

export function LanguageProvider({ children }: { children: React.ReactNode }) {
  const [language, setLanguage] = useState<Language>(() => {
    return (localStorage.getItem("optitux-language") as Language) || "en";
  });

  const [t, setT] = useState<Translations>(language === "en" ? en : ptBr);

  useEffect(() => {
    localStorage.setItem("optitux-language", language);
    setT(language === "en" ? en : ptBr);
  }, [language]);

  return (
    <LanguageContext.Provider value={{ language, setLanguage, t }}>
      {children}
    </LanguageContext.Provider>
  );
}

export function useLanguage() {
  const context = useContext(LanguageContext);
  if (context === undefined) {
    throw new Error("useLanguage must be used within a LanguageProvider");
  }
  return context;
}
