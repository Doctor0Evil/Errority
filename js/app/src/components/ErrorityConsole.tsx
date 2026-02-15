import React, { useState } from "react";
import { ERRORITY_MANIFESTO } from "../ethics/manifesto";

type Props = {
  transform: (input: string) => Promise<string>;
};

/**
 * ErrorityConsole
 *
 * UI surface where the user can type anything – especially ideas that feel
 * pointless, broken, or confused – and watch the system attempt to make
 * something meaningful out of it.
 */
export const ErrorityConsole: React.FC<Props> = ({ transform }) => {
  const [input, setInput] = useState("");
  const [output, setOutput] = useState<string | null>(null);

  const handleRun = async () => {
    if (!input.trim()) return;
    const result = await transform(input);
    setOutput(result);
  };

  return (
    <div>
      <h1>Errority</h1>
      <p>{ERRORITY_MANIFESTO.principles[0]}</p>
      <textarea
        rows={4}
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder="Type anything you want. Errority will try to make something of it."
      />
      <button onClick={handleRun}>Transform</button>
      {output && (
        <div>
          <h2>Knowledge Candidate</h2>
          <pre>{output}</pre>
        </div>
      )}
    </div>
  );
};
