import { createTheme, MantineColorsTuple } from "@mantine/core";

const thaiPurple: MantineColorsTuple = ["#f2f2f8", "#e2e2e9", "#c2c1d4", "#a09dbf", "#837fad", "#716da2", "#68639e", "#57538b", "#4d4a7d", "#423f6f"];
const thaiRed: MantineColorsTuple = ["#ffebf0", "#f9d6dc", "#f0aab5", "#e87b8d", "#e0546b", "#dd3c55", "#dc2f4a", "#c3223c", "#af1b34", "#9a0e2b"];
const thaiGold: MantineColorsTuple = ["#fffbe1", "#fff5cc", "#ffeb9b", "#ffe064", "#ffd738", "#ffd11c", "#ffce09", "#e3b500", "#c9a100", "#ae8b00"];

export const thaiScriptTheme = createTheme({
  primaryColor: "thai-purple",
  scale: 1.25,
  colors: {
    "thai-purple": thaiPurple,
    "thai-red": thaiRed,
    "thai-gold": thaiGold,
  },
});
