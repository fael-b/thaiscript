import { ActionIcon, AppShell, Button, Group } from "@mantine/core";
import { LetterVariantsList } from "../LetterVariantsList";
import { LineChart, Play } from "lucide-react";
import styles from "./HomeView.module.css";

export function HomeView() {
  return (
    <AppShell
      footer={{
        height: 72,
      }}
      header={{
        collapsed: true,
        height: 0,
      }}
    >
      <AppShell.Main>
        <LetterVariantsList />
      </AppShell.Main>
      <AppShell.Footer>
        <Group justify="center" style={{ height: "100%" }}>
          <ActionIcon title="Logs" radius="xl" size="xl" variant="outline">
            <LineChart />
          </ActionIcon>
          <Button
            leftSection={<Play />}
            title="Review letters"
            radius="xl"
            color="thai-gold"
            variant="gradient"
            gradient={{
              from: "yellow",
              to: "thai-gold",
            }}
            size="md"
            miw="50%"
            className={styles.review}
          >
            Review
          </Button>
        </Group>
      </AppShell.Footer>
    </AppShell>
  );
}
