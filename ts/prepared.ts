import { prepare } from "https://deno.land/x/plugin_prepare@v0.6.0/mod.ts";
import { resolve } from "https://deno.land/std@v0.53.0/path/mod.ts";

export * from "./mod.ts";

const releaseUrl = "https://github.com/PandawanFr/deno_notifs/releases/download/0.1.0";

let DENO_NOTIFS_PLUGIN_BASE = Deno.env.get("DENO_NOTIFS_PLUGIN_BASE");
export const PLUGIN_URL_BASE = DENO_NOTIFS_PLUGIN_BASE
  ? resolvePathToURL(DENO_NOTIFS_PLUGIN_BASE)
  : releaseUrl;

let DENO_NOTIFS_PLUGIN = Deno.env.get("DENO_NOTIFS_PLUGIN");
const PLUGIN_URL = DENO_NOTIFS_PLUGIN
  ? resolvePathToURL(DENO_NOTIFS_PLUGIN)
  : undefined;
const DEBUG = Boolean(Deno.env.get("DENO_NOTIFS_DEBUG"));

/**
 * Resolves local paths to file:// URLs, leaving any other type of path as is.
 * @param path The path to resolve
 */
function resolvePathToURL(path: string) {
  if (path.startsWith("http://") || path.startsWith("https://") || path.startsWith('file://')) {
    return path;
  } else {
    let resolvedPath = resolve(path);
    return "file://" + resolvedPath;
  }
}

/**
 * Load the plugin
 */
async function load(cache = true, verbose = false): Promise<number> {
  unload();
  return await prepare({
    name: "deno_notifs",
    checkCache: cache,
    printLog: verbose,
    urls: {
      darwin: PLUGIN_URL || `${PLUGIN_URL_BASE}/libdeno_notifs.dylib`,
      windows: PLUGIN_URL || `${PLUGIN_URL_BASE}/deno_notifs.dll`,
      linux: PLUGIN_URL || `${PLUGIN_URL_BASE}/libdeno_notifs.so`,
    },
  });
}

/**
 * Free the plugin resource
 */
function unload(): void {
  if (_pluginId !== null) Deno.close(_pluginId);
  _pluginId = null;
}

let _pluginId: number | null = 0;

export const pluginId  = _pluginId;

await load(!DEBUG, DEBUG);
window.addEventListener("unload", unload);