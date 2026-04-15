import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-center p-8 bg-background text-foreground transition-colors duration-300">
      <div className="max-w-md w-full space-y-8 text-center">
        <div className="flex justify-center space-x-6">
          <img src="/vite.svg" className="h-20 w-20 animate-bounce" alt="Vite logo" />
          <img src="/tauri.svg" className="h-20 w-20 animate-pulse" alt="Tauri logo" />
        </div>

        <h1 className="text-4xl font-extrabold tracking-tight lg:text-5xl">
          OptiTux GUI
        </h1>

        <p className="text-muted-foreground">
          Manage your Optiscaler installations with ease.
        </p>

        <form
          className="flex space-x-2"
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <input
            id="greet-input"
            className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter your name..."
          />
          <Button type="submit">Greet</Button>
        </form>

        {greetMsg && (
          <div className="p-4 rounded-lg bg-secondary text-secondary-foreground animate-in fade-in slide-in-from-bottom-4 duration-500">
            {greetMsg}
          </div>
        )}
      </div>
    </main>
  );
}

export default App;
