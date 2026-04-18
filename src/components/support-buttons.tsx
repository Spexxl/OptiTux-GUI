import { Coffee } from "lucide-react";
import { useLanguage } from "@/contexts/LanguageContext";
import links from "@/data/links.json";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";

export function BuyMeACoffeeButton() {
  const { t } = useLanguage();
  return (
    <Button
      variant="default"
      size="lg"
      className="rounded-xl font-bold gap-2 bg-[#FFDD00] hover:bg-[#E6C700] hover:scale-102 text-black border-0 shadow-lg transition-all duration-300"
      onClick={() => invoke("open_url_cmd", { url: links.buymeacoffee })}
    >
      <Coffee className="w-5 h-5 fill-current" />
      {t.community.supportBuyMeACoffee}
    </Button>
  );
}

export function PatreonButton() {
  const { t } = useLanguage();
  return (
    <Button
      variant="default"
      size="lg"
      className="rounded-xl font-bold gap-2 bg-white hover:bg-gray-300 hover:scale-102 text-black border-0 shadow-lg transition-all duration-300"
      onClick={() => invoke("open_url_cmd", { url: links.patreon })}
    >
      <svg
        viewBox="0 0 512 512"
        className="w-5 h-5 fill-current"
        xmlns="http://www.w3.org/2000/svg"
        fillRule="evenodd"
        clipRule="evenodd"
        strokeLinejoin="round"
        strokeMiterlimit="2"
      >
        <g transform="matrix(.47407 0 0 .47407 .383 .422)">
          <clipPath id="prefix__a">
            <path d="M0 0h1080v1080H0z" />
          </clipPath>
          <g clipPath="url(#prefix__a)">
            <path d="M1033.05 324.45c-.19-137.9-107.59-250.92-233.6-291.7-156.48-50.64-362.86-43.3-512.28 27.2-181.1 85.46-237.99 272.66-240.11 459.36-1.74 153.5 13.58 557.79 241.62 560.67 169.44 2.15 194.67-216.18 273.07-321.33 55.78-74.81 127.6-95.94 216.01-117.82 151.95-37.61 255.51-157.53 255.29-316.38z" fillRule="nonzero" />
          </g>
        </g>
      </svg>
      {t.community.supportPatreon}
    </Button>
  );
}
