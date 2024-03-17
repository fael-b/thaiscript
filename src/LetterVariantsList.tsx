import { Group, Title, Stack } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import { fetchLetterVariants } from "./api/queries";
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
    queryKey: ["letterVariants"],
    queryFn: fetchLetterVariants,
  });

  const letterVariantsByCategoryAndGroup = useMemo(() => {
    const letterVariantsByCategoryAndGroupMap = new Map<LetterVariant["category"], Map<LetterVariant["group"], LetterVariant[]>>();
    return (
      letterVariants?.reduce((acc, letterVariant) => {
        if (!acc.has(letterVariant.category)) {
          acc.set(letterVariant.category, new Map());
        }
        let categoryMap = acc.get(letterVariant.category)!;
        if (!categoryMap.has(letterVariant.group)) {
          categoryMap.set(letterVariant.group, []);
        }
        categoryMap.get(letterVariant.group)!.push(letterVariant);
        return acc;
      }, letterVariantsByCategoryAndGroupMap) ?? letterVariantsByCategoryAndGroupMap
    );
  }, [letterVariants]);

  const currentCategoryLetterVariants = useMemo(() => {
    return Array.from(letterVariantsByCategoryAndGroup.get(currentCategory)?.entries() ?? []);
  }, [currentCategory, letterVariantsByCategoryAndGroup]);

  return (
    <Stack gap="xs" mt="xs">
      {isLoading && <div>Loading...</div>}
      {error && <div>Error: {error.message}</div>}
      <section key={currentCategory}>
        {currentCategoryLetterVariants.map(([group, letterVariants]) => (
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
