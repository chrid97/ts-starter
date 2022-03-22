#!/usr/bin/env node

import readline from "readline";
import fs from "fs";
import { execSync } from "child_process";

const ignoreContent = `
  node_modules
  dist
`;

const prettierContent = `
  {
    "semi": true,
    "singleQuote": true,
    "trailingComma": "all",
    "arrowParens": "always",
    "tabWidth": 4
  }
`;

function main(): void {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  rl.question("Project Name: ", (projectName) => {
    const tsconfigContent = fs.readFileSync(
      `./Node-TypeScript-Starter/src/templates/tsconfig.json`,
      "utf8"
    );
    const eslintConfigContent = fs.readFileSync(
      `./Node-TypeScript-Starter/src/templates/.eslintrc.json`,
      "utf8"
    );
    const newDir = `${process.cwd()}/${projectName}`;
    fs.mkdirSync(newDir);
    process.chdir(newDir);
    execSync("git init");
    fs.mkdirSync(`${process.cwd()}/src`);
    writeFile("/src/index.ts");
    writeFile("README.md", `# ${projectName}`);
    writeFile(".eslintignore", ignoreContent);
    writeFile(".eslintrc.json", eslintConfigContent);
    writeFile(".prettierrc", prettierContent);
    writeFile(".prettierignore", ignoreContent);
    writeFile(".gitignore", ignoreContent);
    writeFile("tsconfig.json", tsconfigContent);
    installPackages();
    updatePackageJson();
    console.log(`Project Created!`);
    process.exit(0);
  });

  rl.on("close", () => {
    console.log(`\nProcess cancelled`);
    process.exit(0);
  });
}

function writeFile(file: string, data = ""): void {
  fs.writeFileSync(`${process.cwd()}/${file}`, data);
}

main();

function installPackages(): void {
  console.log("npm init");
  execSync("npm init -y");
  console.log("npm install (may take a while)");
  execSync(
    "npm install --save-dev typescript tsc-watch @types/node prettier eslint"
  );
}

function updatePackageJson(): void {
  const path = `${process.cwd()}/package.json`;
  const content = JSON.parse(fs.readFileSync(path, "utf8"));
  const newFile = {
    ...content,
    main: "dist/index.js",
    type: "module",
    scripts: {
      lint: "eslint . --ext .ts",
      format: "prettier 'src/**/*.ts' --write",
      start: "node dist/index.js",
      dev: "tsc-watch --onSuccess 'npm run start'",
      test: 'echo "Error: no test specified" && exit 1',
    },
  };
  const newContent = JSON.stringify(newFile);
  fs.writeFileSync(path, newContent);
}
