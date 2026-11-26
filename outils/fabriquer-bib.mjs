// Prescrire BIB : même code que Prescrire (src/index.html), lancé en mode
// gestionnaire de bibliothèques. Ce script prépare le dossier bib-app/,
// utilisé par la configuration src-tauri/tauri.bib.conf.json.
import { readFileSync, writeFileSync, mkdirSync, cpSync, rmSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const racine = join(dirname(fileURLToPath(import.meta.url)), "..");
const src = join(racine, "src"), dest = join(racine, "bib-app");
const b64 = f => readFileSync(join(racine, "bib", f)).toString("base64");

let html = readFileSync(join(src, "index.html"), "utf8");
html = html.replace("<title>Prescrire</title>", "<title>Prescrire BIB</title>");
const i = html.indexOf("<body>") + "<body>".length;
if (i < 6) throw new Error("src/index.html : balise <body> introuvable");
html = html.slice(0, i) +
  `\n<script>window.PRESCRIRE_MODE="bib";window.BIB_LOGO="data:image/png;base64,${b64("logo_128.png")}";</script>` +
  `\n<link rel="icon" href="data:image/png;base64,${b64("logo_64.png")}">\n` + html.slice(i);

if (existsSync(dest)) rmSync(dest, { recursive: true });
mkdirSync(dest, { recursive: true });
writeFileSync(join(dest, "index.html"), html);
for (const d of ["vendor", "fonts"]) if (existsSync(join(src, d))) cpSync(join(src, d), join(dest, d), { recursive: true });
console.log("bib-app/ prêt");
