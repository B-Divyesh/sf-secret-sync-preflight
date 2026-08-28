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
  response.writeHead(found ? 200 : 404, { "Content-Type": types[extname(file)] ?? "application/octet-stream", "X-Content-Type-Options": "nosniff", "Referrer-Policy": "no-referrer" });
  createReadStream(file).pipe(response);
}).listen(4173, "127.0.0.1");
