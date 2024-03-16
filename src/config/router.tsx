import { createBrowserRouter } from "react-router-dom";
import { HomeView } from "../views/HomeView";

export const appRouter = createBrowserRouter([
  {
    path: "/",
    element: <HomeView />,
    errorElement: <div>404 Not Found</div>,
  },
]);
