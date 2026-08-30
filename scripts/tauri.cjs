const path = require('path');

const cargoBin = 'C:\\Users\\Administrator\\.cargo\\bin';
const mingwBin = 'C:\\Users\\Administrator\\AppData\\Local\\Microsoft\\WinGet\\Packages\\MartinStorsjo.LLVM-MinGW.UCRT_Microsoft.Winget.Source_8wekyb3d8bbwe\\llvm-mingw-20260616-ucrt-x86_64\\bin';

process.env.PATH = `${cargoBin};${mingwBin};${process.env.PATH}`;

const cli = require('@tauri-apps/cli/main');
const args = process.argv.slice(2);

cli.run(args, 'npm run tauri').catch((err) => {
  if (err) {
    cli.logError(err.message || err);
  }
  process.exit(1);
});
