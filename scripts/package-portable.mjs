import { execSync } from "child_process";
import { existsSync, readFileSync, copyFileSync, readdirSync, mkdirSync, rmSync } from "fs";
import { join, dirname } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootDir = join(__dirname, "..");

const pkg = JSON.parse(readFileSync(join(rootDir, "package.json"), "utf-8"));
const version = pkg.version;

const cargoToml = readFileSync(join(rootDir, "src-tauri", "Cargo.toml"), "utf-8");
const binNameMatch = cargoToml.match(/^name\s*=\s*"(.+)"$/m);
const binName = binNameMatch ? binNameMatch[1] : "valo-comp-analyser";

const productName = "ValoCompAnalyser";
const releaseDir = join(rootDir, "src-tauri", "target", "release");

const exeName = process.platform === "win32" ? `${binName}.exe` : binName;
const exePath = join(releaseDir, exeName);

if (!existsSync(exePath)) {
  // dev 模式下没有 release 二进制，静默跳过
  console.log("[SKIP] 未找到 release 二进制，跳过硬打包（dev 模式正常现象）。");
  process.exit(0);
}

const portableDir = join(rootDir, `${productName}_portable`);
const zipName = `${productName}_v${version}_portable.zip`;
const zipPath = join(rootDir, zipName);

// 清理旧的便携版目录和 zip
if (existsSync(portableDir)) rmSync(portableDir, { recursive: true });
if (existsSync(zipPath)) rmSync(zipPath);

// 创建便携版目录
mkdirSync(portableDir, { recursive: true });

// 复制主程序
copyFileSync(exePath, join(portableDir, productName + ".exe"));
console.log(`[OK] 复制 ${productName}.exe`);

// 复制 DLL 文件
const dllFiles = readdirSync(releaseDir).filter(
  (f) => f.endsWith(".dll") && !f.startsWith("apitrace")
);
for (const dll of dllFiles) {
  copyFileSync(join(releaseDir, dll), join(portableDir, dll));
  console.log(`[OK] 复制 ${dll}`);
}

// 复制 WebView2 loader（如果存在）
const webview2Loader = join(releaseDir, "WebView2Loader.dll");
if (existsSync(webview2Loader)) {
  copyFileSync(webview2Loader, join(portableDir, "WebView2Loader.dll"));
  console.log("[OK] 复制 WebView2Loader.dll");
}

console.log(`\n打包中: ${zipName}...`);

// PowerShell Compress-Archive
const psCmd = `Compress-Archive -Path '${portableDir}' -DestinationPath '${zipPath}' -Force`;
execSync(`powershell -NoProfile -Command "${psCmd}"`, { stdio: "inherit" });

// 清理临时目录
rmSync(portableDir, { recursive: true });

console.log(`\n[DONE] 便携版已生成: ${zipPath}`);
console.log("解压即可运行，无需安装。");
