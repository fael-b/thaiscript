import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { speak } from "./speech";
import { Button } from "@mantine/core";

export function LetterVariantsList() {
  const [letterVariants, setLetterVariants] = useState<any[]>([]);

  function fetchLetterVariants() {
    invoke("get_letter_variants")
      .then((response) => {
        console.log(response);
        setLetterVariants(response as any[]);
      })
      .catch((err) => {
        console.error(err);
      });
  }

  return (
    <div>
      <button onClick={fetchLetterVariants}>Fetch letter variants</button>
      {letterVariants.map((letterVariant) => {
        return (
          <Button onClick={() => speak(letterVariant.exampleWord)} title={letterVariant.romanization} key={letterVariant.id}>
            {letterVariant.letter} - {letterVariant.exampleWordEmoji}
          </Button>
        );
      })}
    </div>
  );
}
