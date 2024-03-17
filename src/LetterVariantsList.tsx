import { Group, Title, Stack } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { fetchLetterVariantsByCategory } from "./api/queries";
import { LetterVariant } from "./api/types";
import { useMemo } from "react";
import { useAtom } from "jotai";
import { LetterVariantCard } from "./LetterVariantCard";
import { currentCategoryAtom } from "./state/navigation";
import { groupLabels } from "./labels/letter-variant";

export function LetterVariantsList() {
  const [currentCategory] = useAtom(currentCategoryAtom);
  const {
    data: letterVariants,
    isLoading,
    error,
  } = useQuery({
    queryKey: ["letterVariants", currentCategory] as const,
    queryFn: ({ queryKey }) => fetchLetterVariantsByCategory(queryKey[1]),
  });

  const letterVariantsByGroup = useMemo(() => {
    const letterVariantsByGroupMap = new Map<LetterVariant["group"], LetterVariant[]>();
    return Array.from(
      (
        letterVariants?.reduce((acc, letterVariant) => {
          if (!acc.has(letterVariant.group)) {
            acc.set(letterVariant.group, []);
          }
          acc.get(letterVariant.group)!.push(letterVariant);
          return acc;
        }, letterVariantsByGroupMap) ?? letterVariantsByGroupMap
      )?.entries() ?? [],
    );
  }, [letterVariants]);

  return (
    <Stack gap="xs" mt="xs">
      {isLoading && <div>Loading...</div>}
      {error && <div>Error: {error.message}</div>}
      <section key={currentCategory}>
        {letterVariantsByGroup.map(([group, letterVariants]) => (
          <div key={group}>
            <Title order={2} px="xs" mb="xs">
              {groupLabels[group]}
            </Title>
            <Group gap="xs" px="xs" mb="md">
              {letterVariants.map((letterVariant) => {
                return <LetterVariantCard key={letterVariant.id} {...letterVariant} />;
              })}
            </Group>
          </div>
        ))}
      </section>
    </Stack>
  );
}
