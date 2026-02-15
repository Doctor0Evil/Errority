#!/usr/bin/env node

/**
 * Errority Tool (Node CLI)
 *
 * Reads text from stdin or arguments, sends it through the same philosophy:
 * - every input has the right to exist,
 * - disagreements are encouraged,
 * - knowledge is earned, not assumed.
 */

import { spawn } from "child_process";

function run() {
  const args = process.argv.slice(2);
  const text = args.join(" ");

  if (!text) {
    console.error("Usage: errority-tool \"your input text\"");
    process.exit(1);
  }

  const cli = spawn("errority-cli", [text], { stdio: "inherit" });

  cli.on("exit", (code) => {
    process.exit(code ?? 0);
  });
}

run();
