import { getCurrentWindow } from "@tauri-apps/api/window";

// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;

export async function load() {
    // Show the window first and do initial jobs under splash screen
    await getCurrentWindow().show();

    // Inital jobs goes here
    await new Promise((resolve) => setTimeout(resolve, 2000)); // Simulate some initial jobs

    return {};
}