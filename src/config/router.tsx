import { createBrowserRouter } from "react-router-dom";
import { LetterVariantsList } from "../LetterVariantsList";

export const appRouter = createBrowserRouter([
  {
    path: "/",
    element: <LetterVariantsList />,
    errorElement: <div>404 Not Found</div>,
  },
]);
