import { createReadStream, existsSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { extname, join, normalize } from "node:path";

const root = join(process.cwd(), "dist/site");
const types = { ".css": "text/css", ".html": "text/html", ".js": "text/javascript", ".svg": "image/svg+xml", ".png": "image/png", ".webp": "image/webp", ".xml": "application/xml", ".txt": "text/plain", ".woff": "font/woff", ".woff2": "font/woff2" };
createServer((request, response) => {
  const path = decodeURIComponent(new URL(request.url, "http://localhost").pathname);
  const relative = normalize(path === "/" ? "/index.html" : path).replace(/^[/\\]+/, "");
  let file = join(root, relative);
  if (existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
  const found = existsSync(file) && statSync(file).isFile();
  if (!found) file = join(root, "404.html");
  const headers = {
    "Content-Type": types[extname(file)] ?? "application/octet-stream",
    "Content-Security-Policy": "default-src 'self'; img-src 'self'; script-src 'self'; style-src 'self'; font-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'self'; frame-ancestors 'none'",
    "Referrer-Policy": "no-referrer",
    "X-Content-Type-Options": "nosniff",
    "Strict-Transport-Security": "max-age=31536000; includeSubDomains",
    "Permissions-Policy": "camera=(), microphone=(), geolocation=()"
  };
  if (path.startsWith("/assets/") || path.endsWith(".webp")) {
    headers["Cache-Control"] = "public, max-age=31536000, immutable";
  }
  response.writeHead(found ? 200 : 404, headers);
  createReadStream(file).pipe(response);
}).listen(4173, "127.0.0.1");
