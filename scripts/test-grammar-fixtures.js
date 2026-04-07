#!/usr/bin/env node

const childProcess = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");

const repoRoot = path.resolve(__dirname, "..");
const extensionTomlPath = path.join(repoRoot, "extension.toml");
const fixtureRoot = path.join(repoRoot, "fixtures", "grammar");
const treeSitterBinary = path.join(
  repoRoot,
  "node_modules",
  ".bin",
  process.platform === "win32" ? "tree-sitter.cmd" : "tree-sitter"
);

const grammars = [
  {
    extension: ".bicep",
    fixtureDir: path.join(fixtureRoot, "bicep"),
    name: "bicep",
    section: "grammars.bicep",
  },
  {
    extension: ".bicepparam",
    fixtureDir: path.join(fixtureRoot, "bicep_params"),
    name: "bicep_params",
    section: "grammars.bicep_params",
  },
];

function run(command, args, options = {}) {
  return childProcess.execFileSync(command, args, {
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
    ...options,
  });
}

function sectionValue(contents, section, key) {
  const lines = contents.split(/\r?\n/);
  const sectionLines = [];
  let inSection = false;

  for (const line of lines) {
    const sectionMatch = line.match(/^\[(.+)\]$/);

    if (sectionMatch) {
      if (inSection) {
        break;
      }

      inSection = sectionMatch[1] === section;
      continue;
    }

    if (inSection) {
      sectionLines.push(line);
    }
  }

  if (!inSection && sectionLines.length === 0) {
    throw new Error(`Missing [${section}] section in extension.toml`);
  }

  const sectionContents = sectionLines.join("\n");
  const keyPattern = new RegExp(String.raw`^${key} = "([^"]+)"$`, "m");
  const valueMatch = sectionContents.match(keyPattern);

  if (!valueMatch) {
    throw new Error(`Missing ${key} in [${section}] section`);
  }

  return valueMatch[1];
}

function listFixtures(dir, extension) {
  return fs
    .readdirSync(dir, { withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name.endsWith(extension))
    .map((entry) => path.join(dir, entry.name))
    .sort();
}

function copyFixtures(files, destinationDir) {
  fs.mkdirSync(destinationDir, { recursive: true });

  return files.map((file) => {
    const target = path.join(destinationDir, path.basename(file));
    fs.copyFileSync(file, target);
    return target;
  });
}

function assertNoErrors(output, grammarName) {
  if (/\b(ERROR|MISSING)\b/.test(output)) {
    throw new Error(`${grammarName} fixtures produced parse errors:\n${output}`);
  }
}

function main() {
  if (!fs.existsSync(treeSitterBinary)) {
    throw new Error(
      "tree-sitter CLI is not installed. Run `npm install` before testing grammar fixtures."
    );
  }

  const extensionToml = fs.readFileSync(extensionTomlPath, "utf8");
  const tempRoot = fs.mkdtempSync(path.join(os.tmpdir(), "zed-bicep-grammar-"));

  try {
    for (const grammar of grammars) {
      const repository = sectionValue(extensionToml, grammar.section, "repository");
      const commit = sectionValue(extensionToml, grammar.section, "commit");
      const fixtures = listFixtures(grammar.fixtureDir, grammar.extension);

      if (fixtures.length === 0) {
        throw new Error(`No fixtures found for ${grammar.name} in ${grammar.fixtureDir}`);
      }

      const checkoutDir = path.join(tempRoot, grammar.name);
      run("git", ["clone", "--quiet", repository, checkoutDir]);
      run("git", ["-C", checkoutDir, "checkout", "--quiet", commit]);

      const copiedFixtures = copyFixtures(fixtures, path.join(checkoutDir, "zed-fixtures"));
      const parseOutput = run(
        treeSitterBinary,
        ["parse", ...copiedFixtures.map((file) => path.relative(checkoutDir, file))],
        { cwd: checkoutDir }
      );

      assertNoErrors(parseOutput, grammar.name);
      console.log(
        `Parsed ${copiedFixtures.length} ${grammar.name} fixture(s) with ${repository}@${commit}.`
      );
    }
  } finally {
    fs.rmSync(tempRoot, { force: true, recursive: true });
  }
}

main();
