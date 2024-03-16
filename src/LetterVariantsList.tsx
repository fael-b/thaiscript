import { speak } from "./speech";
import { Button } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { fetchLetterVariants } from "./data/queries";

export function LetterVariantsList() {
  const {
    data: letterVariants,
    isLoading,
    error,
  } = useQuery({
    queryKey: ["letterVariants"],
    queryFn: fetchLetterVariants,
  });

  return (
    <div>
      {isLoading && <div>Loading...</div>}
      {error && <div>Error: {error.message}</div>}
      {letterVariants?.map((letterVariant) => {
        return (
          <Button onClick={() => speak(letterVariant.exampleWord)} title={letterVariant.romanization} key={letterVariant.id}>
            {letterVariant.letter} - {letterVariant.exampleWordEmoji}
          </Button>
        );
      })}
    </div>
  );
}
